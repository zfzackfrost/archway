use alloc::sync::{Arc, Weak};

use crate::kind::PointerKind;
use crate::shared::{AnySharedPointer, SharedPointer};
use crate::weak::AnyWeakPointer;

impl<T: ?Sized> AnySharedPointer<T> for Arc<T> {
    type Weak = Weak<T>;
    fn downgrade(&self) -> Self::Weak {
        Arc::downgrade(self)
    }
}
impl<T: Sized> SharedPointer<T> for Arc<T> {}
impl<T: ?Sized> AnyWeakPointer<T> for Weak<T> {
    type Shared = Arc<T>;
    fn upgrade(&self) -> Option<Self::Shared> {
        Weak::upgrade(self)
    }
}

/// [`PointerKind`] implementor for [`Arc`].
#[derive(Debug, Copy, Clone)]
pub struct ArcKind;

impl PointerKind for ArcKind {
    type Shared<T> = Arc<T>;
    type Weak<T> = Weak<T>;
}
