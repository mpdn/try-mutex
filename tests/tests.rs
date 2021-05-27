use std::sync::Arc;
use std::thread;
use try_mutex::{TryMutex, TryMutexGuard};
use static_assertions::assert_impl_all;

#[test]
fn get_mut() {
    let mut m = TryMutex::new(0);
    *m.get_mut() += 1;
    assert!(m.into_inner() == 1);
}

#[test]
fn single_thread() {
    let m = TryMutex::new(0);
    *m.try_lock().unwrap() += 1;
    assert!(m.into_inner() == 1);
}

#[test]
fn across_threads() {
    let a = Arc::new(TryMutex::new(false));
    let b = a.clone();
    thread::spawn(move || {
        *a.try_lock().unwrap() = true;
    })
    .join()
    .unwrap();
    assert!(*b.try_lock().unwrap());
}

#[test]
fn only_one_lock() {
    let m = TryMutex::new(false);

    {
        let mut a = m.try_lock().unwrap();
        assert!(m.try_lock().is_none());
        *a = true;
    }

    assert!(*m.try_lock().unwrap())
}

assert_impl_all!(TryMutex<bool>: Sync);
assert_impl_all!(TryMutexGuard<'static, bool>: Sync);
assert_impl_all!(TryMutexGuard<'static, bool>: Send);
