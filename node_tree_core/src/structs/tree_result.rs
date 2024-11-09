//===================================================================================================================================================================================//
//
//  /$$$$$$$$                                  /$$$$$$$                                /$$   /$$    
// |__  $$__/                                 | $$__  $$                              | $$  | $$    
//    | $$  /$$$$$$   /$$$$$$   /$$$$$$       | $$  \ $$  /$$$$$$   /$$$$$$$ /$$   /$$| $$ /$$$$$$  
//    | $$ /$$__  $$ /$$__  $$ /$$__  $$      | $$$$$$$/ /$$__  $$ /$$_____/| $$  | $$| $$|_  $$_/  
//    | $$| $$  \__/| $$$$$$$$| $$$$$$$$      | $$__  $$| $$$$$$$$|  $$$$$$ | $$  | $$| $$  | $$    
//    | $$| $$      | $$_____/| $$_____/      | $$  \ $$| $$_____/ \____  $$| $$  | $$| $$  | $$ /$$
//    | $$| $$      |  $$$$$$$|  $$$$$$$      | $$  | $$|  $$$$$$$ /$$$$$$$/|  $$$$$$/| $$  |  $$$$/
//    |__/|__/       \_______/ \_______/      |__/  |__/ \_______/|_______/  \______/ |__/   \___/  
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Implements a counterpart to the standard library's `Result<T, E>` which enables for result-like
//! dynamics with error reporting that is tied into the current node tree and logger.
//! Currently does not support error types other than `String.
//! 

use std::fmt;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;
use std::ops::{ Deref, DerefMut };
use std::result::{ Iter, IterMut };

use crate::traits::node_tree::NodeTree;
use super::rid::RID;
use super::logger::Log;
use super::tree_option::TreeOption;


/// A simple counterpart to the standard library's `Result`, which has a few extra features such as
/// logging panics or undesired behaviours to the log.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
pub struct TreeResult<'a, T> {
    tree:   *mut dyn NodeTree,
    owner:  RID,
    object: Result<T, String>,
    p_life: PhantomData<&'a ()>
}

impl <'a, T> TreeResult<'a, T> {
    
    /// Creates a new `TreeResult<T>`.
    ///
    /// # Safety
    /// This is marked unsafe because it is unknown if the passed in tree pointer is valid.
    /// Instead of constructing this type yourself, it is best to only use it when a node function
    /// constructs it for you.
    #[inline]
    pub const unsafe fn new(tree: *mut dyn NodeTree, owner: RID, object: Result<T, String>) -> Self {
        TreeResult { owner, tree, object, p_life: PhantomData }
    }

    /// Converts this to a `Result<T, String>` type.
    #[inline]
    pub fn to_result(self) -> Result<T, String> {
        self.object
    }

    
    /// Converts this to an `Option<T>` type.
    #[inline]
    pub fn to_option(self) -> Option<T> {
        self.object.ok()
    }
    
    /// Returns `true` if the result is `Ok`.
    #[inline]
    pub const fn is_ok(&self) -> bool {
        self.object.is_ok()
    }

    /// Returns `true` if the result is `Ok` and the value inside of it matches a predicate.
    #[inline]
    pub fn is_ok_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
        self.object.as_ref().is_ok_and(f)
    }
    
    /// Returns `true` if the result is `Err`.
    #[inline]
    pub const fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Returns `true` if the result is `Err` and the value inside of it matches a predicate.
    #[inline]
    pub fn is_err_and(&self, f: impl FnOnce(&String) -> bool) -> bool {
        self.object.as_ref().is_err_and(f)
    }

    /// Converts from `TreeResult<T>` to `TreeOption<T>`.
    ///
    /// Converts `self` into an `TreeOption<T>`, consuming `self`,
    /// and discarding the error, if any.
    #[inline]
    pub fn ok(self) -> TreeOption<'a, T> {
        unsafe {
            TreeOption::new(self.tree, self.owner, self.object.ok())
        }
    }

    /// Converts from `TreeResult<T>` to `TreeOption<String>`.
    ///
    /// Converts `self` into an `TreeOption<String>`, consuming `self`,
    /// and discarding the success value, if any.
    #[inline]
    pub fn err(self) -> TreeOption<'a, String> {
        unsafe {
            TreeOption::new(self.tree, self.owner, self.object.err())
        }
    }

    /// Converts from `&TreeResult<T>` to `TreeResult<&T>`.
    ///
    /// Produces a new `TreeResult`, containing a reference
    /// into the original, leaving the original in place.
    #[inline]
    pub fn as_ref(&self) -> TreeResult<&T> {
        match self.object.as_ref() {
            Ok(object) => TreeResult { tree: self.tree, owner: self.owner, object: Ok(object),           p_life: self.p_life },
            Err(err)   => TreeResult { tree: self.tree, owner: self.owner, object: Err(err.to_string()), p_life: self.p_life }
        }
    }
    

    /// Converts from `&mut TreeResult<T>` to `TreeResult<&mut T>`.
    ///
    /// Produces a new `TreeResult`, containing a reference
    /// into the original, leaving the original in place.
    #[inline]
    pub fn as_mut(&mut self) -> TreeResult<&mut T> {
        match self.object.as_mut() {
            Ok(object) => TreeResult { tree: self.tree, owner: self.owner, object: Ok(object),           p_life: self.p_life },
            Err(err)   => TreeResult { tree: self.tree, owner: self.owner, object: Err(err.to_string()), p_life: self.p_life }
        }
    }

    // Maps a `TreeResult<T>` to `TreeResult<U>` by applying a function to a
    /// contained `Ok` value, leaving an `Err` value untouched.
    ///
    /// This function can be used to compose the results of two functions.
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> TreeResult<'a, U> {
        TreeResult { tree: self.tree, owner: self.owner, object: self.object.map(op), p_life: self.p_life }
    }

    /// Returns the provided default (if `Err`), or
    /// applies a function to the contained value (if `Ok`).
    ///
    /// Arguments passed to `map_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use `map_or_else`,
    /// which is lazily evaluated.
    #[inline]
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        self.object.map_or(default, f)
    }

    /// Maps a `TreeResult<T>` to `U` by applying fallback function `default` to
    /// a contained `Err` value, or function `f` to a contained `Ok` value.
    ///
    /// This function can be used to unpack a successful result
    /// while handling an error.
    #[inline]
    pub fn map_or_else<U, D: FnOnce(String) -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        self.object.map_or_else(default, f)
    }

    /// Maps a `TreeResult<T>` to `TreeResult<T>` by applying a function to a
    /// contained `Err` value, leaving an `Ok` value untouched.
    ///
    /// This function can be used to pass through a successful result while handling
    /// an error.
    #[inline]
    pub fn map_err<O: FnOnce(String) -> String>(self, op: O) -> TreeResult<'a, T> {
        TreeResult { tree: self.tree, owner: self.owner, object: self.object.map_err(op), p_life: self.p_life }
    }
    
    /// Calls a function with a reference to the contained value if `Ok`.
    ///
    /// Returns the original result.
    #[inline]
    pub fn inspect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Ok(ref t) = self.object {
            f(t);
        }
        self
    }

    /// Calls a function with a reference to the contained value if `Err`.
    ///
    /// Returns the original result.
    #[inline]
    pub fn inspect_err<F: FnOnce(&String)>(self, f: F) -> Self {
        if let Err(ref e) = self.object {
            f(e);
        }
        self
    }

    /// Converts from `TreeResult<T>` (or `&TreeResult<T>`) to `TreeResult<&<T as Deref>::Target>`.
    ///
    /// Coerces the `Ok` variant of the original `Result` via `Deref`
    /// and returns the new `Result`.
    #[inline]
    pub fn as_deref(&'a self) -> TreeResult<'a, &T::Target>
    where
        T: Deref,
    { self.as_ref().map(|t| t.deref()) }

    /// Converts from `TreeResult<T>` (or `&mut TreeResult<T>`) to `TreeResult<&mut <T as DerefMut>::Target>`.
    ///
    /// Coerces the `Ok` variant of the original `Result` via `DerefMut`
    /// and returns the new `Result`.
    #[inline]
    pub fn as_deref_mut(&'a mut self) -> TreeResult<'a, &mut T::Target>
    where
        T: DerefMut,
    { self.as_mut().map(|t| t.deref_mut()) }

    /// Returns an iterator over the possibly contained value.
    ///
    /// The iterator yields one value if the result is `Result::Ok`, otherwise none.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.object.iter()
    }

    /// Returns a mutable iterator over the possibly contained value.
    ///
    /// The iterator yields one value if the result is `Result::Ok`, otherwise none.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.object.iter_mut()
    }

    /// Returns the contained `Ok` value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the `Err`
    /// case explicitly, or call `unwrap_or`, `unwrap_or_else`, or
    /// `unwrap_or_default`.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Err`, with a panic message including the
    /// passed message, and the content of the `Err`.
    pub fn expect(self, msg: &str) -> T {
        match self.object {
            Ok(object)   => object,
            Err(ref err) => self.fail(msg, err)
        }
    }

    /// Returns the contained `Ok` value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the `Err`
    /// case explicitly, or call `unwrap_or`, `unwrap_or_else`, or
    /// `unwrap_or_default`.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Err`, with a panic message provided by the
    /// `Err`'s value.
    pub fn unwrap(self) -> T {
        match self.object {
            Ok(object)   => object,
            Err(ref err) => self.fail("called `TreeResult::unwrap()` on an `Err` value", err)
        }
    }

    /// Returns the contained `Ok` value or a default
    ///
    /// Consumes the `self` argument then, if `Ok`, returns the contained
    /// value, otherwise if `Err`, returns the default value for that
    /// type.
    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        self.object.unwrap_or_default()
    }

    /// Returns the contained `Err` value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Ok`, with a panic message including the
    /// passed message, and the content of the `Ok`.
    #[inline]
    pub fn expect_err(self, msg: &str) -> String
    where
        T: fmt::Debug,
    {
        match self.object {
            Ok(ref t) => self.fail(msg, &format!("{t:?}")),
            Err(e)    => e
        }
    }

    /// Returns the contained `Err` value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Ok`, with a custom panic message provided
    /// by the `Ok`'s value.
    #[inline]
    pub fn unwrap_err(self) -> String
    where
        T: fmt::Debug,
    {
        match self.object {
            Ok(ref t) => self.fail("called `TreeResult::unwrap_err()` on an `Ok` value", &format!("{t:?}")),
            Err(e)    => e
        }
    }
    
    /// Returns `res` if the result is `Ok`, otherwise returns the `Err` value of `self`.
    ///
    /// Arguments passed to `and` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use `and_then`, which is
    /// lazily evaluated.
    #[inline]
    pub fn and<'b, U>(self, res: TreeResult<'b, U>) -> TreeResult<'a, U> {
        match self.object {
            Ok(_)  => self.transfer_owner(res),
            Err(e) => TreeResult { tree: self.tree, owner: self.owner, object: Err(e), p_life: self.p_life },
        }
    }

    /// Calls `op` if the result is `Ok`, otherwise returns the `Err` value of `self`.
    ///
    /// This function can be used for control flow based on `Result` values.
    #[inline]
    pub fn and_then<U, F: FnOnce(T) -> TreeResult<'a, U>>(self, op: F) -> TreeResult<'a, U> {
        match self.object {
            Ok(t)  => op(t),
            Err(e) => TreeResult { tree: self.tree, owner: self.owner, object: Err(e), p_life: self.p_life }
        }
    }

    /// Returns `res` if the result is `Err`, otherwise returns the `Ok` value of `self`.
    ///
    /// Arguments passed to `or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`or_else`], which is
    /// lazily evaluated.
    #[inline]
    pub fn or<'b>(self, res: TreeResult<'b, T>) -> TreeResult<'a, T> {
        match self.object {
            Ok(_)  => self,
            Err(_) => self.transfer_owner(res)
        }
    }

    /// Calls `op` if the result is `Err`, otherwise returns the `Ok` value of `self`.
    ///
    /// This function can be used for control flow based on result values.
    #[inline]
    pub fn or_else<'b, F, O: FnOnce(&str) -> TreeResult<'b, T>>(self, op: O) -> TreeResult<'a, T> {
        match self.object {
            Ok(_)      => self,
            Err(ref e) => self.transfer_owner(op(e))
        }
    }

    /// Returns the contained `Ok` value or a provided default.
    ///
    /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use `unwrap_or_else`,
    /// which is lazily evaluated.
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self.object {
            Ok(t)  => t,
            Err(_) => default
        }
    }

    /// Returns the contained `Ok` value or computes it from a closure.
    #[inline]
    pub fn unwrap_or_else<F: FnOnce(&str) -> T>(self, op: F) -> T {
        match self.object {
            Ok(t)  => t,
            Err(e) => op(&e),
        }
    }

    /// Returns the contained `Ok` value, consuming the `self` value,
    /// without checking that the value is not an `Err`.
    ///
    /// # Safety
    ///
    /// Calling this method on an `Err` is *undefined behavior*.
    #[inline]
    pub unsafe fn unwrap_unchecked(self) -> T {
        match self.object {
            Ok(object) => object,
            Err(_)     => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the contained `Err` value, consuming the `self` value,
    /// without checking that the value is not an `Ok`.
    ///
    /// # Safety
    ///
    /// Calling this method on an `Ok` is *undefined behavior*.
    #[inline]
    pub unsafe fn unwrap_err_unchecked(self) -> String {
        match self.object {
            Ok(_)    => unsafe { unreachable_unchecked() },
            Err(err) => err
        }
    }

    /// Gives another `TreeResult` the same node owner as this one.
    pub fn transfer_owner<'b, U>(&self, other: TreeResult<'b, U>) -> TreeResult<'a, U> {
        TreeResult { tree: self.tree, owner: self.owner, object: other.object, p_life: self.p_life}
    }
    
    /// Marks a failed operation with a panic on the log, and panics the main thread.
    fn fail(&self, msg: &str, error: &str) -> ! {
        unsafe { (*self.tree).get_node(self.owner).unwrap_unchecked() }.post(Log::Panic(&format!("{msg}: {error}")));
        println!("\n[RUST TRACE]");
        panic!();
    }
}

impl <'a, T> TreeResult<'a, &T> {
    
    /// Maps a `TreeResult<&T>` to a `TreeResult<T>` by copying the contents of the
    /// `Ok` part.
    #[inline]
    pub fn copied(self) -> TreeResult<'a, T>
    where
        T: Copy,
    { self.map(|&t| t) }

    /// Maps a `TreeResult<&T>` to a `TreeResult<T>` by cloning the contents of the
    /// `Ok` part.
    #[inline]
    pub fn cloned(self) -> TreeResult<'a, T>
    where
        T: Clone,
    { self.map(|t| t.clone()) }
}

impl <'a, T> TreeResult<'a, &mut T> {
    
    /// Maps a `TreeResult<&mut T>` to a `TreeResult<T>` by copying the contents of the
    /// `Ok` part.
    #[inline]
    pub fn copied(self) -> TreeResult<'a, T>
    where
        T: Copy,
    { self.map(|&mut t| t) }

    /// Maps a `TreeResult<&mut T>` to a `TreeResult<T>` by cloning the contents of the
    /// `Ok` part.
    #[inline]
    pub fn cloned(self) -> TreeResult<'a, T>
    where
        T: Clone,
    { self.map(|t| t.clone()) }
}

impl <'a, 'b, T> TreeResult<'a, TreeOption<'b, T>> {

    /// Transposes a `TreeResult` of an `TreeOption` into an `TreeOption` of a `TreeResult`.
    ///
    /// `Ok(None)` will be mapped to `None`.
    /// `Ok(Some(_))` and `Err(_)` will be mapped to `Some(Ok(_))` and `Some(Err(_))`.
    #[inline]
    pub fn transpose(self) -> TreeOption<'a, TreeResult<'a, T>> {
        match self.object {
            Ok(inner) => {
                match inner.to_option() {
                    Some(x) => unsafe { TreeOption::new(self.tree, self.owner, Some(TreeResult::new(self.tree, self.owner, Ok(x)))) },
                    None    => unsafe { TreeOption::new(self.tree, self.owner, None) },
                }
            },
            Err(e) => unsafe { TreeOption::new(self.tree, self.owner, Some(TreeResult::new(self.tree, self.owner, Err(e)))) }
        }
    }
}

impl <'a, 'b, T> TreeResult<'a, TreeResult<'b, T>> {
    
    /// Converts from `TreeResult<TreeResult<T>>` to `TreeResult<T, E>`
    #[inline]
    pub fn flatten(self) -> TreeResult<'a, T> {
        match self.object {
            Ok(inner) => TreeResult { tree: self.tree, owner: self.owner, object: inner.to_result(), p_life: self.p_life },
            Err(err)  => TreeResult { tree: self.tree, owner: self.owner, object: Err(err),          p_life: self.p_life }
        }
    }
}
