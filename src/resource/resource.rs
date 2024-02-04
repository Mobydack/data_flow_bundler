use std::fmt::Display;

pub enum ResourceError<E> {
    ResourceIsNotAllowed(E),
}

/// Base resource definition.
/// Each concrete resource should implementation resource operations
pub trait Resource<E>: Display {
    fn is_resource_allowed(&self) -> Result<bool, ResourceError<E>>;
}