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

mod parser;
mod generator;

use parser::*;
use generator::*;

use quote::quote;
use syn::{ parse_macro_input, DeriveInput };
use syn::punctuated as punc;
use proc_macro::TokenStream;
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
    let name:  syn::Ident  = input.ident;

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

            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn clone_as_instance(&self) -> Box<dyn node_tree::traits::node::Node> {
                Box::new(self.clone())
            }

            fn name_as_type(&self) -> String {
                std::any::type_name::<Self>().to_string()
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
 * Register
 */


#[proc_macro_derive(Register)]
pub fn derive_registered(input: TokenStream) -> TokenStream {
    let ast:    DeriveInput             = parse_macro_input!(input as DeriveInput);
    let name:   &syn::Ident             = &ast.ident;
    let fields: &punc::Punctuated<_, _> = match &ast.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
            _ => panic!("Registered trait can only be derived for structs with named fields"),
        },
        _ => panic!("Registered trait can only be derived for structs"),
    };

    let field_names: Vec<_> = fields
        .iter()
        .filter(|field| field.ident.as_ref().unwrap() != "base")
        .map(|field| field.ident.as_ref().unwrap())
        .collect();

    // Initialize deserialization lines from the fields.
    let mut type_definitions: Vec<TokenStream2>      = Vec::new();
    let     type_define_ptr:  *mut Vec<TokenStream2> = &mut type_definitions as *mut _;
    let     deserialization:  Vec<TokenStream2>      = fields.iter()
        .filter(|field| field.ident.as_ref().unwrap() != "base")
        .map(|field| {
            let field_name: &syn::Ident = field.ident.as_ref().expect("Field must be named");
            let field_type: &syn::Type  = &field.ty;
            
            // Create a unique ident for the type; this is to avoid having to parse colons between
            // generic arguments and the type.
            let unique_ident: syn::Ident = syn::Ident::new(&format!("Unique{}", type_definitions.len()), proc_macro::Span::call_site().into());
            unsafe { &mut *type_define_ptr }.push(quote! {
                type #unique_ident = #field_type;
            });
            
            quote! {
                #field_name: {
                    if #unique_ident::is_ghost_export_type() {
                        #unique_ident::void()
                    } else {
                        #unique_ident::from_value(
                            owned_state.remove(stringify!(#field_name)).ok_or(format!("corrupt save data; `{}` missing", stringify!(#field_name)))?
                        ).ok_or(format!("corrupt save data; `{}` invalid type", stringify!(#field_name)))?
                    }
                }
            }
        }).collect(); // We need to collect here so that the unique identities are created here and now!

    let static_name: syn::Ident   = syn::Ident::new(&format!("__static_init_{}", name.to_string().to_lowercase()), name.span());
    let expanded:    TokenStream2 = quote! {
        impl node_tree::traits::registered::Registered for #name {
            fn save_from_owned(&self) -> node_tree::services::node_registry::FieldMap {
                let mut map = node_tree::services::node_registry::FieldMap::new();
                #(
                    map.insert(
                        Box::<str>::from(stringify!(#field_names)),
                        Box::new(self.#field_names.clone()),
                    );
                )*
                map
            }

            fn load_from_owned(mut owned_state: node_tree::services::node_registry::SFieldMap) -> Result<Self, String> where Self: Sized {
                #(#type_definitions)*
                Ok(Self {
                    base: node_tree::prelude::NodeBase::new(stringify!(#name).to_string()),
                    #(#deserialization,)*
                })
            }
        }
        
        // Runs before main.
        #[node_tree::startup::ctor]
        unsafe fn #static_name() {
            use node_tree::prelude::Registered;
            
            node_tree::services::node_registry::register_deserializer(std::any::type_name::<#name>().into(), Box::new(|s_field_map| {
                let node: #name = #name::load_from_owned(s_field_map)?;
                Ok(Box::new(node) as Box<dyn node_tree::traits::node::Node>)
            }));
        };
    };

    expanded.into()
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
    let name:  syn::Ident  = input.ident;

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


/// A simple short-hand way of initializing `NodeScene`s quickly and elegantly.
/// Here is an example of a `NodeScene` initialized in this manner:
/// ```rust, ignore
/// use node_tree::prelude::*;
///
/// let scene: NodeScene = scene! {
///     OwnerNode [
///         NodeWithNoArgs,
///         NodeWithArgs(1, "two"), // Required when _init() is defined with arguments.
///         NodeWithSettings {
///            field1: "value1", // These are public fields that are set on the node.
///            field2: 2
///         },
///         NodeWithChildren [
///             Foo,
///             Bar
///         ],
///         NodeWithChildrenAndSettings {
///             field1: "value1",
///             field2: 2,
///             [
///                 Foo,
///                 Bar
///             ]
///         }
///     ]
/// };
///
/// let named_scene: NodeScene = scene! {
///     Owner: "Owner" [
///         Child: "ChildA",
///         Child: "ChildB"
///     ]
/// };
///
/// let complex_scene: NodeScene = scene! {
///     RootNode [
///         NodeA,
///         NodeB [
///             NodeC,
///             $scene // Links a scene with the name following `$` to that position...
///         ]
///     ]
/// };
/// ```
#[proc_macro]
pub fn scene(input: TokenStream) -> TokenStream {
    let root:     SceneNode    = syn::parse_macro_input!(input as SceneNode);
    let expanded: TokenStream2 = generate_node(&root);

    TokenStream::from(quote! {
        {
            use node_tree::traits::element::AsElement;
            
            #expanded
        }
    })
}


/*
 * Class
 *      Macro
 */


/// An easy way to be able to define a Node.
/// 
/// # Example
/// ```rust, ignore
/// use node_tree::prelude::*;
///
/// class! {
///     
///     /// Documentation and attributes are supported!
///     pub declare NodeName extends UniqueTraitGroup1, UniqueTraitGroup2; // Will need to write a separate `impl` for each trait listed here.
///     
///     /// A signal can be connected to node functions and emitted.
///     /// Safety is guaranteed via the scene tree.
///     pub signal on_event(param_name: Type, ..);
///
///     /// Constants are supported.
///     const SOME_CONST: &str = "Hello";
///
///     /// Fields can be defined like so, with default or without default values.
///     let field_uninit:      u8;
///     let field_initialized: String = "These are not constant expressions!".to_string();
///
///     // Fields can have special attributes, like so:
///     default let field_default: u8; // Will automatically initialzie to zero.
///     unique  let field_unique: *mut c_void; // When cloned or serialized, this will safetly be initialized as a `None` value.
///
///     // Exportable fields will be saved and loaded from whenever a node scene is serialized.
///     // Note: All exported types will need to implement the `Exportable` trait.
///     export         let some_parameter: String;
///     export default let some_parameter_default: bool;
///
///     // Hooks are any system functions that can be overridden.
///     // This include the constructor `_init()`, `loaded()`, `ready()`, `process()`, `terminal()`, and `process_mode()`.
///
///     /// The constructor may only need to be implemented if there exists fields that do not have
///     /// a default value.
///     /// Note that this macro will automatically create a `new()` invokation, even without a
///     /// predefined `_init()` hook. All attributes given to this hook will be transferred to the
///     /// `new()` function.
///     hk _init(starter_value: u8) {
///         
///         // Initialize a value by creating a variable with the same field name:
///         let field_uninit: u8 = starter_value;
///     }
///
///     /// Functions can be declared as per usual.
///     fn foo(bar: Type) -> Type {
///         todo!();
///     }
/// }
/// ```
#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    
    // Parse the given macro as a class definition.
    let Class {
        name,
        extends,
        attribs,
        public,
        signals,
        consts,
        fields,
        hooks,
        funcs
    } = parse_macro_input!(input as Class);

    // Generate the class attributes, such as visibility and inherited traits.
    let visibility: TokenStream2 = if public { quote! { pub } } else { TokenStream2::new() };
    
    let extends = extends.iter().map(|inh_trait| {
        quote! {
            node_tree::intertrait::castable_to!(crate = node_tree::intertrait | #name => node_tree::traits::node::Node, #inh_trait);
        }
    });
    
    // Generate the constant fields.
    let const_fields = consts.iter().map(|Const { attribs, public, declare }| {
        let visibility: TokenStream2 = if *public { quote! { pub } } else { TokenStream2::new() };
        quote! {
            #(#attribs)*
            #visibility #declare
        }
    });

    // Generate the signal fields.
    let signal_fields = signals.iter().map(|signal| {
        let Signal {
            name,
            public,
            attribs,
            args
        } = signal;
        
        let visibility: TokenStream2 = if *public { quote! { pub } } else { TokenStream2::new() };
        match args.len() {
            0 => quote! { #(#attribs)* #visibility #name: node_tree::prelude::Signal<()> },
            1 => {
                let only_arg: &syn::Type = &args[0];
                quote! { #(#attribs)* #visibility #name: node_tree::prelude::Signal<#only_arg> }
            },
            _ => quote! { #(#attribs)* #visibility #name: node_tree::prelude::Signal<(#(#args,)*)>}
        }
    });

    // Generate the custom fields.
    let custom_fields = fields.iter().map(|field| {
        let Field {
            name,
            attribs,
            kind,
            public,
            ty,
            ..
        } = field;

        let visibility: TokenStream2 = if *public { quote! { pub } } else { TokenStream2::new() };
        match kind {
            FieldKind::Regular        => quote! { #(#attribs)* #visibility #name: node_tree::structs::node_field::Field<#ty>           },
            FieldKind::Export         => quote! { #(#attribs)* #visibility #name: node_tree::structs::node_field::ExportableField<#ty> },
            FieldKind::ExportDefault  => quote! { #(#attribs)* #visibility #name: node_tree::structs::node_field::ExportableField<#ty> },
            FieldKind::Unique         => quote! { #(#attribs)* #visibility #name: node_tree::structs::node_field::UniqueField<#ty>     },
            FieldKind::Default        => quote! { #(#attribs)* #visibility #name: node_tree::structs::node_field::DefaultField<#ty>    }
        }
    });

    // Generate the constructor.
    // Take note if an _init definition is not required.
    const INIT: &str = "_init";

    let needs_init: bool          = fields.iter().any(|field| field.init.is_none() && !field.kind.supports_default_init());
    let init_hook:  Option<&Hook> = hooks.iter().find(|hook| hook.name == INIT);

    let constructor_signals = signals.iter().map(|signal| {
        let signal_name: &syn::Ident = &signal.name;
        quote! {
            #signal_name: node_tree::prelude::Signal::new()
        }
    });

    let constructor_fields = fields.iter().map(|field| {
        let Field {
            name,
            kind,
            ty,
            ..
        } = field;

        match kind {
            FieldKind::Regular => if let Some(default_value) = &field.init {
                quote! {
                    #name: node_tree::structs::node_field::Field::new(#default_value)
                }
            } else {
                quote! {
                    #name: node_tree::structs::node_field::Field::new(#name)
                }
            },
            FieldKind::Export => if let Some(default_value) = &field.init {
                quote! {
                    #name: node_tree::structs::node_field::ExportableField::new(#default_value)
                }
            } else {
                quote! {
                    #name: node_tree::structs::node_field::ExportableField::new(#name)
                }
            },
            FieldKind::ExportDefault => quote! {
                #name: node_tree::structs::node_field::ExportableField::new(#ty::default())
            }, 
            FieldKind::Unique => if let Some(default_value) = &field.init {
                quote! {
                    #name: node_tree::structs::node_field::UniqueField::new(#default_value)
                }
            } else {
                quote! {
                    #name: node_tree::structs::node_field::UniqueField::new(#name)
                }
            },
            FieldKind::Default => quote! {
                #name: node_tree::structs::node_field::DefaultField::new(#ty::default())
            }
        }
    });

    // Generate other hook implementations.
    let hook_impls = hooks.iter().filter(|hook| hook.name != INIT).map(|hook| {
        let Hook {
            name,
            attribs,
            sig,
            args,
            out,
            body
        } = hook;

        let out: TokenStream2 = match out {
            Some(out) => quote! { -> #out },
            None      => TokenStream2::new()
        };

        quote! {
            #(#attribs)*
            fn #name(#sig #(, #args)*) #out {
                #body
            }
        }
    });

    // Generate the functions.
    let func_impls = funcs.iter().map(|Func { attribs, public, declare }| {
        let visibility: TokenStream2 = if *public { quote! { pub } } else { TokenStream2::new() };
        quote! {
            #(#attribs)*
            #visibility #declare
        }
    });

    // Generate the final implementation.
    let constructor: TokenStream2 = match init_hook {
        Some(init_hook) => {
            let init_attribs: &[syn::Attribute] = &init_hook.attribs;
            let init_args:    &[syn::PatType]   = &init_hook.args;
            let init_body:    &[syn::Stmt]      = &init_hook.body.stmts;

            quote! {
                #(#init_attribs)*
                #visibility fn new(#(#init_args),*) -> Self {
                    #(#init_body)*
                    Self {
                        base: node_tree::prelude::NodeBase::new(stringify!(#name).to_string()),
                        #(#constructor_signals,)*
                        #(#constructor_fields,)*
                    }
                }

            }
        },
        None => {
            if needs_init {
                panic!("Requires an _init() hook");
            }

            quote! {
                #visibility fn new() -> Self {
                    Self {
                        base: node_tree::prelude::NodeBase::new(stringify!(#name).to_string()),
                        #(#constructor_signals,)*
                        #(#constructor_fields,)*
                    }
                }
            }
        }
    };

    let expanded: TokenStream2 = quote! {
        #(#attribs)*
        #[derive(Debug, Clone, node_tree::prelude::Abstract, node_tree::prelude::Register)]
        #visibility struct #name {
            base: node_tree::prelude::NodeBase,
            #(#signal_fields,)*
            #(#custom_fields,)*
        }

        impl #name {
            #(#const_fields)*
            #constructor
            #(#func_impls)*
        }

        impl node_tree::prelude::Node for #name {
            #(#hook_impls)*
        }

        #(#extends)*
    };
    TokenStream::from(expanded)
}


/*
 * Connect
 *      Macro
 */


/// Allows for a safe abstraction for connecting listener functions in other nodes via `Tp<T>` to a
/// signal.
///
/// # Note
/// - This will enforce the use of tree pointers (`Tp<T>`).
/// - Must be called within a node's member function or hook.
///
/// # Example
/// ```rust, ignore
/// // Assuming that this is within a node's member function or hook.
/// let tp: Tp<YourNode> = todo!();
///
/// connect! { signal_name -> tp.constant_listener };
/// connect! { signal_name ~> tp.one_shot_listener };
/// ```
#[proc_macro]
pub fn connect(input: TokenStream) -> TokenStream {
    let Connection {
        signal_name,
        one_shot,
        tree_pointer,
        callback
    } = parse_macro_input!(input as Connection);

    // TODO: Support argument passing!
    
    let connect_type: TokenStream2 = if one_shot { quote! { connect_once } } else { quote! { connect } };
    TokenStream::from(quote! {
        unsafe { // Enforce `move,` as without it a segfault occurs!
            let tp_: node_tree::prelude::Tp<_> = #tree_pointer;
            self.#signal_name.#connect_type(move |args| {
                tp_.#callback(&args)
            });
        }
    })
}
