use std::rc::{Rc,Weak};
use std::cell::RefCell;

type NodeRef<T, V> = Option<Rc<RefCell<Node<T,V>>>>;

struct Node<T: Ord, V >{
    key: T,
    value: V,
    
    parent: Option<Weak<RefCell<Node<T,V>>>>,
    left: NodeRef<T, V>,
    right: NodeRef<T, V>
}

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
}

fn main() {
    let node = Node {
        key: 111,
        value: "dois",
        parent: None,
        left: None,
        right: None
    };
    println!("{}", node.value);
}
