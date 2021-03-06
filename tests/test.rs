extern crate linked_hash_map;

use linked_hash_map::LinkedHashMap;

fn assert_opt_eq<V: PartialEq>(opt: Option<&V>, v: V) {
    assert!(opt.is_some());
    assert!(opt.unwrap() == &v);
}

#[test]
fn test_insert_and_get() {
    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert_opt_eq(map.get(&1), 10);
    assert_opt_eq(map.get(&2), 20);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_index() {
    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(10, map[&1]);
    map[&2] = 22;
    assert_eq!(22, map[&2]);
}

#[test]
fn test_insert_update() {
    let mut map = LinkedHashMap::new();
    map.insert("1".to_string(), vec![10, 10]);
    map.insert("1".to_string(), vec![10, 19]);
    assert_opt_eq(map.get(&"1".to_string()), vec![10, 19]);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_debug() {
    let mut map = LinkedHashMap::new();
    assert_eq!(format!("{:?}", map), "{}");
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    assert_eq!(format!("{:?}", map), "{1: 10, 2: 20, 3: 30}");
    map.insert(2, 22);
    assert_eq!(format!("{:?}", map), "{1: 10, 3: 30, 2: 22}");
    map.get(&3);
    assert_eq!(format!("{:?}", map), "{1: 10, 3: 30, 2: 22}");
    map.get_refresh(&mut 3);
    assert_eq!(format!("{:?}", map), "{1: 10, 2: 22, 3: 30}");
    map.clear();
    assert_eq!(format!("{:?}", map), "{}");
}

#[test]
fn test_remove() {
    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.insert(5, 50);
    map.remove(&3);
    map.remove(&4);
    assert!(map.get(&3).is_none());
    assert!(map.get(&4).is_none());
    map.insert(6, 60);
    map.insert(7, 70);
    map.insert(8, 80);
    assert_opt_eq(map.get(&6), 60);
    assert_opt_eq(map.get(&7), 70);
    assert_opt_eq(map.get(&8), 80);
}


#[test]
fn test_pop() {
    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.insert(5, 50);
    assert_eq!(map.pop_front(), Some((1, 10)));
    assert!(map.get(&1).is_none());
    assert_eq!(map.pop_back(), Some((5, 50)));
    assert!(map.get(&5).is_none());
    map.insert(6, 60);
    map.insert(7, 70);
    map.insert(8, 80);
    assert_eq!(map.pop_front(), Some((2, 20)));
    assert!(map.get(&2).is_none());
    assert_eq!(map.pop_back(), Some((8, 80)));
    assert!(map.get(&8).is_none());
    map.insert(3, 30);
    assert_eq!(map.pop_front(), Some((4, 40)));
    assert!(map.get(&4).is_none());
    assert_eq!(map.pop_back(), Some((3, 30)));
    assert!(map.get(&3).is_none());
}

#[test]
fn test_clear() {
    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.clear();
    assert!(map.get(&1).is_none());
    assert!(map.get(&2).is_none());
    assert_eq!(format!("{:?}", map), "{}");
}

#[test]
fn test_iter() {
    let mut map = LinkedHashMap::new();

    // empty iter
    assert_eq!(None, map.iter().next());

    map.insert("a", 10);
    map.insert("b", 20);
    map.insert("c", 30);

    // regular iter
    let mut iter = map.iter();
    assert_eq!((&"a", &10), iter.next().unwrap());
    assert_eq!((&"b", &20), iter.next().unwrap());
    assert_eq!((&"c", &30), iter.next().unwrap());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());

    // reversed iter
    let mut rev_iter = map.iter().rev();
    assert_eq!((&"c", &30), rev_iter.next().unwrap());
    assert_eq!((&"b", &20), rev_iter.next().unwrap());
    assert_eq!((&"a", &10), rev_iter.next().unwrap());
    assert_eq!(None, rev_iter.next());
    assert_eq!(None, rev_iter.next());

    // mixed
    let mut mixed_iter = map.iter();
    assert_eq!((&"a", &10), mixed_iter.next().unwrap());
    assert_eq!((&"c", &30), mixed_iter.next_back().unwrap());
    assert_eq!((&"b", &20), mixed_iter.next().unwrap());
    assert_eq!(None, mixed_iter.next());
    assert_eq!(None, mixed_iter.next_back());
}

#[test]
fn test_iter_mut() {
    let mut map = LinkedHashMap::new();
    map.insert("a", 10);
    map.insert("c", 30);
    map.insert("b", 20);

    {
        let mut iter = map.iter_mut();
        let entry = iter.next().unwrap();
        assert_eq!(&"a", entry.0);
        *entry.1 = 17;

        // reverse iterator
        let mut iter = iter.rev();
        let entry = iter.next().unwrap();
        assert_eq!(&"b", entry.0);
        *entry.1 = 23;

        let entry = iter.next().unwrap();
        assert_eq!(&"c", entry.0);
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    assert_eq!(17, map[&"a"]);
    assert_eq!(23, map[&"b"]);
}

#[test]
fn test_borrow() {
    #[derive(PartialEq, Eq, Hash)] struct Foo(Bar);
    #[derive(PartialEq, Eq, Hash)] struct Bar(i32);

    impl ::std::borrow::Borrow<Bar> for Foo {
        fn borrow(&self) -> &Bar { &self.0 }
    }

    let mut map = LinkedHashMap::new();
    map.insert(Foo(Bar(1)), "a");
    map.insert(Foo(Bar(2)), "b");

    assert!(map.contains_key(&Bar(1)));
    assert!(map.contains_key(&Bar(2)));
    assert!(map.contains_key(&Foo(Bar(1))));
    assert!(map.contains_key(&Foo(Bar(2))));

    assert_eq!(map.get(&Bar(1)), Some(&"a"));
    assert_eq!(map.get(&Bar(2)), Some(&"b"));
    assert_eq!(map.get(&Foo(Bar(1))), Some(&"a"));
    assert_eq!(map.get(&Foo(Bar(2))), Some(&"b"));

    assert_eq!(map.get_refresh(&Bar(1)), Some(&mut "a"));
    assert_eq!(map.get_refresh(&Bar(2)), Some(&mut "b"));
    assert_eq!(map.get_refresh(&Foo(Bar(1))), Some(&mut "a"));
    assert_eq!(map.get_refresh(&Foo(Bar(2))), Some(&mut "b"));

    assert_eq!(map.get_mut(&Bar(1)), Some(&mut "a"));
    assert_eq!(map.get_mut(&Bar(2)), Some(&mut "b"));
    assert_eq!(map.get_mut(&Foo(Bar(1))), Some(&mut "a"));
    assert_eq!(map.get_mut(&Foo(Bar(2))), Some(&mut "b"));

    assert_eq!(map[&Bar(1)], "a");
    assert_eq!(map[&Bar(2)], "b");
    assert_eq!(map[&Foo(Bar(1))], "a");
    assert_eq!(map[&Foo(Bar(2))], "b");

    assert_eq!(map.remove(&Bar(1)), Some("a"));
    assert_eq!(map.remove(&Bar(2)), Some("b"));
    assert_eq!(map.remove(&Foo(Bar(1))), None);
    assert_eq!(map.remove(&Foo(Bar(2))), None);
}

#[test]
fn test_send_sync() {
    fn is_send_sync<T: Send + Sync>() {}

    is_send_sync::<LinkedHashMap<u32, i32>>();
    is_send_sync::<linked_hash_map::Iter<u32, i32>>();
    is_send_sync::<linked_hash_map::IterMut<u32, i32>>();
    is_send_sync::<linked_hash_map::Keys<u32, i32>>();
    is_send_sync::<linked_hash_map::Values<u32, i32>>();
}
