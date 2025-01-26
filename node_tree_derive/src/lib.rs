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

use quote::quote;
use syn::{ parenthesized, parse::{ Parse, ParseStream }, parse_macro_input, DeriveInput, Receiver, Token };
use syn::token as tok;
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

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn clone_as_instance(&self) -> Box<dyn Node> {
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
        impl Registered for #name {
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
        #[node_tree::ctor::ctor]
        unsafe fn #static_name() {
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


enum SceneNode {
    Link (syn::Ident),
    Node {
        node_type: syn::Ident,
        params:    Option<punc::Punctuated<syn::Expr, tok::Comma>>,
        name:      Option<syn::LitStr>,
        children:  Vec<SceneNode>,
    }
}

impl Parse for SceneNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        
        // Determine if this is a link or a node.
        if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            Ok(SceneNode::Link(input.parse()?))
        } else {
            let node_type: syn::Ident = input.parse()?;

            // Parse optional parameters
            let params: Option<punc::Punctuated<syn::Expr, tok::Comma>> = if input.peek(syn::token::Paren) {
                let content;
                syn::parenthesized!(content in input);
                Some(punc::Punctuated::parse_terminated(&content)?)
            } else {
                None
            };

            // Parse a name if given.
            let mut name: Option<syn::LitStr> = None;
            if input.peek(tok::Colon) {
                input.parse::<tok::Colon>()?;
                name = Some(input.parse()?);
            }
            
            // Parse children.
            let mut children: Vec<SceneNode> = Vec::new();
            if input.peek(tok::Brace) {
                let content;
                syn::braced!(content in input);

                while !content.is_empty() {
                    children.push(content.parse()?);
                    if !content.is_empty() {
                        content.parse::<Token![,]>()?;
                    }
                }
            }

            Ok(SceneNode::Node {
                node_type,
                params,
                name,
                children,
            })
        }
    }
}

fn generate_node(node: &SceneNode) -> TokenStream2 {
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

/// A simple short-hand way of initializing `NodeScene`s quickly and elegantly.
/// Here is an example of a `NodeScene` initialized in this manner:
/// ```rust, ignore
/// use node_tree::prelude::*;
///
/// let scene: NodeScene = scene! {
///     OwnerNode {
///         NodeWithNoArgs,
///         NodeWithOneArg(1),
///         NodeWithTwoArgs(1, "two"),
///         NodeWithChildren {
///             Foo,
///             Bar
///         }
///     }
/// };
///
/// let named_scene: NodeScene = scene! {
///     Owner: "Owner" {
///         Child: "ChildA",
///         Child: "ChildB"
///     }
/// };
///
/// let complex_scene: NodeScene = scene! {
///     RootNode {
///         NodeA,
///         NodeB {
///             NodeC,
///             $scene // Links a scene with the name following `$` to that position...
///         }
///     }
/// };
/// ```
#[proc_macro]
pub fn scene(input: TokenStream) -> TokenStream {
    let root:     SceneNode    = syn::parse_macro_input!(input as SceneNode);
    let expanded: TokenStream2 = generate_node(&root);
    TokenStream::from(expanded)
}


/*
 * Class
 *      Macro
 */


struct Class {
    name:    syn::Ident,
    attribs: Vec<syn::Attribute>,
    public:  bool,
    signals: Vec<Signal>,
    consts:  Vec<Const>,
    fields:  Vec<Field>,
    hooks:   Vec<Hook>,
    funcs:   Vec<Func>
}

struct Signal {
    name:    syn::Ident,
    public:  bool,
    attribs: Vec<syn::Attribute>,
    args:    Vec<syn::Type>
}

struct Const {
    attribs: Vec<syn::Attribute>,
    public:  bool,
    declare: syn::ItemConst
}

#[derive(PartialEq, Eq)]
enum FieldKind {
    Regular,
    Export,
    ExportDefault,
    Unique,
    Default
}

impl FieldKind {
    
    /// Returns whether a field supports a defualt initialization.
    fn supports_default_init(&self) -> bool {
        match self {
            FieldKind::ExportDefault => true,
            FieldKind::Default       => true,
            _                        => false
        }
    }
}

struct Field {
    name:    syn::Ident,
    attribs: Vec<syn::Attribute>,
    public:  bool,
    ty:      syn::Type,
    kind:    FieldKind,
    init:    Option<syn::Expr>
}

struct Hook {
    name:    syn::Ident,
    attribs: Vec<syn::Attribute>,
    sig:     Option<syn::Receiver>,
    args:    Vec<syn::PatType>,
    out:     Option<syn::Ident>,
    body:    syn::Block
}

struct Func {
    attribs: Vec<syn::Attribute>,
    public:  bool,
    declare: syn::ItemFn
}


impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        // Determine any class attributes.
        let class_attribs: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        
        // Determine if this class is public.
        let public: bool = if input.peek(Token![pub]) {
            input.parse::<tok::Pub>()?;
            true
        } else {
            false
        };

        // Parse the declaration statement.
        let  class_dec:  syn::Ident = input.parse()?;
        let  class_name: syn::Ident = input.parse()?;
        let _semi_token: tok::Semi  = input.parse()?;
        
        if class_dec != "dec" {
            return Err(syn::Error::new_spanned(class_dec, "Expected a class declaration starting with `dec`"));
        }

        // Go through the class's fields and hooks.
        let mut signals: Vec<Signal> = Vec::new();
        let mut consts:  Vec<Const>  = Vec::new();
        let mut fields:  Vec<Field>  = Vec::new();
        let mut hooks:   Vec<Hook>   = Vec::new();
        let mut funcs:   Vec<Func>   = Vec::new();
        
        while !input.is_empty() {

            // Parse any item attributes.
            let item_attribs: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;

            // Check if the following statement is publicly visible:
            let mut is_public: Option<tok::Pub> = None;
            if input.peek(Token![pub]) {
                is_public = Some(input.parse::<tok::Pub>()?);
            }

            // Parse the item kind or a unique statement keyword if there is one.
            let mut item_kind:      FieldKind          = FieldKind::Regular;
            let mut unique_starter: Option<syn::Ident> = None;
            if input.peek(syn::Ident) {
                let token:      syn::Ident = input.parse::<syn::Ident>()?;
                let token_name: &str       = &token.to_string();
                match token_name {
                    "export" => {
                        if input.peek(syn::Ident) {
                            let next_token: syn::Ident = input.parse::<syn::Ident>()?;
                            if &next_token.to_string() == "default" {
                                item_kind = FieldKind::ExportDefault;   
                            } else {
                                return Err(syn::Error::new_spanned(next_token, "'export' only supports 'default' as a secondary attribute"));
                            }
                        } else {
                            item_kind = FieldKind::Export;
                        }
                    },
                    "unique"  => item_kind = FieldKind::Unique,
                    "default" => item_kind = FieldKind::Default,
                    _         => unique_starter = Some(token)
                }
            }

            // Parse a custom statement.
            if let Some(token) = unique_starter {
                let token_name: &str = &token.to_string();
                match token_name {
                    "sig" => {
                        if item_kind != FieldKind::Regular {
                            return Err(syn::Error::new_spanned(token, "Signals cannot have field attributes"));
                        }
                        let signal_name: syn::Ident = input.parse()?;

                        // Parse the arguments.
                        let content;
                        parenthesized!(content in input);

                        let signal_args: Vec<syn::Type> = punc::Punctuated::<syn::FnArg, Token![,]>::parse_terminated(&content)?
                            .into_iter()
                            .map(|arg: syn::FnArg| {
                                match arg {
                                    syn::FnArg::Typed(pat)    => Ok(*pat.ty),
                                    syn::FnArg::Receiver(rec) => Err(syn::Error::new_spanned(rec, "Signals cannot have a reciever"))
                                }
                            })
                        .collect::<syn::Result<Vec<_>>>()?;

                        input.parse::<tok::Semi>()?;
                        signals.push(Signal {
                            name:    signal_name,
                            public:  is_public.is_some(),
                            attribs: item_attribs,
                            args:    signal_args
                        });
                    },

                    "hk" => {
                        if let Some(public) = is_public {
                            return Err(syn::Error::new_spanned(public, "Hooks cannot have visibility modifiers"));
                        }
                        if item_kind != FieldKind::Regular {
                            return Err(syn::Error::new_spanned(token, "Hooks cannot have field attributes"));
                        }
                        let hook_name: syn::Ident = input.parse()?;

                        // Parse the arguments.
                        let content;
                        parenthesized!(content in input);

                        let mut reciever: Option<Receiver>  = None;
                        let     args:     Vec<syn::PatType> = punc::Punctuated::<syn::FnArg, Token![,]>::parse_terminated(&content)?
                            .into_iter()
                            .filter_map(|arg: syn::FnArg| {
                                match arg {
                                    syn::FnArg::Receiver(rec)   => { reciever = Some(rec); None }, 
                                    syn::FnArg::Typed(pat_type) => Some(pat_type)
                                }
                            })
                        .collect::<Vec<_>>();

                        // Parse the output (if there is one!).
                        let out: Option<syn::Ident> = if input.peek(Token![->]) {
                            input.parse::<Token![->]>()?;
                            Some(input.parse()?)
                        } else {
                            None
                        };

                        let body: syn::Block = input.parse()?;
                        hooks.push(Hook {
                            name:    hook_name,
                            attribs: item_attribs,
                            sig:     reciever,
                            args,
                            out,
                            body
                        });
                    },

                    _ => return Err(syn::Error::new_spanned(token, format!("Unknown token defined: {}", token_name))) 
                }
            }

            // Parse a let statement:
            else if item_kind != FieldKind::Regular || input.peek(Token![let]) {
                input.parse::<tok::Let>()?;

                let  name:  syn::Ident = input.parse()?;
                let _colon: tok::Colon = input.parse()?;
                let  ty:    syn::Type  = input.parse()?;

                // Check for a default value.
                let default_value: Option<syn::Expr> = if input.peek(Token![=]) {
                    input.parse::<tok::Eq>()?;
                    Some(input.parse::<syn::Expr>()?)
                } else {
                    None
                };

                if let Some(ref default_value) = default_value {
                    if item_kind == FieldKind::Default {
                        return Err(syn::Error::new_spanned(default_value, "A field with the attribute `default` cannot have an initialized value"));
                    }
                }

                input.parse::<tok::Semi>()?;

                // Append it to the fields group.
                fields.push(Field {
                    name,
                    attribs: item_attribs,
                    kind:    item_kind,
                    public:  is_public.is_some(),
                    ty,
                    init:    default_value
                });
            }

            // Parse a constant:
            else if input.peek(Token![const]) {
                let declare: syn::ItemConst = input.parse()?; 
                if item_kind != FieldKind::Regular {
                    return Err(syn::Error::new_spanned(declare, "Fonstants cannot have field attributes"));
                }

                consts.push(Const {
                    attribs: item_attribs,
                    public:  is_public.is_some(),
                    declare 
                });
            }

            // Parse a function:
            else if input.peek(Token![fn]) {
                let declare: syn::ItemFn = input.parse()?;
                if item_kind != FieldKind::Regular {
                    return Err(syn::Error::new_spanned(declare, "Functions cannot have field attributes"));
                }

                funcs.push(Func {
                    attribs: item_attribs,
                    public:  is_public.is_some(),
                    declare
                });
            }

            // Otherwise panic
            else {
                panic!("Unknown token!");
            }
        }
        
        Ok(Class {
            name:    class_name,
            attribs: class_attribs,
            public,
            signals,
            consts,
            fields,
            hooks,
            funcs
        })
    }
}


/// An easy way to be able to define a Node.
/// 
/// # Example
/// ```rust, ignore
/// use node_tree::prelude::*;
///
/// class! {
///     
///     /// Documentation and attributes are supported!
///     pub dec NodeName;
///     
///     /// A signal can be connected to node functions and emitted.
///     /// Safety is guaranteed via the scene tree.
///     pub sig on_event(param_name: Type, ..);
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
        attribs,
        public,
        signals,
        consts,
        fields,
        hooks,
        funcs
    } = parse_macro_input!(input as Class);
    let visibility: TokenStream2 = if public { quote! { pub } } else { TokenStream2::new() };
    
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
    };
    TokenStream::from(expanded)
}


/*
 * Connect
 *      Macro
 */


struct Connection {
    signal_name:  syn::Ident,
    one_shot:     bool,
    tree_pointer: syn::Ident,
    callback:     syn::Ident
}

impl Parse for Connection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let     signal_name: syn::Ident = input.parse()?;
        let mut one_shot:    bool       = false;

        if input.peek(Token![~]) {
            input.parse::<Token![~]>()?;
            input.parse::<Token![>]>()?;
            
            one_shot = true;
        } else {
            input.parse::<Token![->]>()?;
        }

        let  tree_pointer: syn::Ident = input.parse()?;
        let _punct:        tok::Dot   = input.parse()?;
        let  callback:     syn::Ident = input.parse()?;

        Ok(Connection {
            signal_name,
            one_shot,
            tree_pointer,
            callback
        })
    }
}

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
