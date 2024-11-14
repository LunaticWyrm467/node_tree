use std::ops::Deref;

use crate::traits::exportable::Exportable;
use crate::structs::node_field::{ Field, ExportableField, UniqueField, DefaultField };



/// Used for function arguments that should be flexible and take in types in both their raw form
/// and their `NodeField` forms.
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
