//===================================================================================================================================================================================//
//
//   /$$$$$$                      /$$           /$$ /$$                     /$$       /$$          
//  /$$__  $$                    |__/          | $$|__/                    | $$      | $$          
// | $$  \__/  /$$$$$$   /$$$$$$  /$$  /$$$$$$ | $$ /$$ /$$$$$$$$  /$$$$$$ | $$$$$$$ | $$  /$$$$$$ 
// |  $$$$$$  /$$__  $$ /$$__  $$| $$ |____  $$| $$| $$|____ /$$/ |____  $$| $$__  $$| $$ /$$__  $$
//  \____  $$| $$$$$$$$| $$  \__/| $$  /$$$$$$$| $$| $$   /$$$$/   /$$$$$$$| $$  \ $$| $$| $$$$$$$$
//  /$$  \ $$| $$_____/| $$      | $$ /$$__  $$| $$| $$  /$$__/   /$$__  $$| $$  | $$| $$| $$_____/
// |  $$$$$$/|  $$$$$$$| $$      | $$|  $$$$$$$| $$| $$ /$$$$$$$$|  $$$$$$$| $$$$$$$/| $$|  $$$$$$$
//  \______/  \_______/|__/      |__/ \_______/|__/|__/|________/ \_______/|_______/ |__/ \_______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Provides the `Serializable` trait, which all types that are to be used in nodes must implement.
//! Implementing a `Serializable` trait is quite simple, with there being only two functions for
//! serializing and deserializing a value.
//! 

use std::{
    collections::HashMap,
    ops::Deref,
    mem
};

use toml::value::Datetime;

use crate::structs::node_path::NodePath;


/// Used for representing types that can be parsed and loaded from `TOML` files, and as such are
/// supported fully via `node_tree`'s saving and loading system.
pub trait Serializable {
    
    /// Converts a type to a toml value.
    fn to_value(&self) -> toml::Value;

    /// Converts a toml value right back to its origin type.
    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized;
}

impl Serializable for () {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(Vec::new())
    }

    fn from_value(_value: toml::Value) -> Option<Self> where Self: Sized {
        Some(())
    }
}

impl Serializable for bool {
    fn to_value(&self) -> toml::Value {
        (*self).into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Boolean(b) => Some(b),
            _                       => None
        }
    }
}

impl Serializable for u8 {
    fn to_value(&self) -> toml::Value {
        (*self as i64).into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Integer(i) => Some(i as u8),
            _                       => None
        }
    }
}
impl Serializable for u16 {
    fn to_value(&self) -> toml::Value {
        (*self as i64).into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Integer(i) => Some(i as u16),
            _                       => None
        }
    }
}
impl Serializable for u32 {
    fn to_value(&self) -> toml::Value {
        (*self as i64).into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Integer(i) => Some(i as u32),
            _                       => None
        }
    }
}
impl Serializable for u64 {
    fn to_value(&self) -> toml::Value {
        (*self as i64).into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Integer(i) => Some(i as u64),
            _                       => None
        }
    }
}
impl Serializable for i8 {
    fn to_value(&self) -> toml::Value {
        (*self as i64).into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Integer(i) => Some(i as i8),
            _                       => None
        }
    }
}
impl Serializable for i16 {
    fn to_value(&self) -> toml::Value {
        (*self as i64).into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Integer(i) => Some(i as i16),
            _                       => None
        }
    }
}
impl Serializable for i32 {
    fn to_value(&self) -> toml::Value {
        (*self as i64).into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Integer(i) => Some(i as i32),
            _                       => None
        }
    }
}
impl Serializable for i64 {
    fn to_value(&self) -> toml::Value {
        (*self as i64).into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Integer(i) => Some(i as i64),
            _                       => None
        }
    }
}

impl Serializable for char {
    fn to_value(&self) -> toml::Value {
        self.to_string().into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::String(c) => {
                if c.len() != 1 {
                    None
                } else {
                    Some(c.chars().collect::<Vec<_>>()[0])
                }
            },
            _ => None
        }
    }
}
impl Serializable for String {
    fn to_value(&self) -> toml::Value {
        self.to_owned().into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::String(s) => Some(s),
            _                      => None
        }
    }
}
impl Serializable for NodePath {
    fn to_value(&self) -> toml::Value {
        self.to_owned().to_string().to_value()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        String::from_value(value).map(|str| NodePath::from_str(&str))
    }
}

impl Serializable for Datetime {
    fn to_value(&self) -> toml::Value {
        toml::Value::Datetime(self.to_owned())
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Datetime(dt) => Some(dt),
            _                         => None
        }
    }
}

impl <T: Serializable> Serializable for Vec<T> {
    fn to_value(&self) -> toml::Value {
        self.iter().map(|v| (v.to_owned()).to_value()).collect::<Vec<toml::Value>>().into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => arr.into_iter().map(|x| T::from_value(x)).collect::<Option<Vec<T>>>(),
            _                       => None
        }
    }
}

impl <V: Serializable> Serializable for HashMap<char, V> {
    fn to_value(&self) -> toml::Value {
        self.iter().map(|(k, v)| (k.to_string(), (v.to_owned()).to_value())).collect::<toml::Table>().into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Table(table) => {
                table.into_iter().map(|(key, value)| {
                    if key.len() != 1 {
                        None
                    } else {
                        match V::from_value(value) {
                            Some(value) => Some((key.chars().collect::<Vec<_>>()[0], value)),
                            None        => None
                        }
                    }
                }).collect::<Option<HashMap<char, V>>>()
            },
            _ => None
        }
    }
}

impl <V: Serializable> Serializable for HashMap<String, V> {
    fn to_value(&self) -> toml::Value {
        self.iter().map(|(k, v)| (k.to_owned(), (v.to_owned()).to_value())).collect::<toml::Table>().into()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Table(table) => {
                table.into_iter()
                    .map(|(key, value)| V::from_value(value).map(|value| (key, value)))
                    .collect::<Option<HashMap<String, V>>>()
            },
            _ => None
        }
    }
}

impl <T: Serializable> Serializable for Box<T> {
    fn to_value(&self) -> toml::Value {
        self.deref().to_value()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        T::from_value(value).map(|x| Box::new(x))
    }
}

impl <T: Serializable> Serializable for std::rc::Rc<T> {
    fn to_value(&self) -> toml::Value {
        self.deref().to_value()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        T::from_value(value).map(|x| std::rc::Rc::new(x))
    }
}

impl <T: Serializable> Serializable for std::sync::Arc<T> {
    fn to_value(&self) -> toml::Value {
        self.deref().to_value()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        T::from_value(value).map(|x| std::sync::Arc::new(x))
    }
}

impl <T: Serializable> Serializable for std::sync::Mutex<T> {
    fn to_value(&self) -> toml::Value {
        self.lock().unwrap_or_else(|err| panic!("Serialization failed: {err}")).to_value()
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        T::from_value(value).map(|x| std::sync::Mutex::new(x))
    }
}

impl <const N: usize, T: Serializable> Serializable for [T; N] {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(self.iter().map(|x| x.to_value()).collect())
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if arr.len() != N {
                    return None;
                }

                let mut rtr_arr: [T; N] = unsafe { mem::zeroed() };
                for (i, element) in arr.into_iter().enumerate() {
                    rtr_arr[i] = T::from_value(element)?;
                }
                Some(rtr_arr)
            },
            _ => None
        }
    }
}

impl <A: Serializable> Serializable for (A,) {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(vec![self.0.to_value()])
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [a] = arr.as_slice() {
                    return Some((A::from_value(a.to_owned())?,))
                }
                None
            },
            _ => None
        }
    }
}
impl <A: Serializable, B: Serializable> Serializable for (A, B) {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(vec![self.0.to_value(), self.1.to_value()])
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [a, b] = arr.as_slice() {
                    return Some((A::from_value(a.to_owned())?, B::from_value(b.to_owned())?))
                }
                None
            },
            _ => None
        }
    }
}
impl <A: Serializable, B: Serializable, C: Serializable> Serializable for (A, B, C) {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(vec![self.0.to_value(), self.1.to_value(), self.2.to_value()])
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [a, b, c] = arr.as_slice() {
                    return Some((A::from_value(a.to_owned())?, B::from_value(b.to_owned())?, C::from_value(c.to_owned())?))
                }
                None
            },
            _ => None
        }
    }
}
impl <A: Serializable, B: Serializable, C: Serializable, D: Serializable> Serializable for (A, B, C, D) {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(vec![self.0.to_value(), self.1.to_value(), self.2.to_value(), self.3.to_value()])
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [a, b, c, d] = arr.as_slice() {
                    return Some((A::from_value(a.to_owned())?, B::from_value(b.to_owned())?, C::from_value(c.to_owned())?, D::from_value(d.to_owned())?))
                }
                None
            },
            _ => None
        }
    }
}
impl <A: Serializable, B: Serializable, C: Serializable, D: Serializable, E: Serializable> Serializable for (A, B, C, D, E) {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(vec![self.0.to_value(), self.1.to_value(), self.2.to_value(), self.3.to_value(), self.4.to_value()])
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [a, b, c, d, e] = arr.as_slice() {
                    return Some((A::from_value(a.to_owned())?, B::from_value(b.to_owned())?, C::from_value(c.to_owned())?, D::from_value(d.to_owned())?, E::from_value(e.to_owned())?))
                }
                None
            },
            _ => None
        }
    }
}
impl <A: Serializable, B: Serializable, C: Serializable, D: Serializable, E: Serializable,
      F: Serializable> Serializable for (A, B, C, D, E, F) {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(vec![
            self.0.to_value(), self.1.to_value(), self.2.to_value(), self.3.to_value(), self.4.to_value(),
            self.5.to_value()
        ])
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [a, b, c, d, e, f] = arr.as_slice() {
                    return Some((
                            A::from_value(a.to_owned())?, B::from_value(b.to_owned())?, C::from_value(c.to_owned())?, D::from_value(d.to_owned())?, E::from_value(e.to_owned())?,
                            F::from_value(f.to_owned())?
                    ))
                }
                None
            },
            _ => None
        }
    }
}
impl <A: Serializable, B: Serializable, C: Serializable, D: Serializable, E: Serializable,
      F: Serializable, G: Serializable> Serializable for (A, B, C, D, E, F, G) {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(vec![
            self.0.to_value(), self.1.to_value(), self.2.to_value(), self.3.to_value(), self.4.to_value(),
            self.5.to_value(), self.6.to_value()
        ])
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [a, b, c, d, e, f, g] = arr.as_slice() {
                    return Some((
                            A::from_value(a.to_owned())?, B::from_value(b.to_owned())?, C::from_value(c.to_owned())?, D::from_value(d.to_owned())?, E::from_value(e.to_owned())?,
                            F::from_value(f.to_owned())?, G::from_value(g.to_owned())?
                    ))
                }
                None
            },
            _ => None
        }
    }
}
impl <A: Serializable, B: Serializable, C: Serializable, D: Serializable, E: Serializable,
      F: Serializable, G: Serializable, H: Serializable> Serializable for (A, B, C, D, E, F, G, H) {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(vec![
            self.0.to_value(), self.1.to_value(), self.2.to_value(), self.3.to_value(), self.4.to_value(),
            self.5.to_value(), self.6.to_value(), self.7.to_value()
        ])
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [a, b, c, d, e, f, g, h] = arr.as_slice() {
                    return Some((
                            A::from_value(a.to_owned())?, B::from_value(b.to_owned())?, C::from_value(c.to_owned())?, D::from_value(d.to_owned())?, E::from_value(e.to_owned())?,
                            F::from_value(f.to_owned())?, G::from_value(g.to_owned())?, H::from_value(h.to_owned())?
                    ))
                }
                None
            },
            _ => None
        }
    }
}
impl <A: Serializable, B: Serializable, C: Serializable, D: Serializable, E: Serializable,
      F: Serializable, G: Serializable, H: Serializable, I: Serializable> Serializable for (A, B, C, D, E, F, G, H, I) {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(vec![
            self.0.to_value(), self.1.to_value(), self.2.to_value(), self.3.to_value(), self.4.to_value(),
            self.5.to_value(), self.6.to_value(), self.7.to_value(), self.8.to_value()
        ])
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [a, b, c, d, e, f, g, h, i] = arr.as_slice() {
                    return Some((
                            A::from_value(a.to_owned())?, B::from_value(b.to_owned())?, C::from_value(c.to_owned())?, D::from_value(d.to_owned())?, E::from_value(e.to_owned())?,
                            F::from_value(f.to_owned())?, G::from_value(g.to_owned())?, H::from_value(h.to_owned())?, I::from_value(i.to_owned())?
                    ))
                }
                None
            },
            _ => None
        }
    }
}
impl <A: Serializable, B: Serializable, C: Serializable, D: Serializable, E: Serializable,
      F: Serializable, G: Serializable, H: Serializable, I: Serializable, J: Serializable> Serializable for (A, B, C, D, E, F, G, H, I, J) {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(vec![
            self.0.to_value(), self.1.to_value(), self.2.to_value(), self.3.to_value(), self.4.to_value(),
            self.5.to_value(), self.6.to_value(), self.7.to_value(), self.8.to_value(), self.9.to_value()
        ])
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [a, b, c, d, e, f, g, h, i, j] = arr.as_slice() {
                    return Some((
                            A::from_value(a.to_owned())?, B::from_value(b.to_owned())?, C::from_value(c.to_owned())?, D::from_value(d.to_owned())?, E::from_value(e.to_owned())?,
                            F::from_value(f.to_owned())?, G::from_value(g.to_owned())?, H::from_value(h.to_owned())?, I::from_value(i.to_owned())?, J::from_value(j.to_owned())?
                    ))
                }
                None
            },
            _ => None
        }
    }
}
