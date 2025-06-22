mod bst;
pub mod bst_hashmap;

// let mut map = BstHashmap::<i32, String>::default();
// 
// map.insert(5, "five".to_string());
// map.insert(3, "three".to_string());
// map.insert(7, "seven".to_string());
// map.insert(6, "six".to_string());
// map.insert(8, "eight".to_string());
// 
// if let Some(value) = map.search(7) {
//     println!("Found key 7 with value: {}", value);
// } else {
//     println!("Key 7 not found");
// }
// 
// if let Some((min_key, min_val)) = map.min(5) {
//     println!("Min in subtree of 5: key = {}, value = {}", min_key, min_val);
// }
// 
// if let Some((max_key, max_val)) = map.max(5) {
//     println!("Max in subtree of 5: key = {}, value = {}", max_key, max_val);
// }
// 
// map.remove(7);
// println!("Removed key 7");
// 
// if map.search(7).is_none() {
//     println!("Key 7 no longer found after removal");
// }

#[cfg(test)]
mod tests {
    use super::bst::Bst;
    use super::bst_hashmap::BstHashmap;

    #[test]
    fn test_insert_and_search() {
        let mut map = BstHashmap::default();
        map.insert(5, "five");
        map.insert(3, "three");
        map.insert(7, "seven");

        assert_eq!(map.search(5), Some("five"));
        assert_eq!(map.search(3), Some("three"));
        assert_eq!(map.search(7), Some("seven"));
        assert_eq!(map.search(10), None);
    }

    #[test]
    fn test_min_max() {
        let mut map = BstHashmap::default();
        map.insert(10, "ten");
        map.insert(5, "five");
        map.insert(15, "fifteen");
        map.insert(3, "three");
        map.insert(7, "seven");

        assert_eq!(map.min(10), Some((3, "three")));
        assert_eq!(map.max(10), Some((15, "fifteen")));
        assert_eq!(map.min(5), Some((3, "three")));
        assert_eq!(map.max(5), Some((7, "seven")));
    }

    #[test]
    fn test_remove() {
        let mut map = BstHashmap::default();
        map.insert(5, "five");
        map.insert(3, "three");
        map.insert(7, "seven");

        map.remove(3);
        assert_eq!(map.search(3), None);

        map.remove(5);
        assert_eq!(map.search(5), None);

        assert_eq!(map.search(7), Some("seven"));
    }

    #[test]
    fn basic_bst_operations() {
        let mut bst = Bst::default();

        bst.insert(3, "val3");
        bst.insert(4, "val4");
        bst.insert(6, "val6");
        bst.insert(7, "val7");
        bst.insert(2, "val2");

        let node = bst.search(6);
        assert!(node.is_some(), "Node with key 6 should be found");
        let node_ref = node.unwrap();
        assert_eq!(node_ref.borrow().key, 6);

        let min_node = bst.min(Some(node_ref.clone()));
        assert!(min_node.is_some(), "Min should exist in subtree");
        assert_eq!(min_node.unwrap().borrow().key, 6);

        let max_node = bst.max(Some(node_ref));
        assert!(max_node.is_some(), "Max should exist in subtree");
        assert_eq!(max_node.unwrap().borrow().key, 7);

        assert_eq!(bst.root.as_ref().unwrap().borrow().key, 3);
    }

    #[test]
    fn remove_leaf_node() {
        let mut bst = Bst::default();
        bst.insert(5, "val5");
        bst.insert(3, "val3");
        bst.insert(7, "val7");

        let node = bst.search(3);
        assert!(node.is_some());
        bst.remove(node);

        assert!(bst.search(3).is_none());

        assert_eq!(bst.root.as_ref().unwrap().borrow().key, 5);
        assert!(bst.search(7).is_some());
    }

    #[test]
    fn remove_node_with_one_child() {
        let mut bst = Bst::default();
        bst.insert(5, "val5");
        bst.insert(3, "val3");
        bst.insert(4, "val4"); // filho direito de 3

        let node = bst.search(3);
        assert!(node.is_some());
        bst.remove(node);

        assert!(bst.search(3).is_none());
        assert!(bst.search(4).is_some());
        assert_eq!(bst.root.as_ref().unwrap().borrow().key, 5);
    }

    #[test]
    fn remove_node_with_two_children() {
        let mut bst = Bst::default();
        bst.insert(5, "val5");
        bst.insert(3, "val3");
        bst.insert(7, "val7");
        bst.insert(6, "val6");
        bst.insert(8, "val8");

        let node = bst.search(7);
        assert!(node.is_some());
        bst.remove(node);

        assert!(bst.search(7).is_none());
        assert!(bst.search(6).is_some());
        assert!(bst.search(8).is_some());
    }

    #[test]
    fn remove_root_node() {
        let mut bst = Bst::default();
        bst.insert(5, "val5");
        bst.insert(3, "val3");
        bst.insert(7, "val7");

        let root_node = bst.root.clone();
        bst.remove(root_node);

        assert!(bst.root.is_some());
        assert_ne!(bst.root.as_ref().unwrap().borrow().key, 5);
    }
}
