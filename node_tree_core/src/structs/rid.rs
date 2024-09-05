//===================================================================================================================================================================================//
//
//  /$$$$$$$  /$$$$$$ /$$$$$$$ 
// | $$__  $$|_  $$_/| $$__  $$
// | $$  \ $$  | $$  | $$  \ $$
// | $$$$$$$/  | $$  | $$  | $$
// | $$__  $$  | $$  | $$  | $$
// | $$  \ $$  | $$  | $$  | $$
// | $$  | $$ /$$$$$$| $$$$$$$/
// |__/  |__/|______/|_______/ 
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! A system that allows for the efficient storage of procedurally tagged items.
//! 

use std::collections::{ hash_map::{ Values, ValuesMut }, HashMap };


/// Describes an RID type.
pub type RID = u64;


/// Holds a hashmap with automatically managed keys or RIDs (reference IDs).
#[derive(Debug, Clone)]
pub struct RIDHolder<T> {
    data:    HashMap<RID, T>,
    highest: RID,
    vacant:  Vec<RID>
}

impl <T> RIDHolder<T> {
    
    /// Creates an empty RID holder structure.
    pub fn new() -> Self {
        RIDHolder {
            data:    HashMap::new(),
            highest: 0,
            vacant:  Vec::new()
        }
    }

    /// Creates a new RID holder from a vector, where each index of each item is its RID.
    pub fn from_vec(slice: Vec<T>) -> Self {
        let highest: RID = (slice.len() - 1) as RID;
        RIDHolder {
            data:    slice.into_iter().enumerate().map(|(rid, item)| (rid as RID, item)).collect(),
            highest,
            vacant:  Vec::new()
        }
    }

    /// Adds a new item to the holder, registering it under the returned ID.
    pub fn push(&mut self, item: T) -> RID {
        let rid: RID = match self.vacant.pop() {
            Some(id) => id,
            None     => {
                let id: RID = self.highest;
                self.highest += 1;
                id
            }
        };

        self.data.insert(rid, item);
        rid
    }

    /// Retrieves an item's reference via an RID.
    pub fn retrieve(&self, rid: RID) -> Option<&T> {
        self.data.get(&rid)
    }
    
    /// Retrieves an item's mutable reference via an RID.
    pub fn modify(&mut self, rid: RID) -> Option<&mut T> {
        self.data.get_mut(&rid)
    }
    
    /// Removes an item from the collection by the passed RID.
    /// Returns the item.
    pub fn take(&mut self, rid: RID) -> Option<T> {
        match self.data.remove(&rid) {
            None       => None,
            Some(item) => {
                self.vacant.push(rid);
                Some(item)
            }
        }
    }

    /// Returns an iter for each of the items.
    pub fn iter(&self) -> Values<RID, T> {
        self.data.values()
    }
    
    /// Returns a mutable iter for each of the items.
    pub fn iter_mut(&mut self) -> ValuesMut<RID, T> {
        self.data.values_mut()
    }
}
