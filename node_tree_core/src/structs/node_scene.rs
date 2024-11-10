//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                  /$$$$$$                                         
// | $$$ | $$                | $$                 /$$__  $$                                        
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$       | $$  \__/  /$$$$$$$  /$$$$$$  /$$$$$$$   /$$$$$$ 
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$      |  $$$$$$  /$$_____/ /$$__  $$| $$__  $$ /$$__  $$
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$       \____  $$| $$      | $$$$$$$$| $$  \ $$| $$$$$$$$
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/       /$$  \ $$| $$      | $$_____/| $$  | $$| $$_____/
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$      |  $$$$$$/|  $$$$$$$|  $$$$$$$| $$  | $$|  $$$$$$$
// |__/  \__/ \______/  \_______/ \_______/       \______/  \_______/ \_______/|__/  |__/ \_______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! This provides the `NodeScene` type, which allows for the saving and loading of scenes, as well
//! as the easy initialization of them via the `scene!` macro!
//! 

use std::io::{ Read, Write };
use std::path::Path;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use std::hash::{ self, Hash, Hasher };

use toml_edit as toml;

use crate::structs::rid::RID;
use crate::traits::{ node::Node, instanceable::Instanceable };
use crate::services::node_registry::{ self, FieldMap, SFieldMap };


/*
 * Node Scene
 *      Struct
 */


/// A recursive structure that allows for the storage, saving, and loading of a dormant scene of nodes.
/// The root node is what every node in the scene will have its owner set to.
#[derive(Debug)]
pub struct NodeScene {
    this:      *mut dyn Node,
    children:  Vec<NodeScene>,
    from_disk: bool,
    
    pub is_owner: bool
}

impl NodeScene {
    
    /// Creates a new `NodeScene` with a root node.
    pub fn new<N: Node>(root: N) -> Self {
        Self::new_dyn(root.to_dyn_box())
    }

    /// Creates a new `NodeScene` from a dynamic node.
    pub fn new_dyn(root: Box<dyn Node>) -> Self {
        NodeScene {
            this:      Box::into_raw(root),
            children:  Vec::new(),
            from_disk: false,
            is_owner:  true
        }
    }

    /// Loads a `NodeScene` from a `.scn` file.
    pub fn load(path: &Path) -> Result<Self, String> {
        
        // Ensure that the file described is a scene file.
        match path.extension().map(|ext| ext.to_str()).flatten() {
            Some("scn") => (),
            Some(_)     => return Err("Attempted to load a file with an extension differing from .scn".to_string()),
            None        => return Err("Path did not contain a valid file extension".to_string())
        }
        
        // Attempt to load the file and write its contents to a buffer.
        let mut file:   fs::File = fs::File::open(path).map_err(|err| format!("{err}"))?;
        let mut buffer: Vec<u8>  = Vec::new();
        
        file.read_to_end(&mut buffer).map_err(|err| format!("{err}"))?;
        drop(file);
        
        // Attempt to parse the file as a table.
        let document: String            = String::from_utf8(buffer).map_err(|err| format!("{err}"))?;
        let document: toml::DocumentMut = document.parse().map_err(|err| format!("{err}"))?;
        
        // Go through each node and deserialize it:
        let mut node_scene: Option<NodeScene>        = None;
        let mut traversal:  HashMap<RID, Vec<usize>> = HashMap::new(); // Cache used for quick traversal.

        for (key, node_data) in document.iter() {

            // Deserialize the node's metadata.
            let node_data: &toml::Table       = node_data.as_table().ok_or(format!("Failed to parse {}'s data", key))?;
            let metadata:  &toml::InlineTable = node_data.get("metadata").map(|nd| nd.as_inline_table()).flatten().ok_or(format!("Failed to parse {}'s metadata", key))?;
            let type_name: String             = metadata.get("type_name").map(|tn| tn.as_str().map(|s| s.to_string())).flatten().ok_or(format!("Failed to parse {}'s type name", key))?;
            let is_owner:  bool               = metadata.get("is_owner").map(|tn| tn.as_bool()).flatten().ok_or(format!("Failed to parse {}'s ownership status", key))?;
            let parent:    Option<RID>        = metadata.get("parent").map(|p| p.as_integer().map(|rid| rid as RID)).flatten();

            // Deserialize the node data back into its respective type.
            let node_fields: Option<SFieldMap> = node_data.into_iter()
                .filter(|(field, _)| *field != "metadata")
                .map(|(field, value)| {
                    match value {
                        toml::Item::Value(value) => Some((field.into(), value.to_owned())),
                        _                        => None
                    }
                }).collect();

            let node:      Box<dyn Node> = node_registry::deserialize(&type_name, node_fields.ok_or("Could not parse node fields".to_string())?)?;
            let local_rid: RID           = key.split_once('_')
                .ok_or("Failed to parse Node key".to_string())?
                .1.parse()
                .map_err(|err| format!("{err}"))?;
            
            // Append the node to the scene.
            match node_scene.as_mut() {
                Some(node_scene) => {
                    
                    // These nodes should have parents; check it and is it to determine the node's
                    // placement.
                    let parent_rid: RID = parent.ok_or("No parent registered for a non-root node".to_string())?;
                    if parent_rid == 0 {

                        // Save the node as a child of the root node and cache its traversal
                        // coordinates.
                        let mut new_scene: NodeScene = NodeScene::new_dyn(node);
                                new_scene.from_disk  = true;

                        if is_owner {
                            node_scene.append_as_owner(new_scene);
                        } else {
                            node_scene.append(new_scene);
                        }
                        traversal.insert(local_rid, vec![node_scene.children.len() - 1]);
                        continue;
                    }

                    // Otherwise, if the parent RID is other than zero, then we have to traverse to
                    // the parent's position in the node_scene and append it from there.
                    match traversal.get(&parent_rid) {
                        Some(cached_path) => {
                            
                            // Funny pointer traversal
                            let mut cursor: Option<*mut NodeScene> = None;
                            for &segment in cached_path {
                                match cursor {
                                    Some(_) => cursor = cursor.map(|scene_ptr| &mut (unsafe { &mut *scene_ptr }.children[segment]) as *mut _),
                                    None    => cursor = Some(&mut node_scene.children[segment])
                                }
                            }

                            // This should always be `Some(_)`:
                            let     found_parent: &mut NodeScene = unsafe { &mut *cursor.expect("Could not find a parent - internal bug") };
                            let mut new_scene:    NodeScene      = NodeScene::new_dyn(node);
                                    new_scene.from_disk          = true;

                            if is_owner {
                                found_parent.append_as_owner(new_scene);
                            } else {
                                found_parent.append(new_scene);
                            }
                        },
                        None => return Err("Child was declared ahead of parent in the .scn file".to_string())
                    }
                },
                None => node_scene = Some(NodeScene::new_dyn(node))
            }
        }

        node_scene.ok_or("No root node found in scene".to_string())
    }
    
    /// Saves a `NodeScene` to a `toml` like `.scn` file.
    pub fn save(&self, path: &Path, name: &str) -> Result<(), String> {

        // Constuct a buffer for the toml format.
        let mut document: toml::DocumentMut = toml::DocumentMut::new();

        // Go through each node and serialize it:
        self.update_internal(0);
        self.clone().iterate(|parent, node, is_owner| {
            let node:   &dyn Node         = unsafe { &*node };
            let parent: Option<&dyn Node> = parent.map(|x| unsafe { &*x });
            
            // Format the metadata.
            let node_key: String = format!("Node_{}", node.rid());
            
            document[&node_key]                          = toml::Item::Table(toml::Table::new());
            document[&node_key]["metadata"]              = toml::InlineTable::new().into();
            document[&node_key]["metadata"]["type_name"] = node.name_as_type().into();
            document[&node_key]["metadata"]["is_owner"]  = is_owner.into();

            if let Some(parent_rid) = parent.map(|p| p.rid()) {
                document[&node_key]["metadata"]["parent"] = (parent_rid as i64).into();
            }

            // Save the fields.
            let node_fields: FieldMap = node.save_from_owned();
            for (field_name, value) in node_fields {
                document[&node_key][&field_name.to_string()] = toml::Item::Value(value.to_value());
            }
        });
        
        // Write the saved scene data to disk.
        let mut full_name: PathBuf = path.to_owned();
                full_name.push(Path::new(&format!("{name}.scn")));

        let mut buffer: String = "# This scene file was generated automatically via node_tree.\n\n".to_string();
                buffer        += &document.to_string();

        let mut file: fs::File = fs::File::create(full_name).map_err(|err| format!("{err}"))?;
                file.write_all(buffer.as_bytes()).map_err(|err| format!("{err}"))?;
        Ok(())
    }

    /// Recursively builds a hash that represents the scene layout.
    /// This will NOT check node fields, but will only compare the shape, ownership, and types
    /// present throughout a scene tree.
    ///
    /// This can be useful to test for scene changes across loading and saving.
    /// 
    /// # Note
    /// This uses the default hasher. The `Hash` trait is also implemented to support hashing with
    /// other hash functions.
    pub fn structural_hash(&self) -> u64 {
        let mut hasher: hash::DefaultHasher = hash::DefaultHasher::new();
        self.internal_structural_hash(&mut hasher);
        hasher.finish()
    }

    // Internal hash function.
    fn internal_structural_hash<H: hash::Hasher>(&self, state: &mut H) {
        
        // Hash the type name, current ownership value, and the number of children.
        unsafe { &*self.this }.name_as_type().hash(state);
        self.is_owner.hash(state);
        self.children.len().hash(state);

        // Recursively hash all of the children.
        for child in &self.children {
            child.hash(state);
        }
    }

    /// Appends a `NodeScene` as a child.
    pub fn append(&mut self, mut child: NodeScene) {
        child.is_owner = false; // Have this only be applied for single nodes, not whole node scenes!
        self.children.push(child);
    }

    /// Appends an owning `NodeScene` as a child, ensuring that the root node of the added
    /// `NodeScene` is always an owner.
    pub fn append_as_owner(&mut self, mut child: NodeScene) {
        child.is_owner = true;
        self.children.push(child);
    }

    /// Returns this `NodeScene` instance's associated node.
    /// 
    /// # Safety
    /// This is marked unsafe as if the resulting `Box<T>` is dropped, the internal pointer could
    /// be invalidated.
    pub unsafe fn get_node(&self) -> Box<dyn Node> {
        Box::from_raw(self.this)
    }

    /// Gets the children.
    pub fn children(&self) -> &[NodeScene] {
        &self.children
    }

    /// Updates the internal RIDs.
    pub fn update_internal(&self, mut counter: u64) {
        for child in &self.children {

            // Update the counter and set it as this child's rid
            counter += 1;
            unsafe { (&mut *child.this).set_rid(counter) };

            // Recursively traverse the child's children
            child.update_internal(counter);
        }
    }
}

impl Clone for NodeScene {
    fn clone(&self) -> Self {
        let cloned_node = unsafe {
            let     node_original: Box<dyn Node> = Box::from_raw(self.this);
            let mut node_new:      Box<dyn Node> = node_original.clone_as_instance();
            
            node_new.set_rid(node_original.rid());

            Box::into_raw(node_original); // Convert the box back so that its instance isn't deallocated when dropped.
            Box::into_raw(node_new)
        };

        // Recursively clone children
        let cloned_children: Vec<NodeScene> = self.children.to_vec();
        NodeScene {
            this:      cloned_node,
            children:  cloned_children,
            from_disk: self.from_disk,
            is_owner:  self.is_owner,
        }
    }
}

impl hash::Hash for NodeScene {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.internal_structural_hash(state)
    }
}

impl Instanceable for NodeScene {
    fn iterate<F: FnMut(Option<*mut dyn Node>, *mut dyn Node, bool)>(self, mut iterator: F) {
        iterator(None, self.this, self.is_owner);

        // Recursive function to traverse the tree
        fn traverse<F: FnMut(Option<*mut dyn Node>, *mut dyn Node, bool)>(
            node:     NodeScene,
            parent:   *mut dyn Node,
            iterator: &mut F
        ) {
            for child in node.children {

                // Call the iterator for the child node
                if child.from_disk {
                    let child_mut: &mut dyn Node = unsafe { &mut *child.this };
                    unsafe {
                        child_mut.mark_as_loaded();
                    }
                }
                iterator(Some(parent), child.this, child.is_owner);

                // Recursively traverse the child's children
                let child_this: *mut dyn Node = child.this;
                traverse(child, child_this, iterator);
            }
        }

        // Start the traversal from the root.
        let self_this: *mut dyn Node = self.this;
        if self.from_disk {
            let self_mut:  &mut dyn Node = unsafe { &mut *self_this };
            unsafe {
                self_mut.mark_as_loaded();
            }
        }
        traverse(self, self_this, &mut iterator);
    }
}
