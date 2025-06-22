use std::rc::{Rc,Weak};
use std::cell::RefCell;

pub type NodeRef<T, V> = Option<Rc<RefCell<Node<T,V>>>>;

#[derive(Debug)]
pub struct Node<T: Ord, V >{
    pub key: T,
    pub value: V,
    
    parent: Option<Weak<RefCell<Node<T,V>>>>,
    left: NodeRef<T, V>,
    right: NodeRef<T, V>
}

#[derive(Default)]
pub struct Bst<T: Ord, V>{
    pub root: Option<Rc<RefCell<Node<T,V>>>>
}

impl<T: Ord, V> Bst<T,V> {
    pub fn insert(&mut self, key: T, value: V) {
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

    pub fn search(&self, key: T) -> NodeRef<T,V> {
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

    pub fn min(&self, node: NodeRef<T,V>) -> NodeRef<T,V> {
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

    pub fn max(&self, node: NodeRef<T,V>) -> NodeRef<T,V> {
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

    pub fn remove(&mut self, z: NodeRef<T,V>) {
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
