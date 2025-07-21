use syn::{ parenthesized, parse::{ Parse, ParseStream }, Receiver, Token };
use syn::token as tok;
use syn::punctuated as punc;


/*
 * Scene
 *      Macro
 */


pub enum SceneNode {
    Link (syn::Ident),
    Node {
        node_type: syn::Path,
        params:    Option<punc::Punctuated<syn::Expr, tok::Comma>>,
        settings:  Vec<(syn::Ident, syn::Expr)>,
        name:      Option<syn::LitStr>,
        children:  Vec<SceneNode>
    }
}

impl Parse for SceneNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        
        // Determine if this is a link or a node.
        if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            Ok(SceneNode::Link(input.parse()?))
        } else {
            let node_type: syn::Path = input.parse()?;

            // Parse optional parameters
            let params: Option<punc::Punctuated<syn::Expr, tok::Comma>> = if input.peek(tok::Paren) {
                let content;
                syn::parenthesized!(content in input);
                Some(punc::Punctuated::parse_terminated(&content)?)
            } else {
                None
            };

            // Parse a name if given.
            let mut name: Option<syn::LitStr> = None;
            if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                name = Some(input.parse()?);
            }

            // Determine how to parse this going foward:
            // If there is a brace, then this is a node with settings and potentially children.
            // Otherwise, if there is a square bracket, then we just skip to parsing children.
            let mut settings: Vec<(syn::Ident, syn::Expr)> = Vec::new();
            let mut children: Vec<SceneNode>               = Vec::new();

            fn parse_children(input: &ParseStream) -> syn::Result<Vec<SceneNode>> {
                let mut children: Vec<SceneNode> = Vec::new();
                
                // Parse children.
                if input.peek(tok::Bracket) {
                    let content;
                    syn::bracketed!(content in input);

                    while !content.is_empty() {
                        children.push(content.parse()?);
                        if !content.is_empty() {
                            content.parse::<Token![,]>()?;
                        }
                    }
                }

                Ok(children)
            }

            // Parse settings and potentially children.
            if input.peek(tok::Brace) {
                let content;
                syn::braced!(content in input);

                loop {
                    if content.peek(syn::Ident) {
                        let key:    syn::Ident = content.parse()?;
                        let _colon: tok::Colon = content.parse()?;
                        let value:  syn::Expr  = content.parse()?;

                        settings.push((key, value));
                    }

                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    } else {
                        break;
                    }
                }

                // If there is a square bracket, parse children.
                if content.peek(tok::Bracket) {
                    children = parse_children(&&content)?;
                }
            }
            
            // Only parse children.
            else if input.peek(tok::Bracket) {
                children = parse_children(&input)?;
            }
            
            

            Ok(SceneNode::Node {
                node_type,
                params,
                settings,
                name,
                children,
            })
        }
    }
}


/*
 * Class
 *      Macro
 */


pub struct Class {
    pub name:    syn::Ident,
    pub extends: Vec<syn::Ident>,
    pub attribs: Vec<syn::Attribute>,
    pub public:  bool,
    pub signals: Vec<Signal>,
    pub consts:  Vec<Const>,
    pub fields:  Vec<Field>,
    pub hooks:   Vec<Hook>,
    pub funcs:   Vec<Func>
}

pub struct Signal {
    pub name:    syn::Ident,
    pub public:  bool,
    pub attribs: Vec<syn::Attribute>,
    pub args:    Vec<syn::Type>
}

pub struct Const {
    pub attribs: Vec<syn::Attribute>,
    pub public:  bool,
    pub declare: syn::ItemConst
}

#[derive(PartialEq, Eq)]
pub enum FieldKind {
    Regular,
    Export,
    ExportDefault,
    Unique,
    Default
}

impl FieldKind {
    
    /// Returns whether a field supports a defualt initialization.
    pub fn supports_default_init(&self) -> bool {
        match self {
            FieldKind::ExportDefault => true,
            FieldKind::Default       => true,
            _                        => false
        }
    }
}

pub struct Field {
    pub name:    syn::Ident,
    pub attribs: Vec<syn::Attribute>,
    pub public:  bool,
    pub ty:      syn::Type,
    pub kind:    FieldKind,
    pub init:    Option<syn::Expr>
}

pub struct Hook {
    pub name:    syn::Ident,
    pub attribs: Vec<syn::Attribute>,
    pub sig:     Option<syn::Receiver>,
    pub args:    Vec<syn::PatType>,
    pub out:     Option<syn::Ident>,
    pub body:    syn::Block
}

pub struct Func {
    pub attribs: Vec<syn::Attribute>,
    pub public:  bool,
    pub declare: syn::ItemFn
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
        let class_dec:  syn::Ident = input.parse()?;
        let class_name: syn::Ident = input.parse()?;
        
        if class_dec != "declare" {
            return Err(syn::Error::new_spanned(class_dec, "Expected a class declaration starting with `dec`"));
        }

        let mut class_extends: Vec<syn::Ident> = Vec::new(); // Parse extends
        if input.peek(syn::Ident) {
            let extends: syn::Ident = input.parse()?;
            if extends != "extends" {
                return Err(syn::Error::new_spanned(extends, "Expected 'extends' after 'declare'"));
            }

            class_extends = punc::Punctuated::<syn::Ident, Token![,]>::parse_separated_nonempty(input)?
                .into_iter()
                .collect();
        }

        let _semi_token: tok::Semi = input.parse()?;

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
                    "signal" => {
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
            extends: class_extends,
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


/*
 * Connect
 *      Macro
 */


pub struct Connection {
    pub signal_name:  syn::Ident,
    pub one_shot:     bool,
    pub tree_pointer: syn::Ident,
    pub callback:     syn::Ident
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