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

        if self.root.is_none() {
            self.root = Some(node);
            return
        }

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
}

fn main() {
    let mut bst = Bst::default();
    bst.insert(3, "12");
    bst.insert(4, "12");
    bst.insert(6, "12");
    bst.insert(7, "12");
    bst.insert(2, "12");
    let node = bst.search(6);
    if let Some(nodef) = node {
        let min = bst.min(Some(nodef.clone()));
        if let Some(min) = min {
            println!("min {}", min.borrow().key);
        }
        let max = bst.max(Some(nodef));
        if let Some(max) = max {
            println!("max {}", max.borrow().key);
        }
    }
}
