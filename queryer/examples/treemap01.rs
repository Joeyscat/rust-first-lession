use std::collections::BTreeMap;

fn main() {
    let map = BTreeMap::new();
    let mut map = explain("empty", map);

    for i in 0..16usize {
        map.insert(format!("JoJo {}", i), i);
    }

    let mut map = explain("added", map);

    map.remove("JoJo 1");

    let map = explain("remove 1", map);

    for item in map.iter() {
        println!("{:?}", item)
    }
}

// BTreeMap结构有height，node和length
fn explain<K, V>(name: &str, map: BTreeMap<K, V>) -> BTreeMap<K, V> {
    let arr: [usize; 3] = unsafe { std::mem::transmute(map) };
    println!(
        "{}: height: {}, root node: 0x{:x}, len: 0x{:x}",
        name, arr[0], arr[1], arr[2]
    );

    unsafe { std::mem::transmute(arr) }
}
