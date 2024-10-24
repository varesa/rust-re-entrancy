use std::sync::Mutex;

struct Container(usize);

// function to increments contents of c by one
fn increment_by_one_and_print(recurse: bool) {
    static CONTAINER: Mutex<Container> = Mutex::new(Container(0));
    // not too easy to do by accident, since we need to drop the MutexGuard early
    let mut i = { CONTAINER.lock().unwrap().0 };
    i += 1;

    if recurse {
        increment_by_one_and_print(false);
    }

    CONTAINER.lock().unwrap().0 = i;
    println!("{}", CONTAINER.lock().unwrap().0);
}

fn main() {
   increment_by_one_and_print(true); // prints 1 twice
}
