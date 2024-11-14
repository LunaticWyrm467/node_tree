//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                 /$$$$$$$$ /$$           /$$       /$$          
// | $$$ | $$                | $$                | $$_____/|__/          | $$      | $$          
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$       | $$       /$$  /$$$$$$ | $$  /$$$$$$$  /$$$$$$$
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$      | $$$$$   | $$ /$$__  $$| $$ /$$__  $$ /$$_____/
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$      | $$__/   | $$| $$$$$$$$| $$| $$  | $$|  $$$$$$ 
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/      | $$      | $$| $$_____/| $$| $$  | $$ \____  $$
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$      | $$      | $$|  $$$$$$$| $$|  $$$$$$$ /$$$$$$$/
// |__/  \__/ \______/  \_______/ \_______/      |__/      |__/ \_______/|__/ \_______/|_______/ 
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Provides node field types which helps encapsulate some essential functionality.
//! 

use std::ops::{ Deref, DerefMut, self };
use std::mem;

use crate::traits::exportable::{ Voidable, Exportable };


/*
 * Field
 */


/// Provides useful functionality such as a possible `Null` state which occurs after loading.
/// This is only used for non-exported, non-default, non-unique fields.
#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Field<T> {
    Valid(T),
    Void
}

impl <T> Field<T> {
    
    /// Creates a new `Field<T>`.
    pub fn new(item: T) -> Self {
        Self::Valid(item)
    }

    /// Writes a valid state to the node field.
    /// Returns the old valid state if there is one.
    pub fn write_valid(&mut self, item: T) -> Option<T> {
        let old_item: Option<T> = self.take();
        *self = Self::Valid(item);
        old_item
    }

    /// Checks if the container's item is valid.
    pub fn is_reachable(&self) -> bool {
        match self {
            Self::Valid(_) => true,
            Self::Void     => false
        }
    }

    /// Checks if the container's item has been voided.
    pub fn is_void(&self) -> bool {
        match self {
            Self::Valid(_) => false,
            Self::Void     => true
        }
    }

    /// Attempts to take the innner value, returning `None` if the internal state is void.
    /// The original container will be left voided.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        match self {
            Self::Valid(_) => {
                let out: Self = mem::replace(self, Self::Void);
                Some(out.force_take())
            },
            Self::Void => None
        }
    }

    /// Forcefully consumes the `Field<T>`, returning `T`.
    ///
    /// # Panics
    /// Panics if `T` is unreacheable!
    pub fn force_take(self) -> T {
        match self {
            Self::Valid(t) => t,
            Self::Void     => panic!("Attempted to utilize a voided node field")
        }
    }
}

impl <T: Clone> Clone for Field<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Valid(item) => Self::Valid(item.clone()),
            Self::Void        => Self::Void,
        }
    }
}

impl <T> Deref for Field<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Valid(ref valid) => valid,
            Self::Void             => panic!("Attempted to utilize a voided node field")
        }
    }
}

impl <T> DerefMut for Field<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Valid(ref mut valid) => valid,
            Self::Void                 => panic!("Attempted to utilize a voided node field")
        }
    }
}

impl <T> Voidable for Field<T> {
    fn void() -> Self {
        Self::Void
    }
}

impl <T> Exportable for Field<T> {
    unsafe fn is_ghost_export(&self) -> bool { true }
    
    fn to_value(&self) -> toml_edit::Value {
        unimplemented!()
    }

    fn from_value(_value: toml_edit::Value) -> Option<Self> where Self: Sized {
        unimplemented!()
    }
}

impl <T: ops::AddAssign> ops::AddAssign for Field<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self.deref_mut() += rhs.force_take();
    }
}

impl <T: ops::SubAssign> ops::SubAssign for Field<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self.deref_mut() -= rhs.force_take();
    }
}

impl <T: ops::MulAssign> ops::MulAssign for Field<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self.deref_mut() *= rhs.force_take();
    }
}

impl <T: ops::DivAssign> ops::DivAssign for Field<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self.deref_mut() /= rhs.force_take();
    }
}

impl <T: ops::RemAssign> ops::RemAssign for Field<T> {
    fn rem_assign(&mut self, rhs: Self) {
        *self.deref_mut() %= rhs.force_take();
    }
}

impl <T: ops::BitOrAssign> ops::BitOrAssign for Field<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self.deref_mut() |= rhs.force_take();
    }
}

impl <T: ops::BitXorAssign> ops::BitXorAssign for Field<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self.deref_mut() ^= rhs.force_take();
    }
}

impl <T: ops::BitAndAssign> ops::BitAndAssign for Field<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self.deref_mut() &= rhs.force_take();
    }
}

impl <T: ops::ShlAssign> ops::ShlAssign for Field<T> {
    fn shl_assign(&mut self, rhs: Self) {
        *self.deref_mut() <<= rhs.force_take();
    }
}

impl <T: ops::ShrAssign> ops::ShrAssign for Field<T> {
    fn shr_assign(&mut self, rhs: Self) {
        *self.deref_mut() >>= rhs.force_take();
    }
}

impl <T: ops::AddAssign> ops::AddAssign<T> for Field<T> {
    fn add_assign(&mut self, rhs: T) {
        *self.deref_mut() += rhs;
    }
}

impl <T: ops::SubAssign> ops::SubAssign<T> for Field<T> {
    fn sub_assign(&mut self, rhs: T) {
        *self.deref_mut() -= rhs;
    }
}

impl <T: ops::MulAssign> ops::MulAssign<T> for Field<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self.deref_mut() *= rhs;
    }
}

impl <T: ops::DivAssign> ops::DivAssign<T> for Field<T> {
    fn div_assign(&mut self, rhs: T) {
        *self.deref_mut() /= rhs;
    }
}

impl <T: ops::RemAssign> ops::RemAssign<T> for Field<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self.deref_mut() %= rhs;
    }
}

impl <T: ops::BitOrAssign> ops::BitOrAssign<T> for Field<T> {
    fn bitor_assign(&mut self, rhs: T) {
        *self.deref_mut() |= rhs;
    }
}

impl <T: ops::BitXorAssign> ops::BitXorAssign<T> for Field<T> {
    fn bitxor_assign(&mut self, rhs: T) {
        *self.deref_mut() ^= rhs;
    }
}

impl <T: ops::BitAndAssign> ops::BitAndAssign<T> for Field<T> {
    fn bitand_assign(&mut self, rhs: T) {
        *self.deref_mut() &= rhs;
    }
}

impl <T: ops::ShlAssign> ops::ShlAssign<T> for Field<T> {
    fn shl_assign(&mut self, rhs: T) {
        *self.deref_mut() <<= rhs;
    }
}

impl <T: ops::ShrAssign> ops::ShrAssign<T> for Field<T> {
    fn shr_assign(&mut self, rhs: T) {
        *self.deref_mut() >>= rhs;
    }
}


/*
 * Exportable
 *      Field
 */


/// Provides useful functionality for exportable fields.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExportableField<T: Exportable>(T);

impl <T: Exportable> ExportableField<T> {
    
    /// Creates a new `ExportableField<T>`.
    pub fn new(item: T) -> Self {
        Self(item)
    }
}

impl <T: Exportable> Deref for ExportableField<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T: Exportable> DerefMut for ExportableField<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl <T: Exportable + Default> Voidable for ExportableField<T> {
    fn void() -> Self {
        Self::default()
    }
}

impl <T: Exportable> Exportable for ExportableField<T> {
    fn to_value(&self) -> toml_edit::Value {
        self.0.to_value()
    }

    fn from_value(value: toml_edit::Value) -> Option<Self> where Self: Sized {
        Some(Self::new(T::from_value(value)?))
    }
}

impl <T: ops::AddAssign + Exportable> ops::AddAssign for ExportableField<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self.deref_mut() += rhs.0;
    }
}

impl <T: ops::SubAssign + Exportable> ops::SubAssign for ExportableField<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self.deref_mut() -= rhs.0;
    }
}

impl <T: ops::MulAssign + Exportable> ops::MulAssign for ExportableField<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self.deref_mut() *= rhs.0;
    }
}

impl <T: ops::DivAssign + Exportable> ops::DivAssign for ExportableField<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self.deref_mut() /= rhs.0;
    }
}

impl <T: ops::RemAssign + Exportable> ops::RemAssign for ExportableField<T> {
    fn rem_assign(&mut self, rhs: Self) {
        *self.deref_mut() %= rhs.0;
    }
}

impl <T: ops::BitOrAssign + Exportable> ops::BitOrAssign for ExportableField<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self.deref_mut() |= rhs.0;
    }
}

impl <T: ops::BitXorAssign + Exportable> ops::BitXorAssign for ExportableField<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self.deref_mut() ^= rhs.0;
    }
}

impl <T: ops::BitAndAssign + Exportable> ops::BitAndAssign for ExportableField<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self.deref_mut() &= rhs.0;
    }
}

impl <T: ops::ShlAssign + Exportable> ops::ShlAssign for ExportableField<T> {
    fn shl_assign(&mut self, rhs: Self) {
        *self.deref_mut() <<= rhs.0;
    }
}

impl <T: ops::ShrAssign + Exportable> ops::ShrAssign for ExportableField<T> {
    fn shr_assign(&mut self, rhs: Self) {
        *self.deref_mut() >>= rhs.0;
    }
}

impl <T: ops::AddAssign + Exportable> ops::AddAssign<T> for ExportableField<T> {
    fn add_assign(&mut self, rhs: T) {
        *self.deref_mut() += rhs;
    }
}

impl <T: ops::SubAssign + Exportable> ops::SubAssign<T> for ExportableField<T> {
    fn sub_assign(&mut self, rhs: T) {
        *self.deref_mut() -= rhs;
    }
}

impl <T: ops::MulAssign + Exportable> ops::MulAssign<T> for ExportableField<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self.deref_mut() *= rhs;
    }
}

impl <T: ops::DivAssign + Exportable> ops::DivAssign<T> for ExportableField<T> {
    fn div_assign(&mut self, rhs: T) {
        *self.deref_mut() /= rhs;
    }
}

impl <T: ops::RemAssign + Exportable> ops::RemAssign<T> for ExportableField<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self.deref_mut() %= rhs;
    }
}

impl <T: ops::BitOrAssign + Exportable> ops::BitOrAssign<T> for ExportableField<T> {
    fn bitor_assign(&mut self, rhs: T) {
        *self.deref_mut() |= rhs;
    }
}

impl <T: ops::BitXorAssign + Exportable> ops::BitXorAssign<T> for ExportableField<T> {
    fn bitxor_assign(&mut self, rhs: T) {
        *self.deref_mut() ^= rhs;
    }
}

impl <T: ops::BitAndAssign + Exportable> ops::BitAndAssign<T> for ExportableField<T> {
    fn bitand_assign(&mut self, rhs: T) {
        *self.deref_mut() &= rhs;
    }
}

impl <T: ops::ShlAssign + Exportable> ops::ShlAssign<T> for ExportableField<T> {
    fn shl_assign(&mut self, rhs: T) {
        *self.deref_mut() <<= rhs;
    }
}

impl <T: ops::ShrAssign + Exportable> ops::ShrAssign<T> for ExportableField<T> {
    fn shr_assign(&mut self, rhs: T) {
        *self.deref_mut() >>= rhs;
    }
}


/*
 * Unique
 *      Field
 */


/// Provides useful functionality such as a possible `Null` state which occurs after loading or
/// cloning.
/// This is only used for unique fields.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UniqueField<T> {
    Valid(T),
    Void
}

impl <T> UniqueField<T> {
    
    /// Creates a new `UniqueField<T>`.
    pub fn new(item: T) -> Self {
        Self::Valid(item)
    }

    /// Writes a valid state to the node field.
    /// Returns the old valid state if there is one.
    pub fn write_valid(&mut self, item: T) -> Option<T> {
        let old_item: Option<T> = self.take();
        *self = Self::Valid(item);
        old_item
    }

    /// Checks if the container's item is valid.
    pub fn is_reachable(&self) -> bool {
        match self {
            Self::Valid(_) => true,
            Self::Void     => false
        }
    }

    /// Checks if the container's item has been voided.
    pub fn is_void(&self) -> bool {
        match self {
            Self::Valid(_) => false,
            Self::Void     => true
        }
    }

    /// Attempts to take the innner value, returning `None` if the internal state is void.
    /// The original container will be left voided.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        match self {
            Self::Valid(_) => {
                let out: Self = mem::replace(self, Self::Void);
                Some(out.force_take())
            },
            Self::Void => None
        }
    }

    /// Forcefully consumes the `UniqueField<T>`, returning `T`.
    ///
    /// # Panics
    /// Panics if `T` is unreacheable!
    pub fn force_take(self) -> T {
        match self {
            Self::Valid(t) => t,
            Self::Void     => panic!("Attempted to utilize a voided node field")
        }
    }
}

impl <T> Clone for UniqueField<T> {
    fn clone(&self) -> Self {
        Self::Void
    }
}

impl <T> Deref for UniqueField<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Valid(ref valid) => valid,
            Self::Void             => panic!("Attempted to utilize a voided node field")
        }
    }
}

impl <T> DerefMut for UniqueField<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Valid(ref mut valid) => valid,
            Self::Void                 => panic!("Attempted to utilize a voided node field")
        }
    }
}

impl <T> Voidable for UniqueField<T> {
    fn void() -> Self {
        Self::Void
    }
}

impl <T> Exportable for UniqueField<T> {
    unsafe fn is_ghost_export(&self) -> bool { true }
    
    fn to_value(&self) -> toml_edit::Value {
        unimplemented!()
    }

    fn from_value(_value: toml_edit::Value) -> Option<Self> where Self: Sized {
        unimplemented!()
    }
}

impl <T: ops::AddAssign> ops::AddAssign for UniqueField<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self.deref_mut() += rhs.force_take();
    }
}

impl <T: ops::SubAssign> ops::SubAssign for UniqueField<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self.deref_mut() -= rhs.force_take();
    }
}

impl <T: ops::MulAssign> ops::MulAssign for UniqueField<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self.deref_mut() *= rhs.force_take();
    }
}

impl <T: ops::DivAssign> ops::DivAssign for UniqueField<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self.deref_mut() /= rhs.force_take();
    }
}

impl <T: ops::RemAssign> ops::RemAssign for UniqueField<T> {
    fn rem_assign(&mut self, rhs: Self) {
        *self.deref_mut() %= rhs.force_take();
    }
}

impl <T: ops::BitOrAssign> ops::BitOrAssign for UniqueField<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self.deref_mut() |= rhs.force_take();
    }
}

impl <T: ops::BitXorAssign> ops::BitXorAssign for UniqueField<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self.deref_mut() ^= rhs.force_take();
    }
}

impl <T: ops::BitAndAssign> ops::BitAndAssign for UniqueField<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self.deref_mut() &= rhs.force_take();
    }
}

impl <T: ops::ShlAssign> ops::ShlAssign for UniqueField<T> {
    fn shl_assign(&mut self, rhs: Self) {
        *self.deref_mut() <<= rhs.force_take();
    }
}

impl <T: ops::ShrAssign> ops::ShrAssign for UniqueField<T> {
    fn shr_assign(&mut self, rhs: Self) {
        *self.deref_mut() >>= rhs.force_take();
    }
}

impl <T: ops::AddAssign> ops::AddAssign<T> for UniqueField<T> {
    fn add_assign(&mut self, rhs: T) {
        *self.deref_mut() += rhs;
    }
}

impl <T: ops::SubAssign> ops::SubAssign<T> for UniqueField<T> {
    fn sub_assign(&mut self, rhs: T) {
        *self.deref_mut() -= rhs;
    }
}

impl <T: ops::MulAssign> ops::MulAssign<T> for UniqueField<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self.deref_mut() *= rhs;
    }
}

impl <T: ops::DivAssign> ops::DivAssign<T> for UniqueField<T> {
    fn div_assign(&mut self, rhs: T) {
        *self.deref_mut() /= rhs;
    }
}

impl <T: ops::RemAssign> ops::RemAssign<T> for UniqueField<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self.deref_mut() %= rhs;
    }
}

impl <T: ops::BitOrAssign> ops::BitOrAssign<T> for UniqueField<T> {
    fn bitor_assign(&mut self, rhs: T) {
        *self.deref_mut() |= rhs;
    }
}

impl <T: ops::BitXorAssign> ops::BitXorAssign<T> for UniqueField<T> {
    fn bitxor_assign(&mut self, rhs: T) {
        *self.deref_mut() ^= rhs;
    }
}

impl <T: ops::BitAndAssign> ops::BitAndAssign<T> for UniqueField<T> {
    fn bitand_assign(&mut self, rhs: T) {
        *self.deref_mut() &= rhs;
    }
}

impl <T: ops::ShlAssign> ops::ShlAssign<T> for UniqueField<T> {
    fn shl_assign(&mut self, rhs: T) {
        *self.deref_mut() <<= rhs;
    }
}

impl <T: ops::ShrAssign> ops::ShrAssign<T> for UniqueField<T> {
    fn shr_assign(&mut self, rhs: T) {
        *self.deref_mut() >>= rhs;
    }
}


/*
 * Default
 *      Field
 */


/// Provides useful functionality for default-attributed fields.
/// This is only used for non-exported, default, non-unique fields.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefaultField<T: Default>(T);

impl <T: Default> DefaultField<T> {
    
    /// Creates a new `DefaultField<T>`.
    pub fn new(item: T) -> Self {
        Self(item)
    }
}

impl <T: Default> Deref for DefaultField<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T: Default> DerefMut for DefaultField<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl <T: Default> Voidable for DefaultField<T> {
    fn void() -> Self {
        Self::default()
    }
}

impl <T: Default> Exportable for DefaultField<T> {
    unsafe fn is_ghost_export(&self) -> bool { true }
    
    fn to_value(&self) -> toml_edit::Value {
        unimplemented!()
    }

    fn from_value(_value: toml_edit::Value) -> Option<Self> where Self: Sized {
        unimplemented!()
    }
}

impl <T: ops::AddAssign + Default> ops::AddAssign for DefaultField<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self.deref_mut() += rhs.0;
    }
}

impl <T: ops::SubAssign + Default> ops::SubAssign for DefaultField<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self.deref_mut() -= rhs.0;
    }
}

impl <T: ops::MulAssign + Default> ops::MulAssign for DefaultField<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self.deref_mut() *= rhs.0;
    }
}

impl <T: ops::DivAssign + Default> ops::DivAssign for DefaultField<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self.deref_mut() /= rhs.0;
    }
}

impl <T: ops::RemAssign + Default> ops::RemAssign for DefaultField<T> {
    fn rem_assign(&mut self, rhs: Self) {
        *self.deref_mut() %= rhs.0;
    }
}

impl <T: ops::BitOrAssign + Default> ops::BitOrAssign for DefaultField<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self.deref_mut() |= rhs.0;
    }
}

impl <T: ops::BitXorAssign + Default> ops::BitXorAssign for DefaultField<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self.deref_mut() ^= rhs.0;
    }
}

impl <T: ops::BitAndAssign + Default> ops::BitAndAssign for DefaultField<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self.deref_mut() &= rhs.0;
    }
}

impl <T: ops::ShlAssign + Default> ops::ShlAssign for DefaultField<T> {
    fn shl_assign(&mut self, rhs: Self) {
        *self.deref_mut() <<= rhs.0;
    }
}

impl <T: ops::ShrAssign + Default> ops::ShrAssign for DefaultField<T> {
    fn shr_assign(&mut self, rhs: Self) {
        *self.deref_mut() >>= rhs.0;
    }
}

impl <T: ops::AddAssign + Default> ops::AddAssign<T> for DefaultField<T> {
    fn add_assign(&mut self, rhs: T) {
        *self.deref_mut() += rhs;
    }
}

impl <T: ops::SubAssign + Default> ops::SubAssign<T> for DefaultField<T> {
    fn sub_assign(&mut self, rhs: T) {
        *self.deref_mut() -= rhs;
    }
}

impl <T: ops::MulAssign + Default> ops::MulAssign<T> for DefaultField<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self.deref_mut() *= rhs;
    }
}

impl <T: ops::DivAssign + Default> ops::DivAssign<T> for DefaultField<T> {
    fn div_assign(&mut self, rhs: T) {
        *self.deref_mut() /= rhs;
    }
}

impl <T: ops::RemAssign + Default> ops::RemAssign<T> for DefaultField<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self.deref_mut() %= rhs;
    }
}

impl <T: ops::BitOrAssign + Default> ops::BitOrAssign<T> for DefaultField<T> {
    fn bitor_assign(&mut self, rhs: T) {
        *self.deref_mut() |= rhs;
    }
}

impl <T: ops::BitXorAssign + Default> ops::BitXorAssign<T> for DefaultField<T> {
    fn bitxor_assign(&mut self, rhs: T) {
        *self.deref_mut() ^= rhs;
    }
}

impl <T: ops::BitAndAssign + Default> ops::BitAndAssign<T> for DefaultField<T> {
    fn bitand_assign(&mut self, rhs: T) {
        *self.deref_mut() &= rhs;
    }
}

impl <T: ops::ShlAssign + Default> ops::ShlAssign<T> for DefaultField<T> {
    fn shl_assign(&mut self, rhs: T) {
        *self.deref_mut() <<= rhs;
    }
}

impl <T: ops::ShrAssign + Default> ops::ShrAssign<T> for DefaultField<T> {
    fn shr_assign(&mut self, rhs: T) {
        *self.deref_mut() >>= rhs;
    }
}
