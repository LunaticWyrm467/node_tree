use std::ops::Deref;

use crate::traits::exportable::Exportable;
use crate::structs::node_field::{ Field, ExportableField, UniqueField, DefaultField };


/*
 * Element
 *      Trait
 */


/// Used for function arguments that should be flexible and take in types in both their raw form and their `NodeField` forms.
pub trait Element<T> {
    
    /// Converts the element into its inferred type.
    fn as_inner(&self) -> &T;
}

impl <T> Element<T> for T {
    fn as_inner(&self) -> &T {
        self
    }
}

impl <T> Element<T> for Field<T> {
    fn as_inner(&self) -> &T {
        self.deref()
    }
}

impl <T: Exportable> Element<T> for ExportableField<T> {
    fn as_inner(&self) -> &T {
        self.deref()
    }
}

impl <T> Element<T> for UniqueField<T> {
    fn as_inner(&self) -> &T {
        self.deref()
    }
}

impl <T: Default> Element<T> for DefaultField<T> {
    fn as_inner(&self) -> &T {
        self.deref()
    }
}


/*
 * As Element
 *      Trait
 */


/// Used for the simple unambiguous conversion between a type and a type wrapped by a field.
mod private {
    pub trait Sealed {}
    impl <T> Sealed for T {}
}

pub trait AsElement<F>: private::Sealed {

    /// Wraps a type with a field wrapper.
    fn wrap(self) -> F;
}

impl <T> AsElement<Field<T>> for T {
    fn wrap(self) -> Field<T> {
        Field::Valid(self)
    }
}

impl <T: Exportable> AsElement<ExportableField<T>> for T {
    fn wrap(self) -> ExportableField<T> {
        ExportableField::new(self)
    }
}

impl <T> AsElement<UniqueField<T>> for T {
    fn wrap(self) -> UniqueField<T> {
        UniqueField::Valid(self)
    }
}

impl <T: Default> AsElement<DefaultField<T>> for T {
    fn wrap(self) -> DefaultField<T> {
        DefaultField::new(self)
    }
}