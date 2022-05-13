/// smart pointer
#[cfg(test)]
mod smart_pointer {
    use std::{
        cell::RefCell,
        sync::{Arc, Mutex},
        thread,
    };

    #[test]
    fn test_refcall_is_send() {
        let a = RefCell::new(1);
        thread::spawn(move || {
            println!("a = {:?}", a);
        });
    }
    /*
    #[test]
    fn refcell_is_not_sync() {
        let a = Arc::new(RefCell::new(1));
        let b = a.clone();
        let c = a.clone();
        thread::spawn(move || {
            println!("c = {:?}", c);
        });
    }
    */
    #[test]
    fn test_arc_mutex() {
        let a = Arc::new(Mutex::new(1));
        let b = a.clone();
        let c = a.clone();
        let handle = thread::spawn(move || {
            let mut g = c.lock().unwrap();
            *g += 1;
        });

        {
            let mut g = b.lock().unwrap();
            *g += 1;
        }

        handle.join().unwrap();
        println!("a = {:?}", a);
    }
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    #[test]
    fn test_from_into() {
        fn print(v: impl Into<IpAddr>) {
            println!("{:?}", v.into());
        }

        let v4: Ipv4Addr = "2.2.2.2".parse().unwrap();
        let v6: Ipv6Addr = "::1".parse().unwrap();

        print(v4);
        print(v6);

        print([1, 1, 1, 1]);
        print([0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122]);
    }

    #[test]
    fn test_asref() {
        #[allow(dead_code)]
        enum Language {
            Rust,
            TypeScript,
            Elixir,
            Haskell,
        }

        impl AsRef<str> for Language {
            fn as_ref(&self) -> &str {
                match self {
                    Language::Rust => "Rust",
                    Language::TypeScript => "TypeScript",
                    Language::Elixir => "Elixir",
                    Language::Haskell => "Haskell",
                }
            }
        }

        fn print_ref(v: impl AsRef<str>) {
            println!("{}", v.as_ref());
        }

        let lang = Language::Rust;
        // &str 实现了 AsRef<str>
        print_ref("Hello World!");
        // String 实现了 AsRef<str>
        print_ref("Hello World!".to_string());
        // Language 实现了 AsRef<str>
        print_ref(lang);
    }

    use std::alloc::{GlobalAlloc, Layout, System};
    // cargo test --package xx --bin xx --all-features -- tests::test_allocator --exact --nocapture > log 2>&1
    #[test]
    fn test_allocator() {
        struct MyAllocator;

        unsafe impl GlobalAlloc for MyAllocator {
            unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
                let data = System.alloc(layout);
                eprintln!("ALLO: {:p}, size: {}", data, layout.size());
                data
            }

            unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
                System.dealloc(ptr, layout);
                eprintln!("FREE: {:p}, size: {}", ptr, layout.size());
            }
        }

        // #[global_allocator]
        // static GLOBAL: MyAllocator = MyAllocator;

        #[allow(unused)]
        struct Matrix {
            // 使用不规则的数字如505可以让dbg!的打印很容易分辨出来
            data: [u8; 505],
        }
        impl Default for Matrix {
            fn default() -> Self {
                Self { data: [0; 505] }
            }
        }

        let data = Box::new(Matrix::default());

        println!(
            "!!! allocated memory: {:p}, len: {}",
            &*data,
            std::mem::size_of::<Matrix>()
        );

        // FREE
    }

    use std::borrow::Cow;
    use url::Url;
    #[test]
    fn test_cow() {
        fn print_pairs(pair: (Cow<str>, Cow<str>)) {
            println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
        }

        fn show_cow(cow: Cow<str>) -> String {
            match cow {
                Cow::Borrowed(v) => format!("Borrowed {}", v),
                Cow::Owned(v) => format!("Owned {}", v),
            }
        }

        let url =
            Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
        let mut pairs = url.query_pairs();

        assert_eq!(pairs.count(), 3);

        let (mut k, v) = pairs.next().unwrap();

        println!("key: {}, v: {}", k, v);
        k.to_mut().push_str("_lala");

        print_pairs((k, v));

        print_pairs(pairs.next().unwrap());

        print_pairs(pairs.next().unwrap());
    }

    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use std::time::Duration;

    #[test]
    fn test_mutex_guard() {
        // lazy_static 宏可以生成复杂的 static 对象
        lazy_static! {
            // 一般情况下 Mutex 和 Arc 一起在多线程环境下提供对共享内存的使用
            // 如果你把 Mutex 声明成 static，其生命周期是静态的，不需要 Arc
            static ref METRICS: Mutex<HashMap<Cow<'static, str>, usize>> =
                Mutex::new(HashMap::new());
        }

        // 用 Arc 来提供并发环境下的共享所有权（使用引用计数）
        let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> =
            Arc::new(Mutex::new(HashMap::new()));

        for _ in 0..32 {
            let m = metrics.clone();
            thread::spawn(move || {
                let mut g = m.lock().unwrap();

                let data = &mut g;

                let entry = data.entry("hello".into()).or_insert(0);
                *entry += 1;
            });
        }

        thread::sleep(Duration::from_millis(100));

        println!("metrics: {:?}", metrics.lock().unwrap());
    }

    #[test]
    fn test_my_string() {
        use std::{fmt, ops::Deref, str};

        const MINI_STRING_MAX_LEN: usize = 30;

        struct MiniString {
            len: u8,
            data: [u8; MINI_STRING_MAX_LEN],
        }

        impl MiniString {
            fn new(v: impl AsRef<str>) -> Self {
                let bytes = v.as_ref().as_bytes();
                let len = bytes.len();
                let mut data = [0u8; MINI_STRING_MAX_LEN];
                data[..len].copy_from_slice(bytes);
                Self {
                    len: len as u8,
                    data,
                }
            }
        }

        impl Deref for MiniString {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                str::from_utf8(&self.data[..self.len as usize]).unwrap()
            }
        }

        impl fmt::Debug for MiniString {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.deref())
            }
        }

        #[derive(Debug)]
        enum MyString {
            Inline(MiniString),
            Standard(String),
        }

        impl Deref for MyString {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                match *self {
                    MyString::Inline(ref v) => v.deref(),
                    MyString::Standard(ref v) => v.deref(),
                }
            }
        }

        impl From<&str> for MyString {
            fn from(s: &str) -> Self {
                match s.len() > MINI_STRING_MAX_LEN {
                    true => Self::Standard(s.to_owned()),
                    _ => Self::Inline(MiniString::new(s)),
                }
            }
        }

        impl fmt::Display for MyString {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.deref())
            }
        }

        // ----------------

        let len1 = std::mem::size_of::<MyString>();
        let len2 = std::mem::size_of::<MiniString>();
        println!("Len: MyString {}, MiniString {}", len1, len2);

        let s1: MyString = "hello world".into();
        let s2: MyString = "哈哈哈啊哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈阿大撒大撒".into();

        println!("s1: {:?}, s2: {:?}", s1, s2);

        println!(
            "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
            s1,
            s1.len(),
            s1.chars().count(),
            s2,
            s2.len(),
            s2.chars().count()
        );

        assert!(s1.ends_with("world"));
        assert!(s2.starts_with("哈哈哈"));
    }
}

/// collections
#[cfg(test)]
mod collectionsx {
    use std::fmt;

    #[test]
    fn xx() {
        let arr = [1, 2, 3, 4, 5];
        let vec = vec![1, 2, 3, 4, 5];
        let s1 = &arr[..2];
        let s2 = &vec[..2];
        println!("s1: {:?}, s2: {:?}", s1, s2);
    }

    #[test]
    fn derefx() {
        let v = vec![1, 2, 3];

        // Vec 实现了 Deref, &Vec<T> 会被自动解引用为 &[T], 符合接口定义
        print_slice(&v);

        print_slice(&v[..]);

        print_slice1(&v);

        print_slice1("你好".to_string());
    }

    fn print_slice<T: fmt::Debug>(s: &[T]) {
        println!("{:?}", s);
    }

    fn print_slice1<T, U>(s: T)
    where
        T: AsRef<[U]>,
        U: fmt::Debug,
    {
        println!("{:?}", s.as_ref());
    }

    #[test]
    fn test_iter() {
        // IteratorExt「继承」Iterator, 这样可以使用 Iterator 的全部功能
        trait IteratorExt: Iterator {
            fn window_count(self, count: u32) -> WindowCount<Self>
            where
                Self: Sized,
            {
                WindowCount { iter: self, count }
            }
        }

        // 这句很重要，它让所有实现了 Iterator 的 T 都自动实现 IteratorExt
        impl<T: ?Sized> IteratorExt for T where T: Iterator {}

        struct WindowCount<I> {
            iter: I,
            count: u32,
        }

        impl<I: Iterator> Iterator for WindowCount<I> {
            type Item = Vec<I::Item>;

            fn next(&mut self) -> Option<Self::Item> {
                let data = (0..self.count)
                    .filter_map(|_| self.iter.next())
                    .collect::<Vec<_>>();
                if data.is_empty() {
                    None
                } else {
                    Some(data)
                }
            }
        }

        // ----------------
        let data = vec![1, 2, 3, 4, 5];
        let result = data.iter().window_count(2).collect::<Vec<Vec<_>>>();
        println!("{:?}", result);
    }

    // use hashbrown::hash_map;
    use std::collections::HashMap;

    #[test]
    fn test_hashmap1() {
        fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
            println!("{}: len: {}, cap: {}", name, map.len(), map.capacity());
        }

        let mut map = HashMap::new();
        explain("empty", &map);

        map.insert('a', 1);
        explain("added 1", &map);

        map.insert('b', 2);
        map.insert('c', 3);
        explain("added 3", &map);

        map.insert('d', 4);
        explain("added 4", &map);

        // get 时需要使用引用，并且也返回引用
        assert_eq!(map.get(&'a'), Some(&1));
        assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

        map.remove(&'a');
        assert_eq!(map.contains_key(&'a'), false);
        assert_eq!(map.get(&'a'), None);
        explain("removed", &map);

        map.shrink_to_fit();
        explain("shrinked", &map);
    }

    #[test]
    fn test_hashmap2() {
        fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
            let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
            println!(
                "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
                name, arr[2], arr[3], arr[4], arr[5]
            );
            unsafe { std::mem::transmute(arr) }
        }

        let map = HashMap::new();
        let mut map = explain("empty", map);

        map.insert('a', 1);
        let mut map = explain("added 1", map);

        map.insert('b', 2);
        map.insert('c', 3);
        let mut map = explain("added 3", map);

        map.insert('d', 4);
        let mut map = explain("added 4", map);

        map.remove(&'a');
        explain("final", map);
    }
}
