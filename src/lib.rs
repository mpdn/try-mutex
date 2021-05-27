//! Provides a simple mutex that does not support blocking or poisoning, but is
//! faster and simpler than the mutex in stdlib.

use std::cell::UnsafeCell;
use std::convert::From;
use std::fmt;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};

/// A mutual exclusion primitive that does not support blocking or poisoning.
/// This results in a simpler and faster implementation.
pub struct TryMutex<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool,
}

impl<T> TryMutex<T> {
    /// Create a new mutex in unlocked state.
    #[inline]
    pub const fn new(t: T) -> Self {
        TryMutex {
            data: UnsafeCell::new(t),
            locked: AtomicBool::new(false),
        }
    }

    /// Attemps to acquire a lock on this mutex. If this mutex is currently
    /// locked, `None` is returned. Otherwise a RAII guard is returned. The lock
    /// will be unlocked when the guard is dropped.
    #[inline]
    pub fn try_lock(&self) -> Option<TryMutexGuard<'_, T>> {
        self.locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .ok()
            .map(|_| TryMutexGuard {
                lock: self,
                notsend: PhantomData,
            })
    }

    /// Consumes this mutex, returning the underlying data.
    #[inline]
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }

    /// Retrieve a mutable reference to the underlying data. Since this mutably
    /// borrows the mutex, no actual locking needs to take place.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
}

impl<T: Default> Default for TryMutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Debug> Debug for TryMutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(guard) = self.try_lock() {
            f.debug_struct("TryMutex").field("data", &*guard).finish()
        } else {
            struct LockedPlaceholder;
            impl fmt::Debug for LockedPlaceholder {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str("<locked>")
                }
            }

            f.debug_struct("TryMutex")
                .field("data", &LockedPlaceholder)
                .finish()
        }
    }
}

/// A RAII scoped lock on a `TryMutex`. When this this structure is dropped, the
/// mutex will be unlocked.
pub struct TryMutexGuard<'a, T: 'a> {
    lock: &'a TryMutex<T>,
    notsend: PhantomData<*mut T>,
}

impl<'a, T> Deref for TryMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for TryMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for TryMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<'a, T: Debug> Debug for TryMutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TryMutexGuard")
            .field("data", &*self)
            .finish()
    }
}

impl<'a, T: Display> Display for TryMutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T> UnwindSafe for TryMutex<T> {}
impl<T> RefUnwindSafe for TryMutex<T> {}
unsafe impl<T: Send> Send for TryMutex<T> {}
unsafe impl<T: Send> Sync for TryMutex<T> {}
unsafe impl<'a, T: Sync> Sync for TryMutexGuard<'a, T> {}
unsafe impl<'a, T: Send> Send for TryMutexGuard<'a, T> {}

impl<T> From<T> for TryMutex<T> {
    fn from(t: T) -> Self {
        TryMutex::new(t)
    }
}
