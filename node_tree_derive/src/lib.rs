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
use syn::{ parse_macro_input, DeriveInput, Ident, parse::{ Parse, ParseStream }, Token };
use proc_macro2::TokenStream as TokenStream2;


/*
 * Node
 *      Abstract
 */


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

            fn clone_as_instance(&self) -> Box<dyn Node> {
                Box::new(self.clone())
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


/*
 * Tree
 *      Abstract
 */


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


/*
 * Scene
 *      Macro
 */


struct SceneNode {
    node_type: Ident,
    params:    Option<syn::ExprParen>,
    children:  Vec<SceneNode>,
}

impl Parse for SceneNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let node_type: Ident = input.parse()?;
        
        // Have params be optional.
        let params: Option<syn::ExprParen> = if input.peek(syn::token::Paren) {
            Some(input.parse()?)
        } else {
            None
        };
        
        let mut children: Vec<SceneNode> = Vec::new();
        if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);

            while !content.is_empty() {
                children.push(content.parse()?);
                if !content.is_empty() {
                    content.parse::<Token![,]>()?;
                }
            }
        }
        
        Ok(SceneNode {
            node_type,
            params,
            children,
        })
    }
}

fn generate_node(node: &SceneNode) -> TokenStream2 {
    let node_type: &Ident       = &node.node_type;
    let params:    TokenStream2 = match &node.params {
        Some(p) => quote! { #p },
        None    => quote! { () },
    };
    let children: Vec<TokenStream2> = node.children.iter().map(generate_node).collect();
    
    quote! {
        {
            let mut scene: NodeScene = NodeScene::new(#node_type::new #params);
            #(
                scene.append(#children);
            )*
            scene
        }
    }
}

#[proc_macro]
pub fn scene(input: TokenStream) -> TokenStream {
    let root:     SceneNode    = syn::parse_macro_input!(input as SceneNode);
    let expanded: TokenStream2 = generate_node(&root);
    TokenStream::from(expanded)
}
