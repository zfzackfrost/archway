use alloc::sync::{Arc, Weak};

use crate::shared::AnySharedPointer;
use crate::weak::AnyWeakPointer;

impl<T> AnySharedPointer<T> for Arc<T> {
    type Weak = Weak<T>;
    fn downgrade(&self) -> Self::Weak {
        Arc::downgrade(self)
    }
}
impl<T> AnyWeakPointer<T> for Weak<T> {
    type Shared = Arc<T>;
    fn upgrade(&self) -> Option<Self::Shared> {
        Weak::upgrade(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eq_shared_value<T: Clone + PartialEq + core::fmt::Debug, P: AnySharedPointer<T>>(value: T, shared: P) {
        assert_eq!(value, shared.as_ref().clone())
    }
    fn eq_weak_value<T: Clone + PartialEq + core::fmt::Debug, P: AnyWeakPointer<T>>(value: T, weak: P) {
        let shared = weak.upgrade().unwrap();
        assert_eq!(value, shared.as_ref().clone())
    }

    #[test]
    fn test_shared() {
        use alloc::sync::Arc;
        let shared = Arc::new(42i32);
        eq_shared_value(42i32, shared);
    }
    #[test]
    fn test_weak() {
        use alloc::sync::Arc;
        let shared = Arc::new(42i32);
        eq_weak_value(42i32, shared.downgrade());
    }
}
