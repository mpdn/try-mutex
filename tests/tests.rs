extern crate compiletest_rs as compiletest;
extern crate try_mutex;

use std::thread;
use std::sync::Arc;
use std::path::PathBuf;
use try_mutex::TryMutex;

#[test]
fn compile_test() {
    let mut config = compiletest::Config::default();

    config.mode = "compile-fail".parse().expect("Invalid mode");
    config.src_base = PathBuf::from("tests/compile-fail");
    config.link_deps(); 
    config.clean_rmeta();

    compiletest::run_tests(&config);
}

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
    thread::spawn(move || { *a.try_lock().unwrap() = true; }).join().unwrap();
    assert!(*b.try_lock().unwrap() == true);
}

#[test]
fn only_one_lock() {
    let m = TryMutex::new(false);

    {
        let mut a = m.try_lock().unwrap();
        assert!(m.try_lock().is_none());
        *a = true;
    }

    assert!(*m.try_lock().unwrap() == true)
}