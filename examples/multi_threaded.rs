use std::sync::{Arc, Mutex};
use std::time::Duration;

struct Container(usize);

// function to increments contents of c by one
fn increment_by_one(c: Arc<Mutex<Container>>) {
    // not too easy to do by accident, since we need to drop the MutexGuard early
    let mut i = { c.lock().unwrap().0 };
    i += 1;
    std::thread::sleep(Duration::from_millis(100));
    c.lock().unwrap().0 = i;
}

fn main() {
    let c = Container(0);
    let arc = Arc::new(Mutex::new(c));

    let c1 = arc.clone();
    std::thread::spawn(move || { increment_by_one(c1)});
    std::thread::sleep(Duration::from_millis(50));

    let c2 = arc.clone();
    std::thread::spawn(move || { increment_by_one(c2)});
    std::thread::sleep(Duration::from_millis(150));

    println!("Result: {}", arc.lock().unwrap().0); // prints 1 instead of 2
}
