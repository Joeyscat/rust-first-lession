use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

// 如果要支持Hash，可以用#[derive(Hash)]，前提是每个字段都实现了Hash
#[derive(Debug, Hash, PartialEq, Eq)]
struct Student<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Student<'a> {
    pub fn new(name: &'a str, age: u8) -> Self {
        Self { name, age }
    }
}

fn main() {
    let mut hasher = DefaultHasher::new();
    let student = Student::new("JoJo", 18);

    // 实现了Hash的数据结构可以直接调用hash方法
    student.hash(&mut hasher);
    let mut map = HashMap::new();

    // 实现了Hash/PartialEq/Eq的数据结构可以作为HashMap的key
    map.insert(student, vec!["Math", "Writing"]);
    println!("hash: 0x{:x}, map: {:?}", hasher.finish(), map);
}
