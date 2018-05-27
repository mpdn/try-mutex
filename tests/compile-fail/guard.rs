extern crate try_mutex;

fn send<T: Send>(t: T) { }

fn main() {
    let m = try_mutex::TryMutex::new(false);
    send(m.try_lock().unwrap());
    //~^ ERROR the trait bound `*mut bool: std::marker::Send` is not satisfied in `try_mutex::TryMutexGuard<'_, bool>`
}