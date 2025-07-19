//===================================================================================================================================================================================//
//
//   /$$$$$$  /$$                               /$$          
//  /$$__  $$|__/                              | $$          
// | $$  \__/ /$$  /$$$$$$  /$$$$$$$   /$$$$$$ | $$  /$$$$$$$
// |  $$$$$$ | $$ /$$__  $$| $$__  $$ |____  $$| $$ /$$_____/
//  \____  $$| $$| $$  \ $$| $$  \ $$  /$$$$$$$| $$|  $$$$$$ 
//  /$$  \ $$| $$| $$  | $$| $$  | $$ /$$__  $$| $$ \____  $$
// |  $$$$$$/| $$|  $$$$$$$| $$  | $$|  $$$$$$$| $$ /$$$$$$$/
//  \______/ |__/ \____  $$|__/  |__/ \_______/|__/|_______/ 
//                /$$  \ $$                                  
//               |  $$$$$$/                                  
//                \______/                                   
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Includes the `Signal<T>` type, which allows for connections between functions across different
//! nodes, with safety being guaranteed by the `Tp<T>` smart pointer!
//! 

use std::mem;
use std::sync::{ Arc, Mutex, MutexGuard };

use toml_edit as toml;

use crate::structs::rid::{ RID, RIDHolder };
use crate::traits::exportable::{ Voidable, Exportable };
use crate::traits::element::Element;


/// Defines the nature of a connection.
#[derive(Debug, Clone, Copy)]
enum ConnectionType {
    Once,
    UntilDisconnected
}

type MutableArc<T>   = Arc<Mutex<T>>;
type EventHandler<T> = RIDHolder<(*mut dyn FnMut(&T), ConnectionType)>;


/// A type used to define a signal in a Node.
/// A signal is a special event handler which can have listener hooks or connections, of which will
/// activate when a signal is emitted.
///
/// # Example Declaration
/// ```rust, ignore
/// class! {
///     declare YourNode;
///
///     pub signal on_refresh();
///     pub signal on_click(at: Vec2); // Parameter names don't matter - only used for readability!
///     pub signal on_element_hovered(element_id: u64, element_active: bool);
/// }
/// ```
#[derive(Debug)]
pub struct Signal<T> {
    hooks: MutableArc<EventHandler<T>>
}

impl <T> Signal<T> {

    /// Creates a new Signal.
    pub fn new() -> Self {
        Signal {
            hooks: Arc::new(Mutex::new(RIDHolder::new()))
        }
    }
    
    /// Creates a connection between a passed in closure and this signal.
    /// Everytime this signal is emitted, the closure will be called.
    ///
    /// Returns the RID of the connection.
    ///
    /// # Safety
    /// Due to lifetime guarantees, this function's safety relies on the passed closure having the
    /// `move` signature, along with it only accessing fields via tree pointers.
    ///
    /// It is best to use this function implicitly via the `connect!` macro:
    /// ```rust, ignore
    /// // Assuming that this is within a member function of a node.
    /// let node: Tp<YourNode> = todo!();
    /// let rid:  RID          = connect! { signal_name -> node.signal_handler_fn };
    /// ```
    /// Note that `->` is used to designate an indefinite connection, and that `connect!` actively
    /// checks if `node` is a `Tp<T>` or a `TpDyn`.
    pub unsafe fn connect<'a>(&self, callback: impl FnMut(&T) + 'a) -> RID {
        let callback_box: Box<dyn FnMut(&T) + 'a> = Box::new(callback);
        let callback_ext: Box<dyn FnMut(&T)>      = unsafe { mem::transmute(callback_box) };
        let callback_raw: *mut dyn FnMut(&T)      = Box::into_raw(callback_ext);

        self.hooks.lock().unwrap().push((callback_raw, ConnectionType::UntilDisconnected))
    }

    /// Creates a connection between a passed in closure and this signal.
    /// Once this signal is emitted, the closure will be called and the connection will be
    /// terminated.
    ///
    /// Returns the RID of the connection.
    ///
    /// # Safety
    /// Due to lifetime guarantees, this function's safety relies on the passed closure having the
    /// `move` signature, along with it only accessing fields via tree pointers.
    ///
    /// It is best to use this function implicitly via the `connect!` macro:
    /// ```rust, ignore
    /// // Assuming that this is within a member function of a node.
    /// let node: Tp<YourNode> = todo!();
    /// let rid:  RID          = connect! { signal_name ~> node.signal_handler_fn };
    /// ```
    /// Note that `~>` is used to designate a one-time use connection, and that `connect!` actively
    /// checks if `node` is a `Tp<T>` or a `TpDyn`.
    pub unsafe fn connect_once<'a>(&self, callback: impl FnMut(&T) + 'a) -> RID {
        let callback_box: Box<dyn FnMut(&T) + 'a> = Box::new(callback);
        let callback_ext: Box<dyn FnMut(&T)>      = unsafe { mem::transmute(callback_box) };
        let callback_raw: *mut dyn FnMut(&T)      = Box::into_raw(callback_ext);

        self.hooks.lock().unwrap().push((callback_raw, ConnectionType::Once))
    }
    
    /// Emits the signal, calling all connected hooks.
    pub fn emit<E: Element<T>>(&self, parameters: E) {
        let mut hooks:           MutexGuard<EventHandler<T>> = self.hooks.lock().unwrap();
        let mut removed_signals: Vec<RID>                    = Vec::with_capacity(hooks.len());
        let     parameters:      &T                          = parameters.as_inner();

        for (&rid, &(hook, mode)) in hooks.iter_enumerated() {
            unsafe {
                (*hook)(parameters);
            }

            match mode {
                ConnectionType::UntilDisconnected => (),
                ConnectionType::Once              => removed_signals.push(rid)
            }
        }

        for idx in removed_signals.into_iter().rev() {
            hooks.take(idx);
        }
    }

    /// Disconnects a connection given its RID.
    /// Returns whether the connection was successfully disconnected.
    pub fn disconnect(&self, rid: RID) -> bool {
        self.hooks.lock().unwrap().take(rid).is_some()
    }
}

impl <T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl <T> Default for Signal<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl <T> Voidable for Signal<T> {
    fn void() -> Self {
        Self::new()
    }
}

impl <T> Exportable for Signal<T> {
    unsafe fn is_ghost_export(&self) -> bool { true }

    fn to_value(&self) -> toml::Value {
        unimplemented!()
    }

    fn from_value(_value: toml::Value) -> Option<Self> where Self: Sized {
        unimplemented!()
    }
}
