//===================================================================================================================================================================================//
//
//  /$$   /$$   /$$     /$$ /$$ /$$   /$$                     /$$$$$$$$                 
// | $$  | $$  | $$    |__/| $$|__/  | $$                    | $$_____/                 
// | $$  | $$ /$$$$$$   /$$| $$ /$$ /$$$$$$   /$$   /$$      | $$    /$$$$$$$   /$$$$$$$
// | $$  | $$|_  $$_/  | $$| $$| $$|_  $$_/  | $$  | $$      | $$$$$| $$__  $$ /$$_____/
// | $$  | $$  | $$    | $$| $$| $$  | $$    | $$  | $$      | $$__/| $$  \ $$|  $$$$$$ 
// | $$  | $$  | $$ /$$| $$| $$| $$  | $$ /$$| $$  | $$      | $$   | $$  | $$ \____  $$
// |  $$$$$$/  |  $$$$/| $$| $$| $$  |  $$$$/|  $$$$$$$      | $$   | $$  | $$ /$$$$$$$/
//  \______/    \___/  |__/|__/|__/   \___/   \____  $$      |__/   |__/  |__/|_______/ 
//                                            /$$  | $$                                 
//                                           |  $$$$$$/                                 
//                                            \______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Contains utility functions used throughout the crate.
//!

use crate::{ prelude::{ RID, NodeTreeBase, Node }, structs::node_base::NodeStatus };


/// Ensures that the name provided is unique relative to the list of other names.
/// If it is not, then it will create a new unique name.
pub fn ensure_unique_name(name: &str, relative_to: &[String]) -> String {
    fn extract_numerical_suffix(s: &str) -> Option<usize> {
        let mut numerics: String = String::new();
        let mut ptr:      usize  = s.len() - 1;
        
        loop {
            let char: char = s.get(ptr..(ptr + 1)).unwrap().chars().collect::<Vec<_>>()[0];
            if !char.is_numeric() {
                break;
            }
            numerics = char.to_string() + &numerics;
            
            if ptr == 0 {
                break;
            }
            ptr -= 1;
        }

        if numerics.is_empty() {
            return None;
        }
        Some(numerics.parse::<usize>().unwrap())
    }

    // Special Case:
    // If the 'relative_to' array is empty, then return the name.
    if relative_to.is_empty() {
        return name.to_string();
    }

    // Strip the name bare of any numerical suffix.
    let given_value:         Option<usize> = extract_numerical_suffix(name);
    let name_without_suffix: String        = match given_value {
        Some(number) => name.split_at(name.find(&format!("{}", number)).unwrap()).0.to_string(),
        None         => name.to_string()
    };
    
    // Search for any similar names that have the same beginning but different suffixes.
    let mut similar_names: Vec<String> = Vec::new();
    for set_name in relative_to {
        let idx_found: Option<usize> = set_name.find(&name_without_suffix);
        
        if let Some(idx) = idx_found {
            if idx != 0 {   // We do not include similar names when the pattern does not start at the beginning of the string.
                continue;
            }
            similar_names.push(set_name.to_string());
        }
    }

    if similar_names.len() == 0 {
        return name.to_string();
    }

    // Order all of the names with a numerical suffix.
    // If this name does not have a numerical suffix, then give it the lowest possible numerical
    // suffix.
    // Otherwise, give it the closest numerical suffix to the one it currently has (counting
    // upwards).
    let mut new_value: usize      = given_value.unwrap_or(0);
    let     values:    Vec<usize> = similar_names.iter().map(|n| extract_numerical_suffix(n).unwrap_or(0)).collect(); // If there are no numerical suffixes on similar names,
    loop {
        for value in values {
            if new_value == value {
                new_value += 1;
                continue;
            }
        }
        break;
    }
    
    let new_suffix: String = format!("{}", new_value);
    name_without_suffix.to_string() + &new_suffix
}

/// Takes in a NodeTree and prints out a graphical representation with a node as the origin.
/// `view_up` is the amount of layers of nodes above the origin that are drawn (parent, grandparent, etc), and `view_down` is the amount
/// of layers of nodes below the origin that are drawn (children, etc).
pub fn draw_tree(node_tree: &NodeTreeBase, origin: RID, view_up: usize, view_down: usize) -> String {
    fn get_start<'a>(tree: &'a NodeTreeBase, node: &'a dyn Node, view_left: usize) -> &'a dyn Node {
        if node.is_root() || view_left == 0 {
            return node;
        }
        get_start(tree, unsafe { tree.get_node(node.parent_dyn().unwrap_unchecked().rid()).unwrap_unchecked() }, view_left - 1)
    }

    let origin:    &dyn Node = node_tree.get_node(origin).unwrap();
    let draw_from: &dyn Node = get_start(node_tree, origin, view_up);
    let levels:    usize     = view_up + view_down;
    
    const OTHER_CHILD: &str = "│   ";   // prefix: pipe
    const OTHER_ENTRY: &str = "├── ";   // connector: tee
    const FINAL_CHILD: &str = "    ";   // prefix: no more siblings
    const FINAL_ENTRY: &str = "└── ";   // connector: elbow
    
    let mut warnings: Vec<String> = Vec::new();
    let mut panics:   Vec<String> = Vec::new();

    fn walk(tree: &NodeTreeBase, node_rid: RID, prefix: &str, out: &mut String, warnings: &mut Vec<String>, panics: &mut Vec<String>, level: usize) -> () {
        let     node:  &dyn Node = unsafe { tree.get_node(node_rid).unwrap_unchecked() };
        let mut count: usize     = node.num_children();

        for child in node.children() {
            count -= 1;
            let     connector:  &str   = if count == 0 { FINAL_ENTRY } else { OTHER_ENTRY };
            let mut child_name: String = child.name().to_string();

            match child.status() {
                NodeStatus::Normal => (),

                NodeStatus::JustWarned(warn) => {
                    child_name = format!("\u{001b}[33m{}\u{001b}[0m", child_name);
                    warnings.push(format!("{} - {}", child.name(), warn));
                },
                
                NodeStatus::JustPanicked(panic) => {
                    child_name = format!("\u{001b}[31m{}\u{001b}[0m", child_name);
                    panics.push(format!("{} - {}", child.name(), panic));
                }
            }
            
            *out += &format!("{}{}{}\n", prefix, connector, if level != 0 { child_name } else { "...".to_string() });
            if !child.childless() && level != 0 {
                let new_prefix: String = format!("{}{}", prefix, if count == 0 { FINAL_CHILD } else { OTHER_CHILD });
                walk(tree, child.rid(), &new_prefix, out, warnings, panics, level - 1);
            }
        }
    }

    let mut out: String = format!("[REPORT START]\n{}\n", draw_from.name());
    walk(node_tree, draw_from.rid(), "", &mut out, &mut warnings, &mut panics, levels + 1);   // + 1 to compensate for the last names being replaced with "..."
   
    out += "\n[Same-Frame Warnings]";
    if !warnings.is_empty() {
        for warning in warnings {
            out += &format!("\n\u{001b}[33m{}\u{001b}[0m", warning);
        }
    } else {
        out += "\nNone";
    }
    
    out += "\n\n[Same-Frame Panics]";
    if !panics.is_empty() {
        for panic in panics {
            out += &format!("\n\u{001b}[31m{}\u{001b}[0m", panic);
        }
    } else {
        out += "\nNone";
    }
    
    out += "\n\n[REPORT END]";
    out
}
