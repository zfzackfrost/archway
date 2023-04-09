use core::borrow::Borrow;
use core::ops::Deref;

use crate::weak::AnyWeakPointer;

/// Trait for a shared pointer, supporting unsized pointee types
pub trait AnySharedPointer<T: ?Sized>: AsRef<T> + Borrow<T> + Clone + Deref<Target = T> {
    /// Corresponding weak pointer type
    type Weak: AnyWeakPointer<T>;

    /// Return `self` downgraded to a weak pointer.
    fn downgrade(&self) -> Self::Weak;
}

/// Trait for a shared pointer, supporting sized pointee types  only and construction via the trait [`From`]
pub trait SharedPointer<T: Sized>: AnySharedPointer<T> + From<T> {}
