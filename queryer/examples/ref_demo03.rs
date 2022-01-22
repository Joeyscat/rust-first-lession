use std::{rc::Rc, sync::{Arc, Mutex}};

fn main() {

    let s = Arc::new("xx");

    let ss = s.clone();

    let jh = std::thread::spawn(move|| {
        println!("{:?}", ss);
    });

    println!("{:?}", s);

    jh.join().unwrap()
}
