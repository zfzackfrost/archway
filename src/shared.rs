use core::borrow::Borrow;
use core::ops::Deref;

use crate::weak::AnyWeakPointer;

/// Trait for a shared pointer, supporting sized pointee types only
pub trait AnySharedPointer<T: Sized>: AsRef<T> + Borrow<T> + Clone + Deref<Target = T> + From<T> {
    /// Corresponding weak pointer type
    type Weak: AnyWeakPointer<T>;

    /// Return `self` downgraded to a weak pointer.
    fn downgrade(&self) -> Self::Weak;
}

/// Return type for the `downgrade` function of the [`AnySharedPointer<T>`] type `P`
pub type Downgraded<P, T> = <P as AnySharedPointer<T>>::Weak;
