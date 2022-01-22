fn main() {
    let arr = vec![1];

    let jh = std::thread::spawn(move || {
        println!("{:?}", arr);
    });

    jh.join().unwrap();

    // println!("{:?}", arr);
}
