//===================================================================================================================================================================================//
//
//  /$$   /$$              /$$ /$$$$$$$$ /$$          /$$$$$$$           /$$             /$$                        
// | $$  | $$             /$$/|__  $$__/|  $$        | $$__  $$         |__/            | $$                        
// | $$  | $$  /$$$$$$   /$$/    | $$    \  $$       | $$  \ $$ /$$$$$$  /$$ /$$$$$$$  /$$$$$$    /$$$$$$   /$$$$$$ 
// | $$$$$$$$ /$$__  $$ /$$/     | $$     \  $$      | $$$$$$$//$$__  $$| $$| $$__  $$|_  $$_/   /$$__  $$ /$$__  $$
// | $$__  $$| $$  \ $$|  $$     | $$      /$$/      | $$____/| $$  \ $$| $$| $$  \ $$  | $$    | $$$$$$$$| $$  \__/
// | $$  | $$| $$  | $$ \  $$    | $$     /$$/       | $$     | $$  | $$| $$| $$  | $$  | $$ /$$| $$_____/| $$      
// | $$  | $$| $$$$$$$/  \  $$   | $$    /$$/        | $$     |  $$$$$$/| $$| $$  | $$  |  $$$$/|  $$$$$$$| $$      
// |__/  |__/| $$____/    \__/   |__/   |__/         |__/      \______/ |__/|__/  |__/   \___/   \_______/|__/      
//           | $$                                                                                                   
//           | $$                                                                                                   
//           |__/                                                                                                   
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! A shared mutability pointer with implicit cloning.
//! Hp<T> stands for High-Pointer, which derives its name from its intended function: to chip away
//! at some of the low-level unsafe operations that arise from shared mutability.
//! 
//! # Safety
//! To prevent data races, this smart pointer does not derive from Sync + Send, which means that it
//! *cannot* be shared through threads.
//! This should not be used outside of the context of the NodeTree since the NodeTree is designed
//! with shared mutability in mind.
//!
//! # Note
//! These pointers do not destroy the data they have allocated automatically, rather they must be destroyed manually.
//!

use std::alloc::{ Allocator, Global };
use std::marker::{ PhantomData, Unsize };
use std::rc::Rc;
use std::ops::{ CoerceUnsized, Deref, DerefMut, DispatchFromDyn };


/// Please see the module's documentation for more information on the `Hp<T>` smart pointer.
pub struct Hp<T: ?Sized, A: Allocator = Global>(*const T, PhantomData<A>);


// The implementation for Hp<T>.
impl <T: Sized> Hp<T> {
    
    /// Creates a new instance of the smart `Hp<T>` pointer from owned data.
    pub fn new(owned: T) -> Hp<T> {
        
        // Allocated using Rc<T>, then destroy the Rc<T> without deallocating the data.
        // We then save the pointer to the allocation itself.
        let initial_rc: Rc<T>    = Rc::new(owned);
        let pointer:    *const T = Rc::into_raw(initial_rc);   // Use Rc::from_raw to destroy stuff
        
        Hp(pointer, PhantomData)
    }
}

impl <T: ?Sized> Hp<T> {

    /// Destroys the pointer and its allocated memory.
    /// # Safety
    /// It is YOUR responsibility to ensure that all connected references are destroyed alongside
    /// this pointer.
    pub unsafe fn destroy(self) -> () {
        let _: Rc<T> = Rc::from_raw(self.0);   // Let Rc<T> handle the destruction for us.
    }
}


// Required for arbitrary self behaviour in both structs and traits.
impl <T: ?Sized, A: Allocator> Deref for Hp<T, A> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            Rc::increment_strong_count(self.0);   // We do this not to invalidate the data that we are to get a reference from right when we drop the Rc<T>.
            let rc: Rc<T> = Rc::from_raw(self.0);
            &*(rc.as_ref() as *const T)
        }
    }
}


// Provides a safe interface for the unsafe operation of provided multiple mutable references.
impl <T: ?Sized, A: Allocator> DerefMut for Hp<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            Rc::increment_strong_count(self.0);   // We do this not to invalidate the data that we are to get a reference from right when we drop the Rc<T>.
            let mut rc: Rc<T> = Rc::from_raw(self.0);
            &mut *(Rc::get_mut_unchecked(&mut rc) as *mut T)
        }
    }
}


// Required for arbitrary self behaviour in trait objects.
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Hp<U>> for Hp<T>
where
    T: Unsize<U>,
{}


// Required for arbitrary self behaviour in traits.
impl<T, U, A> CoerceUnsized<Hp<U, A>> for Hp<T, A>
where
    T: Unsize<U> + ?Sized,
    A: Allocator,
    U: ?Sized
{}


impl <T: ?Sized, A: Allocator> Clone for Hp<T, A> {
    fn clone(&self) -> Self {
        Hp(self.0.clone(), self.1.clone())
    }
}

impl <T: ?Sized, A: Allocator> Copy for Hp<T, A> {}

impl <T: ?Sized + std::fmt::Debug, A: Allocator> std::fmt::Debug for Hp<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            Rc::increment_strong_count(self.0);   // We do this not to invalidate the data that we are to access.
            let rc: Rc<T> = Rc::from_raw(self.0);
            f.write_str(&format!("Hp {{ {:?} }}", rc.as_ref()))
        }
    }
}
