mod arc;
mod rc;

pub use arc::*;
pub use rc::*;

use crate::shared::SharedPointer;
use crate::weak::AnyWeakPointer;

/// Trait that bundles generic-associated-types for corresponding shared and weak pointers.
pub trait PointerKind {
    /// Shared pointer type, with pointee `T`
    type Shared<T>: SharedPointer<T>;
    /// Weak pointer type, with pointee `T`
    type Weak<T>: AnyWeakPointer<T>;
}
