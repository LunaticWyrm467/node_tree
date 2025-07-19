use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

use crate::parser::*;


/*
 * Scene
 *      Macro
 */


pub fn generate_node(node: &SceneNode) -> TokenStream2 {
    match node {
        SceneNode::Link(with) => {
            quote! {
                #with.clone()
            }
        },
        SceneNode::Node { node_type, params, name, children } => {
            let params: TokenStream2 = match params {
                Some(p) => quote! { (#p) },
                None    => quote! { () },
            };
            let name_set: TokenStream2 = match name {
                None       => quote! {},
                Some(name) => quote! {
                    unsafe {
                        node.set_name_unchecked(#name);
                    }
                }
            };
            let children: Vec<TokenStream2> = children.iter().map(generate_node).collect();

            quote! {
                {
                    let mut node: #node_type = #node_type::new #params;
                    #name_set
                                        
                    let mut scene: NodeScene = NodeScene::new(node);
                    #(
                        scene.append(#children);
                    )*
                    scene
                }
            }
        }
    }
}