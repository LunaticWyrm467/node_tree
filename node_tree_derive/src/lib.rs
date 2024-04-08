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
//! Contains the `NodeSys` derive macro which helps reduce boilerplate in regards to trait
//! implementations.
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput, Ident };

#[proc_macro_derive(NodeSys)]
pub fn node_sys_derive(input: TokenStream) -> TokenStream {
    
    // Parse the input tokens into a syntax tree,
    // and get the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name:  Ident       = input.ident;

    // Expanded the code to host the boilerplate implementations of the Dynamic and NodeAbstract
    // traits.
    let expanded = quote! {
        impl Dynamic for #name {
            fn to_any(&self) -> &dyn std::any::Any { self }
        }

        impl NodeAbstract for #name {
            fn as_dyn(self: Hp<Self>) -> DynNode { self }
            fn base(self: Hp<Self>) -> std::rc::Rc<NodeBase> { self.base.clone() }
        }
    };

    // Return the generated impl as a TokenStream
    TokenStream::from(expanded)
}
