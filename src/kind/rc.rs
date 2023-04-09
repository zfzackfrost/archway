use alloc::rc::{Rc, Weak};

use crate::shared::{AnySharedPointer, SharedPointer};
use crate::weak::AnyWeakPointer;

impl<T: ?Sized> AnySharedPointer<T> for Rc<T> {
    type Weak = Weak<T>;
    fn downgrade(&self) -> Self::Weak {
        Rc::downgrade(self)
    }
}
impl<T: Sized> SharedPointer<T> for Rc<T> {}
impl<T: ?Sized> AnyWeakPointer<T> for Weak<T> {
    type Shared = Rc<T>;
    fn upgrade(&self) -> Option<Self::Shared> {
        Weak::upgrade(self)
    }
}
