//===================================================================================================================================================================================//
//
//  /$$$$$$$$                                   /$$$$$$              /$$     /$$                    
// |__  $$__/                                  /$$__  $$            | $$    |__/                    
//    | $$  /$$$$$$   /$$$$$$   /$$$$$$       | $$  \ $$  /$$$$$$  /$$$$$$   /$$  /$$$$$$  /$$$$$$$ 
//    | $$ /$$__  $$ /$$__  $$ /$$__  $$      | $$  | $$ /$$__  $$|_  $$_/  | $$ /$$__  $$| $$__  $$
//    | $$| $$  \__/| $$$$$$$$| $$$$$$$$      | $$  | $$| $$  \ $$  | $$    | $$| $$  \ $$| $$  \ $$
//    | $$| $$      | $$_____/| $$_____/      | $$  | $$| $$  | $$  | $$ /$$| $$| $$  | $$| $$  | $$
//    | $$| $$      |  $$$$$$$|  $$$$$$$      |  $$$$$$/| $$$$$$$/  |  $$$$/| $$|  $$$$$$/| $$  | $$
//    |__/|__/       \_______/ \_______/       \______/ | $$____/    \___/  |__/ \______/ |__/  |__/
//                                                      | $$                                        
//                                                      | $$                                        
//                                                      |__/                                        
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Implements a counterpart to the standard library's `Option<T>` which enables for option-like
//! dynamics with error reporting that is tied into the current node tree and logger.
//! 

use std::mem;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;
use std::ops::{ Deref, DerefMut };
use std::option::{ Iter, IterMut };

use crate::traits::node_tree::NodeTree;
use super::rid::RID;
use super::logger::Log;
use super::tree_result::TreeResult;


/// A simple counterpart to the standard library's `Option`, which has a few extra features such as
/// logging panics or undesired behaviours to the log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TreeOption<'a, T> {
    tree:   *mut dyn NodeTree,
    owner:  RID,
    object: Option<T>,
    p_life: PhantomData<&'a ()>
}

impl <'a, T> TreeOption<'a, T> {
    
    /// Creates a new `TreeOption<T>`.
    ///
    /// # Safety
    /// This is marked unsafe because it is unknown if the passed in tree pointer is valid.
    /// Instead of constructing this type yourself, it is best to only use it when a node function
    /// constructs it for you.
    #[inline]
    pub const unsafe fn new(tree: *mut dyn NodeTree, owner: RID, object: Option<T>) -> Self {
        TreeOption { tree, owner, object, p_life: PhantomData }
    }

    /// Converts this to an `Option<T>` type.
    #[inline]
    pub fn to_option(self) -> Option<T> {
        self.object
    }

    /// Returns `true` if the option is a `Some` value.
    #[inline]
    pub const fn is_some(&self) -> bool {
        self.object.is_some()
    }

    /// Returns `true` if the option is a `Some` and the value inside of it matches a predicate.
    #[inline]
    pub fn is_some_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
        self.object.as_ref().is_some_and(f)
    }

    /// Returns `true` if the option is a `None` value.
    #[inline]
    pub const fn is_none(&self) -> bool {
        self.object.is_none()
    }

    ///// Returns `true` if the option is a `None` or the value inside of it matches a predicate.
    //#[inline]
    //pub fn is_none_or(&self, f: impl FnOnce(&T) -> bool) -> bool {
    //    self.object.as_ref().is_none_or(f)
    //}
    
    /// Converts from `&Option<T>` to `Option<&T>`.
    #[inline]
    pub const fn as_ref(&self) -> TreeOption<&T> {
        TreeOption { tree: self.tree, owner: self.owner, object: self.object.as_ref(), p_life: self.p_life }
    }

    /// Converts from `&mut Option<T>` to `Option<&mut T>`.
    #[inline]
    pub fn as_mut(&mut self) -> TreeOption<&mut T> {
        TreeOption { tree: self.tree, owner: self.owner, object: self.object.as_mut(), p_life: self.p_life }
    }

    /// Returns a slice of the contained value, if any. If this is `None`, an
    /// empty slice is returned. This can be useful to have a single type of
    /// iterator over an `Option` or slice.
    ///
    /// Note: Should you have an `Option<&T>` and wish to get a slice of `T`,
    /// you can unpack it via `opt.map_or(&[], std::slice::from_ref)`.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.object.as_slice()
    }

    /// Returns a mutable slice of the contained value, if any. If this is
    /// `None`, an empty slice is returned. This can be useful to have a
    /// single type of iterator over an `Option` or slice.
    ///
    /// Note: Should you have an `Option<&mut T>` instead of a
    /// `&mut Option<T>`, which this method takes, you can obtain a mutable
    /// slice via `opt.map_or(&mut [], std::slice::from_mut)`.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.object.as_mut_slice()
    }

    /// Returns the contained `Some` value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a `None` with a custom panic message provided by
    /// `msg`.
    #[inline]
    pub fn expect(self, msg: &str) -> T {
        match self.object {
            Some(object) => object,
            None         => self.fail(msg)
        }
    }

    /// Returns the contained `Some` value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the `None`
    /// case explicitly, or call `unwrap_or`, `unwrap_or_else`, or
    /// `unwrap_or_default`.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals `None`.
    #[inline(always)]
    pub fn unwrap(self) -> T {
        match self.object {
            Some(object) => object,
            None         => self.fail("called `TreeOption::unwrap()` on a `None` value")
        }
    }

    /// Returns the contained `Some` value or a provided default.
    ///
    /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`unwrap_or_else`],
    /// which is lazily evaluated.
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self.object {
            Some(object) => object,
            None         => default
        }
    }

    /// Returns the contained `Some` value or computes it from a closure.
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.object.unwrap_or_else(f)
    }

    /// Returns the contained `Some` value or a default.
    ///
    /// Consumes the `self` argument then, if `Some`, returns the contained
    /// value, otherwise if `None`, returns the default value for that
    /// type.
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        self.object.unwrap_or_default()
    }

    /// Returns the contained `Some` value, consuming the `self` value,
    /// without checking that the value is not `None`.
    ///
    /// # Safety
    ///
    /// Calling this method on `None` is *undefined behavior*.
    #[inline]
    pub unsafe fn unwrap_unchecked(self) -> T {
        match self.object {
            Some(object) => object,
            None         => unsafe { unreachable_unchecked() },
        }
    }

    /// Maps an `TreeOption<T>` to `TreeOption<U>` by applying a function to a contained value (if `Some`) or returns `None` (if `None`).
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> TreeOption<'a, U> {
        TreeOption { tree: self.tree, owner: self.owner, object: self.object.map(f), p_life: self.p_life }
    }

    /// Calls a function with a reference to the contained value if `Some`.
    ///
    /// Returns the original option.
    #[inline]
    pub fn inspect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Some(ref x) = self.object {
            f(x);
        }
        self
    }

    /// Returns the provided default result (if none),
    /// or applies a function to the contained value (if any).
    ///
    /// Arguments passed to `map_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`map_or_else`],
    /// which is lazily evaluated.
    #[inline]
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        self.object.map_or(default, f)
    }

    /// Computes a default function result (if none), or
    /// applies a different function to the contained value (if any).
    #[inline]
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    { self.object.map_or_else(default, f) }

    /// Transforms the `TreeOption<T>` into a `TreeResult<T, E>`, mapping `Some(v)` to
    /// `Ok(v)` and `None` to `Err(err)`.
    ///
    /// Arguments passed to `ok_or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use `ok_or_else`, which is
    /// lazily evaluated.
    #[inline]
    pub fn ok_or<E>(self, err: E) -> TreeResult<'a, T, E> {
        unsafe {
            TreeResult::new(self.tree, self.owner, self.object.ok_or(err))
        }
    }

    /// Transforms the `TreeOption<T>` into a `TreeResult<T, E>`, mapping `Some(v)` to
    /// `Ok(v)` and `None` to `Err(err())`.
    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> TreeResult<'a, T, E> {
        unsafe {
            TreeResult::new(self.tree, self.owner, self.object.ok_or_else(err))
        }
    }

    /// Converts from `TreeOption<T>` (or `&TreeOption<T>`) to `TreeOption<&T::Target>`.
    ///
    /// Leaves the original `Option` in-place, creating a new one with a reference
    /// to the original one, additionally coercing the contents via `Deref`.
    #[inline]
    pub fn as_deref(&self) -> TreeOption<&T::Target>
    where
        T: Deref,
    { self.as_another(self.object.as_deref()) }

    /// Converts from `TreeOption<T>` (or `&mut TreeOption<T>`) to `TreeOption<&mut T::Target>`.
    ///
    /// Leaves the original `Option` in-place, creating a new one containing a mutable reference to
    /// the inner type's `Deref::Target` type.
    #[inline]
    pub fn as_deref_mut(&mut self) -> TreeOption<&mut T::Target>
    where
        T: DerefMut,
    { TreeOption { tree: self.tree, owner: self.owner, object: self.object.as_deref_mut(), p_life: self.p_life } }

    /// Returns an iterator over the possibly contained value.
    #[inline]
    pub /*const*/ fn iter(&self) -> Iter<'_, T> {
        self.object.iter()
    }
    
    /// Returns a mutable iterator over the possibly contained value.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.object.iter_mut()
    }

    /// Returns `None` if the option is `None`, otherwise returns `optb`.
    ///
    /// Arguments passed to `and` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use `and_then`, which is
    /// lazily evaluated.
    #[inline]
    pub fn and<'b, U>(self, optb: TreeOption<'b, U>) -> TreeOption<'a, U> {
        match self.object {
            Some(_) => self.transfer_owner(optb),
            None    => self.as_none()
        }
    }

    /// Returns `None` if the option is `None`, otherwise calls `f` with the
    /// wrapped value and returns the result.
    ///
    /// Some languages call this operation flatmap.
    #[inline]
    #[doc(alias = "flatmap")]
    pub fn and_then<'b, U, F: FnOnce(T) -> TreeOption<'b, U>>(self, f: F) -> TreeOption<'a, U> {
        match self.object {
            Some(x) => TreeOption { tree: self.tree, owner: self.owner, object: f(x).to_option(), p_life: self.p_life },
            None    => TreeOption { tree: self.tree, owner: self.owner, object: None,             p_life: self.p_life }
        }
    }

    /// Returns `None` if the option is `None`, otherwise calls `predicate`
    /// with the wrapped value and returns:
    ///
    /// - `Some(t)` if `predicate` returns `true` (where `t` is the wrapped
    ///   value), and
    /// - `None` if `predicate` returns `false`.
    ///
    /// This function works similar to `Iterator::filter()`. You can imagine
    /// the `TreeOption<T>` being an iterator over one or zero elements. `filter()`
    /// lets you decide which elements to keep.
    #[inline]
    pub fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Self {
        if let Some(ref x) = self.object {
            if predicate(x) {
                return self;
            }
        }
        self.as_none()
    }

    /// Returns the option if it contains a value, otherwise returns `optb`.
    ///
    /// Arguments passed to `or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use `or_else`, which is
    /// lazily evaluated.
    pub fn or<'b>(self, optb: TreeOption<'b, T>) -> TreeOption<'a, T> {
        match self.object {
            object @ Some(_) => TreeOption { tree: self.tree, owner: self.owner, object, p_life: self.p_life },
            None             => self.transfer_owner(optb)
        }
    }

    /// Returns the option if it contains a value, otherwise calls `f` and
    /// returns the result.
    #[inline]
    pub fn or_else<'b, F: FnOnce() -> TreeOption<'b, T>>(self, f: F) -> TreeOption<'a, T> {
        match self.object {
            object @ Some(_) => TreeOption { tree: self.tree, owner: self.owner, object, p_life: self.p_life },
            None             => self.transfer_owner(f()),
        }
    }

    /// Returns `Some` if exactly one of `self`, `optb` is `Some`, otherwise returns `None`.
    #[inline]
    pub fn xor<'b>(self, optb: TreeOption<'b, T>) -> TreeOption<'a, T> {
        match (self.object, optb.object) {
            (opta @ Some(_), None) => TreeOption { tree: self.tree, owner: self.owner, object: opta, p_life: self.p_life },
            (None, optb @ Some(_)) => TreeOption { tree: self.tree, owner: self.owner, object: optb, p_life: self.p_life },
            _                      => TreeOption { tree: self.tree, owner: self.owner, object: None, p_life: self.p_life }
        }
    }

    /// Inserts `value` into the option, then returns a mutable reference to it.
    ///
    /// If the option already contains a value, the old value is dropped.
    ///
    /// See also `Option::get_or_insert`, which doesn't update the value if
    /// the option already contains `Some`.
    #[inline]
    pub fn insert(&mut self, value: T) -> &mut T {
        self.object = Some(value);
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    /// Inserts `value` into the option if it is `None`, then
    /// returns a mutable reference to the contained value.
    ///
    /// See also `Option::insert`, which updates the value even if
    /// the option already contains `Some`.
    #[inline]
    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        if self.object.is_none() {
            self.object = Some(value);
        }
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    /// Inserts the default value into the option if it is `None`, then
    /// returns a mutable reference to the contained value.
    #[inline]
    pub fn get_or_insert_default(&mut self) -> &mut T
    where
        T: Default,
    { self.get_or_insert_with(T::default) }

    /// Inserts a value computed from `f` into the option if it is `None`,
    /// then returns a mutable reference to the contained value.
    #[inline]
    pub fn get_or_insert_with<F: FnOnce() -> T>(&mut self, f: F) -> &mut T {
        if self.object.is_none() {
            self.object = Some(f());
        }
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    /// Takes the value out of the option, leaving a `None` in its place.
    #[inline]
    pub fn take(&mut self) -> TreeOption<T> {
        TreeOption {
            owner:  self.owner,
            tree:   self.tree,
            object: self.object.take(),
            p_life: self.p_life
        }
    }

    /// Takes the value out of the option, but only if the predicate evaluates to
    /// `true` on a mutable reference to the value.
    ///
    /// In other words, replaces `self` with `None` if the predicate returns `true`.
    /// This method operates similar to `Option::take` but conditional.
    #[inline]
    pub fn take_if<P: FnOnce(&mut T) -> bool>(&mut self, predicate: P) -> TreeOption<T> {
        TreeOption {
            tree:   self.tree,
            owner:  self.owner,
            object: if self.as_mut().map_or(false, predicate) { self.object.take() } else { None },
            p_life: self.p_life
        }
    }
    
    /// Replaces the actual value in the option by the value given in parameter,
    /// returning the old value if present,
    /// leaving a `Some` in its place without deinitializing either one.
    #[inline]
    pub fn replace(&mut self, value: T) -> Option<T> {
        mem::replace(&mut self.object, Some(value))
    }

    /// Zips `self` with another `TreeOption`.
    ///
    /// If `self` is `Some(s)` and `other` is `Some(o)`, this method returns `Some((s, o))`.
    /// Otherwise, `None` is returned.
    pub fn zip<'b, U>(self, other: TreeOption<'b, U>) -> TreeOption<'a, (T, U)> {
        TreeOption {
            tree:   self.tree,
            owner:  self.owner,
            object: self.object.zip(other.object),
            p_life: self.p_life
        }
    }

    /*/// Zips `self` and another `Option` with function `f`.
    ///
    /// If `self` is `Some(s)` and `other` is `Some(o)`, this method returns `Some(f(s, o))`.
    /// Otherwise, `None` is returned.
    pub fn zip_with<U, R, F: FnOnce(T, U) -> R>(self, other: TreeOption<'a, U>, f: F) -> TreeOption<'a, R> {
        TreeOption {
            tree:   self.tree,
            owner:  self.owner,
            object: self.object.zip_with(other.object, f),
            p_life: self.p_life
        }
    }*/

    /// Gives another `TreeOption` the same node owner as this one.
    pub fn transfer_owner<'b, U>(&self, other: TreeOption<'b, U>) -> TreeOption<'a, U> {
        TreeOption { tree: self.tree, owner: self.owner, object: other.object, p_life: self.p_life}
    }

    /// Marks a failed operation with a panic on the log, and panics the main thread.
    fn fail(&self, msg: &str) -> ! {
        unsafe { (*self.tree).get_node(self.owner).unwrap_unchecked() }.post(Log::Panic(msg));
        println!("\n[RUST TRACE]");
        panic!();
    }
    
    /// Creates a `TreeOption` with all of the calling `TreeOption`'s metadata attached and the
    /// given optional item as its value.
    fn as_another<U>(&self, object: Option<U>) -> TreeOption<'a, U> {
        TreeOption { tree: self.tree, owner: self.owner, object, p_life: self.p_life }
    }
    
    /*/// Creates a `TreeOption` with all of the calling `TreeOption`'s metadata attached and the
    /// given item as its value.
    fn as_some<U>(&self, object: U) -> TreeOption<'a, U> {
        TreeOption { tree: self.tree, owner: self.owner, object: Some(object), p_life: self.p_life }
    }*/
    
    /// Creates a `TreeOption` with all of the calling `TreeOption`'s metadata attached and `None`
    /// as its value.
    fn as_none<U>(&self) -> TreeOption<'a, U> {
        TreeOption { tree: self.tree, owner: self.owner, object: None, p_life: self.p_life }
    }
}

impl <'a, T, U> TreeOption<'a, (T, U)> {
    
    /// Unzips an option containing a tuple of two options.
    ///
    /// If `self` is `Some((a, b))` this method returns `(Some(a), Some(b))`.
    /// Otherwise, `(None, None)` is returned.
    #[inline]
    pub fn unzip(self) -> (TreeOption<'a, T>, TreeOption<'a, U>) {
        match self.object {
            Some((a, b)) => (TreeOption {
                tree:   self.tree,
                owner:  self.owner,
                object: Some(a),
                p_life: self.p_life
            }, TreeOption {
                tree:   self.tree,
                owner:  self.owner,
                object: Some(b),
                p_life: self.p_life
            }),
            None => (self.as_none(), self.as_none()),
        }
    }
}

impl <'a, T> TreeOption<'a, &T> {

    /// Maps an `TreeOption<&T>` to an `TreeOption<T>` by copying the contents of the
    /// option.
    pub fn copied(self) -> TreeOption<'a, T>
    where
        T: Copy,
    { self.map(|x| *x) }

    /// Maps an `TreeOption<&T>` to an `TreeOption<T>` by cloning the contents of the
    /// option.
    pub fn cloned(self) -> TreeOption<'a, T>
    where
        T: Clone,
    { self.map(|x| x.to_owned()) }
}

impl <'a, T> TreeOption<'a, &mut T> {

    /// Maps an `TreeOption<&mut T>` to an `TreeOption<T>` by copying the contents of the
    /// option.
    pub fn copied(self) -> TreeOption<'a, T>
    where
        T: Copy,
    { self.map(|x| *x) }

    /// Maps an `TreeOption<&mut T>` to an `TreeOption<T>` by cloning the contents of the
    /// option.
    pub fn cloned(self) -> TreeOption<'a, T>
    where
        T: Clone,
    { self.map(|x| x.to_owned()) }
}

impl <'a, 'b, T, E> TreeOption<'a, TreeResult<'b, T, E>> {
    
    /// Transposes an `TreeOption` of a `TreeResult` into a `TreeResult` of an `TreeOption`.
    ///
    /// `None` will be mapped to <code>Ok\(None)</code>.
    /// <code>Some\(Ok\(\_))</code> and <code>Some\(Err\(\_))</code> will be mapped to
    /// <code>Ok\(Some\(\_))</code> and <code>Err\(\_)</code>.
    #[inline]
    pub fn transpose(self) -> TreeResult<'a, TreeOption<'a, T>, E> {
        match self.object {
            Some(inner) => {
                match inner.to_result() {
                    Ok(x)  => unsafe { TreeResult::new(self.tree, self.owner, Ok(TreeOption::new(self.tree, self.owner, Some(x)))) },
                    Err(e) => unsafe { TreeResult::new(self.tree, self.owner, Err(e)) },
                }
            },
            None => unsafe { TreeResult::new(self.tree, self.owner, Ok(TreeOption::new(self.tree, self.owner, None))) }
        }
    }
}

impl <'a, 'b, T> TreeOption<'a, TreeOption<'b, T>> {

    /// Converts from `TreeOption<TreeOption<T>>` to `TreeOption<T>`.
    pub fn flatten(self) -> TreeOption<'a, T> {
        match self.object {
            Some(object) => TreeOption { tree: self.tree, owner: self.owner, object: object.to_option(), p_life: self.p_life },
            None         => self.as_none()
        }
    }
}
