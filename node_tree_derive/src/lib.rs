//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$            /$$$$$$                            /$$$$$$$                      /$$                    
// | $$$ | $$                | $$           /$$__  $$                          | $$__  $$                    |__/                    
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$ | $$  \__/ /$$   /$$  /$$$$$$$      | $$  \ $$  /$$$$$$   /$$$$$$  /$$ /$$    /$$ /$$$$$$ 
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$|  $$$$$$ | $$  | $$ /$$_____/      | $$  | $$ /$$__  $$ /$$__  $$| $$|  $$  /$$//$$__  $$
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$ \____  $$| $$  | $$|  $$$$$$       | $$  | $$| $$$$$$$$| $$  \__/| $$ \  $$/$$/| $$$$$$$$
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/ /$$  \ $$| $$  | $$ \____  $$      | $$  | $$| $$_____/| $$      | $$  \  $$$/ | $$_____/
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$|  $$$$$$/|  $$$$$$$ /$$$$$$$/      | $$$$$$$/|  $$$$$$$| $$      | $$   \  $/  |  $$$$$$$
// |__/  \__/ \______/  \_______/ \_______/ \______/  \____  $$|_______/       |_______/  \_______/|__/      |__/    \_/    \_______/
//                                                    /$$  | $$                                                                      
//                                                   |  $$$$$$/                                                                      
//                                                    \______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Contains the `Abstract` derive macro which helps reduce boilerplate in regards to trait
//! implementations.
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput, Ident };

/// Implements all of the required traits for a `Node` type to be created aside from the `Node`
/// trait itself, which needs to be implemented manually.
#[proc_macro_derive(Abstract)]
pub fn r#abstract(input: TokenStream) -> TokenStream {
    
    // Parse the input tokens into a syntax tree,
    // and get the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name:  Ident       = input.ident;

    // Expanded the code to host the boilerplate implementations of the NodeAbstract, Deref, and
    // DerefMut traits.
    let expanded = quote! {
        impl node_tree::traits::node::NodeAbstract for #name {
            fn base(&self) -> &node_tree::structs::node_base::NodeBase {
                &self.base
            }

            fn base_mut(&mut self) -> &mut node_tree::structs::node_base::NodeBase {
                &mut self.base
            }

            fn as_dyn(&self) -> &dyn node_tree::traits::node::Node {
                self
            }

            fn as_dyn_mut(&mut self) -> &mut dyn node_tree::traits::node::Node {
                self
            }

            fn as_dyn_raw(&self) -> *const dyn node_tree::traits::node::Node {
                self as *const dyn node_tree::traits::node::Node
            }
            
            fn as_dyn_raw_mut(&mut self) -> *mut dyn node_tree::traits::node::Node {
                self as *mut dyn node_tree::traits::node::Node
            }
            
            fn to_dyn_box(self) -> Box<dyn node_tree::traits::node::Node> {
                Box::new(self)
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        impl std::ops::Deref for #name {
            type Target = node_tree::structs::node_base::NodeBase;
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }
    };

    // Return the generated impl as a TokenStream
    TokenStream::from(expanded)
}

/// Implements all of the required traits for a `NodeTree` type to be created.
#[proc_macro_derive(Tree)]
pub fn tree(input: TokenStream) -> TokenStream {
    
    // Parse the input tokens into a syntax tree,
    // and get the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name:  Ident       = input.ident;

    // Expanded the code to host the boilerplate implementations of the NodeTree, Deref, and
    // DerefMut traits.
    let expanded = quote! {
        impl node_tree::traits::node_tree::NodeTree for #name {
            unsafe fn set_base(&mut self, base: node_tree::structs::node_tree_base::NodeTreeBase) {
                self.base = Some(base);
            }
            
            fn base(&self) -> &node_tree::structs::node_tree_base::NodeTreeBase {
                unsafe {
                    self.base.as_ref().unwrap_unchecked()
                }
            }

            fn base_mut(&mut self) -> &mut node_tree::structs::node_tree_base::NodeTreeBase {
                unsafe {
                    self.base.as_mut().unwrap_unchecked()
                }
            }

            fn as_dyn(&self) -> &dyn node_tree::traits::node_tree::NodeTree {
                self
            }

            fn as_dyn_mut(&mut self) -> &mut dyn node_tree::traits::node_tree::NodeTree {
                self
            }

            fn as_dyn_raw(&self) -> *const dyn node_tree::traits::node_tree::NodeTree {
                self as *const dyn node_tree::traits::node_tree::NodeTree
            }
            
            fn as_dyn_raw_mut(&mut self) -> *mut dyn node_tree::traits::node_tree::NodeTree {
                self as *mut dyn node_tree::traits::node_tree::NodeTree
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        impl std::ops::Deref for #name {
            type Target = node_tree::structs::node_tree_base::NodeTreeBase;
            fn deref(&self) -> &Self::Target {
                unsafe {
                    self.base.as_ref().unwrap_unchecked()
                }
            }
        }

        impl std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe {
                    self.base.as_mut().unwrap_unchecked()
                }
            }
        }
    };

    // Return the generated impl as a TokenStream
    TokenStream::from(expanded)
}
