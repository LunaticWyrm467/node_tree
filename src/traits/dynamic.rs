use std::any::Any;

pub trait Dynamic: Any {
    
    /// Returns this object as an Any trait object, allowing for type downcasting.
    fn to_any(&self) -> &dyn Any;
}
