#![allow(unused)]

use crate::bst::{Bst, NodeRef};

#[derive(Default)]
pub struct BstHashmap<T: Ord, V>{
    pub bst: Bst<T, V>
}

impl <T: Ord + Clone, V: Clone> BstHashmap<T, V> {
    pub fn insert(&mut self, key: T, value: V) {
        self.bst.insert(key, value);
    }

    pub fn search(&self, key: T) -> Option<V> {
        let node: NodeRef<T, V> = self.bst.search(key);
        node.map(|node| node.borrow().value.clone())
    }

    pub fn min(&self, key: T) -> Option<(T,V)> {
        let node = self.bst.search(key);
        let min: NodeRef<T, V> = self.bst.min(node);
        min.map(|min_node| (min_node.borrow().key.clone(), min_node.borrow().value.clone()))
    }
    
    pub fn max(&self, key: T) -> Option<(T,V)> {
        let node = self.bst.search(key);
        let max: NodeRef<T, V> = self.bst.max(node);
        max.map(|max_node| (max_node.borrow().key.clone(), max_node.borrow().value.clone()))
    }

    pub fn remove(&mut self, key: T){
        let node = self.bst.search(key);
        self.bst.remove(node);
    }
}
