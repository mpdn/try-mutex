///! Provides a simple mutex that does not support blocking or poisoning, but is
///! faster and simpler than the mutex in stdlib.

use std::convert::From;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::marker::PhantomData;
use std::panic::{UnwindSafe, RefUnwindSafe};

/// A mutual exclusion primitive that does not support blocking or poisoning.
/// This results in a simpler and faster implementation.
pub struct TryMutex<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool,
}

/// A RAII scoped lock on a `TryMutex`. When this this structure is dropped, the
/// mutex will be unlocked.
pub struct TryMutexGuard<'a, T: 'a>{
    mutex: &'a TryMutex<T>,
    notsend: PhantomData<*mut T>,
}

impl<'a, T> Deref for TryMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> DerefMut for TryMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T> Drop for TryMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release);
    }
}

impl<T> TryMutex<T> {
    /// Create a new mutex in unlocked state
    #[inline]
    pub fn new(t: T) -> TryMutex<T> {
        TryMutex {
            data: UnsafeCell::new(t),
            locked: AtomicBool::new(false),
        }
    }

    /// Attemps to acquire a lock on this mutex. If this mutex is currently
    /// locked, `None` is returned. Otherwise a RAII guard is returned. The lock
    /// will be unlocked when the guard is dropped.
    #[inline]
    pub fn try_lock(&self) -> Option<TryMutexGuard<T>> {
        if self.locked.compare_and_swap(false, true, Ordering::Acquire) {
            None
        } else {
            Some(TryMutexGuard{
                mutex: self,
                notsend: PhantomData,
            })
        }
    }

    // Consumes this mutex, returning the underlying data.
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

impl<T> UnwindSafe for TryMutex<T> { }
impl<T> RefUnwindSafe for TryMutex<T> { }
unsafe impl<T> Sync for TryMutex<T> { }

impl<T> From<T> for TryMutex<T> {
    fn from(t: T) -> Self {
        TryMutex::new(t)
    }
}