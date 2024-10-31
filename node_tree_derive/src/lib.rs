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


struct SceneNode {
    node_type: syn::Ident,
    params:    Option<punc::Punctuated<syn::Expr, tok::Comma>>,
    children:  Vec<SceneNode>,
}

impl Parse for SceneNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let node_type: syn::Ident = input.parse()?;
        
        // Parse optional parameters
        let params: Option<punc::Punctuated<syn::Expr, tok::Comma>> = if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            Some(punc::Punctuated::parse_terminated(&content)?)
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
    let node_type: &syn::Ident  = &node.node_type;
    let params:    TokenStream2 = match &node.params {
        Some(p) => quote! { (#p) },
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

/// A simple short-hand way of initializing `NodeScene`s quickly and elegantly.
/// Here is an example of a `NodeScene` initialized in this manner:
/// ```rust, ignore
/// use node_tree::prelude::*;
///
/// let scene: NodeScene = scene! {
///     RootNode {
///         NodeWithNoArgs,
///         NodeWithOneArg(1),
///         NodeWithTwoArgs(1, "two"),
///         NodeWithChildren {
///             Foo,
///             Bar
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
    declare: syn::ItemConst
}

struct Field {
    name:    syn::Ident,
    attribs: Vec<syn::Attribute>,
    public:  bool,
    ty:      syn::Type,
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

            // Parse a constant:
            if input.peek(Token![const]) {
                consts.push(Const {
                    attribs: item_attribs,
                    declare: input.parse()?
                });
            }

            // Parse a function:
            else if input.peek(Token![fn]) {
                funcs.push(Func {
                    attribs: item_attribs,
                    declare: input.parse()?
                });
            }

            // Parse a custom statement:
            else {

                // Check if the following statement is publicly visible:
                let mut is_public: Option<tok::Pub> = None;
                if input.peek(Token![pub]) {
                    is_public = Some(input.parse::<tok::Pub>()?);
                }

                // Parse a let statement:
                if input.peek(Token![let]) {
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

                    input.parse::<tok::Semi>()?;

                    // Append it to the fields group.
                    fields.push(Field {
                        name,
                        attribs: item_attribs,
                        public:  is_public.is_some(),
                        ty,
                        init: default_value
                    });
                }

                // Parse a signal or hook statement:
                else if input.peek(syn::Ident) {
                    let token:      syn::Ident = input.parse()?;
                    let token_name: &str       = &token.to_string();
                    match token_name {
                        "sig" => {
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
                } else {
                    panic!("Unknown token!");
                }
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
///     // Hooks are any system functions that can be overridden.
///     // This include the constructor `_init()`, `ready()`, `process()`, `terminal()`, and `process_mode()`.
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
    let const_fields = consts.iter().map(|Const { attribs, declare }| {
        quote! {
            #(#attribs)*
            #declare
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
            0 => quote! { #(#attribs)* #visibility #name: node_tree::prelude::Doc<node_tree::prelude::Signal<()>> },
            1 => {
                let only_arg: &syn::Type = &args[0];
                quote! { #(#attribs)* #visibility #name: node_tree::prelude::Doc<node_tree::prelude::Signal<#only_arg>> }
            },
            _ => quote! { #(#attribs)* #visibility #name: node_tree::prelude::Doc<node_tree::prelude::Signal<(#(#args,)*)>>}
        }
    });

    // Generate the custom fields.
    let custom_fields = fields.iter().map(|field| {
        let Field {
            name,
            attribs,
            public,
            ty,
            ..
        } = field;

        let visibility: TokenStream2 = if *public { quote! { pub } } else { TokenStream2::new() };
        quote! {
            #(#attribs)* #visibility #name: #ty
        }
    });

    // Generate the constructor.
    // Take note if an _init definition is not required.
    const INIT: &str = "_init";

    let needs_init: bool          = fields.iter().any(|field| field.init.is_none());
    let init_hook:  Option<&Hook> = hooks.iter().find(|hook| hook.name == INIT);

    let constructor_signals = signals.iter().map(|signal| {
        let signal_name: &syn::Ident = &signal.name;
    
        quote! {
            #signal_name: node_tree::prelude::Doc::new(node_tree::prelude::Signal::new())
        }
    });

    let constructor_fields = fields.iter().map(|field| {
        let field_name: &syn::Ident = &field.name;
        if let Some(default_value) = &field.init {
            quote! {
                #field_name: #default_value
            }
        } else {
            quote! {
                #field_name
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
    let func_impls = funcs.iter().map(|Func { attribs, declare }| {
        quote! {
            #(#attribs)*
            #declare
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
        #[derive(Debug, Clone, node_tree::prelude::Abstract)]
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
