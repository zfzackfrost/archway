use alloc::rc::{Rc, Weak};

use crate::kind::PointerKind;
use crate::shared::AnySharedPointer;
use crate::weak::AnyWeakPointer;

impl<T: Sized> AnySharedPointer<T> for Rc<T> {
    type Weak = Weak<T>;
    fn downgrade(&self) -> Self::Weak {
        Rc::downgrade(self)
    }
}
impl<T: Sized> AnyWeakPointer<T> for Weak<T> {
    type Shared = Rc<T>;
    fn upgrade(&self) -> Option<Self::Shared> {
        Weak::upgrade(self)
    }
}

/// [`PointerKind`] implementor for [`Rc`].
#[derive(Debug, Copy, Clone)]
pub struct RcKind;

impl PointerKind for RcKind {
    type Shared<T> = Rc<T>;
    type Weak<T> = Weak<T>;
}
