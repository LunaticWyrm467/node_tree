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

#[proc_macro_derive(Abstract)]
pub fn node_sys_derive(input: TokenStream) -> TokenStream {
    
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

            fn as_dyn(&mut self) -> &mut dyn node_tree::traits::node::Node {
                self
            }

            fn as_dyn_raw(&mut self) -> *mut dyn node_tree::traits::node::Node {
                self as *mut dyn node_tree::traits::node::Node
            }
            
            fn to_dyn_box(self) -> Box<dyn node_tree::traits::node::Node> {
                Box::new(self)
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
