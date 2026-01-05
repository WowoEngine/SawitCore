use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;
use super::types::Value;

#[derive(Clone, Debug)]
pub struct BTreeNode {
    pub is_leaf: bool,
    pub keys: Vec<Value>,
    pub values: Vec<Value>,     // For leaf nodes
    pub children: Vec<BTreeNode>, // For internal nodes
}

impl BTreeNode {
    pub fn new(is_leaf: bool) -> Self {
        BTreeNode {
            is_leaf,
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
        }
    }

    fn insert_non_full(&mut self, order: usize, key: Value, value: Value) {
        let mut i = self.keys.len() as isize - 1;

        if self.is_leaf {
            self.keys.push(key.clone()); 
            self.values.push(value); 

            while i >= 0 {
                let current_key = &self.keys[i as usize];
                if key < *current_key {
                    self.keys.swap(i as usize + 1, i as usize);
                    self.values.swap(i as usize + 1, i as usize);
                    i -= 1;
                } else {
                    break;
                }
            }
        } else {
            while i >= 0 {
                 let current_key = &self.keys[i as usize];
                 if key < *current_key {
                     i -= 1;
                 } else {
                     break;
                 }
            }
            i += 1;
            let idx = i as usize;

            if self.children[idx].keys.len() >= order {
                self.split_child(idx, order);
                if key > self.keys[idx] {
                     self.children[idx + 1].insert_non_full(order, key, value);
                } else {
                     self.children[idx].insert_non_full(order, key, value);
                }
            } else {
                 self.children[idx].insert_non_full(order, key, value);
            }
        }
    }

    fn split_child(&mut self, index: usize, order: usize) {
        let mid = order / 2;
        let full_node = &mut self.children[index];
        let mut new_node = BTreeNode::new(full_node.is_leaf);

        let mut right_keys_vec = full_node.keys.split_off(mid);
        let middle_key = right_keys_vec.remove(0); 
        new_node.keys = right_keys_vec;

        if full_node.is_leaf {
            let right_values = full_node.values.split_off(mid);
             new_node.values = right_values;
             // Remove value corresponding to pivot if necessary (Go logic implied it)
             if new_node.is_leaf && !new_node.values.is_empty(){
                  new_node.values.remove(0);
             }
        } else {
             let right_children = full_node.children.split_off(mid);
             new_node.children = right_children;
        }

        self.keys.insert(index, middle_key);
        self.children.insert(index + 1, new_node);
    }
}

pub struct BTreeIndex {
    pub order: usize,
    pub root: BTreeNode,
    pub name: String,
    pub key_field: String,
}

impl BTreeIndex {
    pub fn new(order: usize, name: String, key_field: String) -> Self {
        let actual_order = if order == 0 { 32 } else { order };
        BTreeIndex {
            order: actual_order,
            root: BTreeNode::new(true),
            name,
            key_field,
        }
    }

    pub fn insert(&mut self, key: Value, value: Value) {
        if self.root.keys.len() >= self.order {
            let new_root = BTreeNode::new(false);
            let old_root = core::mem::replace(&mut self.root, new_root);
            self.root.children.push(old_root);
            
            self.root.split_child(0, self.order);
            self.root.insert_non_full(self.order, key, value);
        } else {
            self.root.insert_non_full(self.order, key, value);
        }
    }
    
    pub fn search(&self, key: &Value) -> Vec<Value> {
        self.search_node(&self.root, key)
    }
    
    fn search_node(&self, node: &BTreeNode, key: &Value) -> Vec<Value> {
        let mut i = 0;
        while i < node.keys.len() && key > &node.keys[i] {
            i += 1;
        }
        
        if i < node.keys.len() && key == &node.keys[i] {
            if node.is_leaf {
                return vec![node.values[i].clone()];
            } else {
                return self.search_node(&node.children[i + 1], key);
            }
        }
        
        if node.is_leaf {
            return Vec::new(); // Empty
        }
        
        self.search_node(&node.children[i], key)
    }
}
