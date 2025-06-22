use std::rc::{Rc,Weak};
use std::cell::RefCell;

type NodeRef<T, V> = Option<Rc<RefCell<Node<T,V>>>>;

#[derive(Debug)]
struct Node<T: Ord, V >{
    key: T,
    value: V,
    
    parent: Option<Weak<RefCell<Node<T,V>>>>,
    left: NodeRef<T, V>,
    right: NodeRef<T, V>
}

#[derive(Default)]
struct Bst<T: Ord, V>{
    root: Option<Rc<RefCell<Node<T,V>>>>
}

impl<T: Ord, V> Bst<T,V> {
    fn insert(&mut self, key: T, value: V) {
        let node = Rc::new(RefCell::new(Node {
            key,
            value,
            parent: None,
            left: None,
            right: None,
        }));

        if let Some(root) = &self.root{
            let mut x = root.clone();
            let mut _parent: NodeRef<T, V> = None;

            loop {
                _parent = Some(x.clone());
                let x_is_bigger;
                {
                    let x_node = x.borrow();
                    x_is_bigger = node.borrow().key < x_node.key;
                }

                if  x_is_bigger {
                    let left;
                    {
                        left = x.borrow().left.clone();
                    }

                    if let Some(left_child) = left {
                        x = left_child.clone();
                    }else {
                       break 
                    }
                } else {
                    let right;
                    {
                        right = x.borrow().right.clone();
                    }

                    if let Some(right_child) = right {
                        x = right_child.clone();
                    } else {
                        break
                    }
                }
            }

            {
                let mut parent_borrow = _parent.as_ref().unwrap().borrow_mut();
                if node.borrow().key < parent_borrow.key {
                    parent_borrow.left = Some(node.clone());
                } else {
                    parent_borrow.right = Some(node.clone());
                }
            }

            node.borrow_mut().parent = Some(Rc::downgrade(_parent.as_ref().unwrap()));
        }else {
            self.root = Some(node);
        }
    }

    fn search(&self, key: T) -> NodeRef<T,V> {
        if let Some(root) = &self.root{
            let mut x = root.clone();

            loop {
                if x.borrow().key == key {
                   return Some(x);
                }else if x.borrow().key < key {
                    let right;
                    {
                        right = x.borrow().right.clone();
                    }

                    if let Some(right_child) = right {
                        x = right_child.clone();
                    } else {
                        return None;
                    }
                }else {
                    let left;
                    {
                        left = x.borrow().left.clone();
                    }

                    if let Some(left_child) = left {
                        x = left_child.clone();
                    }else {
                        return None;
                    }
                }
            }
        }else {
            return None;
        }
    }

    fn min(&self, node: NodeRef<T,V>) -> NodeRef<T,V> {
        if let Some(node) = node{
            let mut x = node.clone();

            loop {
                let left;
                {
                    left = x.borrow().left.clone();
                }

                if let Some(left_child) = left {
                    x = left_child.clone();
                } else {
                    break;
                }
            }
            
            return Some(x);
        }else {
            return None;
        }
    }

    fn max(&self, node: NodeRef<T,V>) -> NodeRef<T,V> {
        if let Some(node) = node {
            let mut x = node.clone();

            loop {
                let right;
                {
                    right = x.borrow().right.clone();
                }

                if let Some(right_child) = right {
                    x = right_child.clone();
                } else {
                    break;
                }
            }

            return Some(x);
        }else {
            return None;
        }
    }

    fn transplanting(&mut self, u: NodeRef<T,V>, v: NodeRef<T,V>) {
        if let Some(u) = u {
            if let Some(u_parent_weak) = u.borrow().parent.as_ref() {
                if let Some(u_parent_node) = u_parent_weak.upgrade() {
                    let mut u_parent_borrow = u_parent_node.borrow_mut();

                    if let Some(left_child) = &u_parent_borrow.left {
                        if Rc::ptr_eq(left_child, &u) {
                            u_parent_borrow.left = v.clone();
                        } else {
                            u_parent_borrow.right = v.clone();
                        }
                    } else {
                        u_parent_borrow.right = v.clone();
                    }
                }
            } else {
                self.root = v.clone();
            }

            if let Some(v) = v {
                v.borrow_mut().parent = u.borrow().parent.clone();
            }
        }
    }

    fn remove(&mut self, z: NodeRef<T,V>) {
        if let Some(z_node) = z {
            let left = z_node.borrow().left.clone();
            let right = z_node.borrow().right.clone();

            if left.is_none() {
                self.transplanting(Some(z_node.clone()), right);
            } else if right.is_none() {
                self.transplanting(Some(z_node.clone()), left);
            } else {
                let y = self.min(right.clone()).unwrap();

                if !Rc::ptr_eq(&y, right.as_ref().unwrap()) {
                    let y_right = y.borrow().right.clone();
                    self.transplanting(Some(y.clone()), y_right.clone());
                    y.borrow_mut().right = right.clone();
                    if let Some(r) = right {
                        r.borrow_mut().parent = Some(Rc::downgrade(&y));
                    }
                }

                self.transplanting(Some(z_node.clone()), Some(y.clone()));
                y.borrow_mut().left = left.clone();
                if let Some(l) = left {
                    l.borrow_mut().parent = Some(Rc::downgrade(&y));
                }
            }
        }
    }
}

fn main() {
    let mut bst = Bst::default();

    bst.insert(3, "value 3");
    bst.insert(4, "value 4");
    bst.insert(6, "value 6");
    bst.insert(7, "value 7");
    bst.insert(2, "value 2");

    let node = bst.search(6);
    if let Some(node_ref) = node {
        println!("Found node with key 6 and value: {}", node_ref.borrow().value);

        let min = bst.min(Some(node_ref.clone()));
        if let Some(min_node) = min {
            println!("Minimum key in the subtree of 6: {}", min_node.borrow().key);
        }

        let max = bst.max(Some(node_ref.clone()));
        if let Some(max_node) = max {
            println!("Maximum key in the subtree of 6: {}", max_node.borrow().key);
        }
    } else {
        println!("Node with key 6 not found");
    }

    bst.remove(bst.search(7));

    bst.remove(bst.search(4));

    bst.remove(bst.search(3));

    if let Some(root) = &bst.root {
        println!("Current root key: {}", root.borrow().key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
