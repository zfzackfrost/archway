mod arc;
mod rc;

pub use arc::*;
pub use rc::*;

use crate::shared::{AnySharedPointer, Downgraded};
use crate::weak::{AnyWeakPointer, Upgraded};

/// Trait that bundles generic-associated-types for corresponding shared and weak pointers.
pub trait PointerKind {
    /// Shared pointer type, with pointee `T`
    type Shared<T>: AnySharedPointer<T>;
    /// Weak pointer type, with pointee `T`
    type Weak<T>: AnyWeakPointer<T>;
}

/// Return type for the `downgrade` function on `Weak<T>` of a the [`PointerKind`] type `P`
pub type PointerKindDowngraded<P, T> = Downgraded<<P as PointerKind>::Shared<T>, T>;

/// Return type for the `upgrade` function on `Shared<T>` of a the [`PointerKind`] type `P`
pub type PointerKindUpgraded<P, T> = Upgraded<<P as PointerKind>::Weak<T>, T>;
