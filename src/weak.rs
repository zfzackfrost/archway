use crate::shared::AnySharedPointer;

/// Trait for a weak pointer
pub trait AnyWeakPointer<T: Sized> {
    /// Corresponding shared pointer type
    type Shared: AnySharedPointer<T>;

    /// Return `self` upgraded to a shared pointer, or `None` if unsuccessful.
    fn upgrade(&self) -> Option<Self::Shared>;
}

/// Unwrapped return type for the `upgrade` function of the [`AnyWeakPointer<T>`] type `P`
pub type Upgraded<P, T> = <P as AnyWeakPointer<T>>::Shared;
