use std::hash::{DefaultHasher, Hash, Hasher};

pub fn hash_values<Hashable>(values: Vec<Hashable>) -> u64 where Hashable: Hash {
    let mut hasher = DefaultHasher::new();
    for value in values.iter() {
        value.hash(&mut hasher);
    }
    hasher.finish()
}

#[derive(Default, Debug, Clone)]
pub struct Node {
    node_hash: u64,
    interval: (usize, usize),
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Default, Debug, Clone)]
pub struct MerkleTree<T> where T: Clone, T: Hash {
    root: Box<Node>,
    values: Vec<Option<T>>,
}

impl<T: Clone + Hash> MerkleTree<T> {
    
    pub fn new(values: Vec<T>) -> MerkleTree<T> {
        let mut option_values: Vec<Option<T>> = Vec::new();
        
        // We populate the self.values array, padding it to reach a power of 2 length
        for value in values.iter() {
            option_values.push(Some(value.clone()));
        }
        let mut pow2: usize = 1;
        while pow2 < values.len() { pow2 *= 2; }
        option_values.append(&mut vec![None; pow2 - option_values.len()]);
        assert_eq!(option_values.len(), pow2);

        let mut nodes = Vec::new();
        for index in 0..option_values.len() {
            let node: Node = Node {
                node_hash: hash_values(vec![&option_values[index]]),
                interval: (index, index),
                left: None,
                right: None,
            };
            nodes.push(Box::new(node.clone()));
        }
        
        while nodes.len() > 1 {
            let mut new_level: Vec<Box<Node>> = Vec::new();
            for index in (0..nodes.len()).step_by(2) {
                let left_node = nodes[index].clone();
                let right_node = nodes[index+1].clone();
                let new_node: Node = Node {
                    node_hash: hash_values(vec![left_node.node_hash, right_node.node_hash]),
                    interval: (left_node.interval.0,right_node.interval.1),
                    left: Some(left_node.clone()),
                    right: Some(right_node.clone()),
                };
                new_level.push(Box::new(new_node.clone()));
            };
            nodes = new_level;
        }
        MerkleTree {
            root: nodes[0].clone(),
            values: option_values,
        }
    }

    pub fn get_root_hash(self) -> u64 {
        self.root.node_hash
    }

    pub fn in_range(index: usize, interval: (usize, usize)) -> bool {
        interval.0 <= index && index <= interval.1
    }

    fn check_index_range(self, index: usize) {
        if index >= self.values.len() {
            panic!("Index out of range of the tree.");
        }
    }

    fn get_recursive_path(node: Node, index: usize) -> Vec<u64> {
        if node.interval.0 == node.interval.1 {
            return Vec::new();
        };
        if Self::in_range(index, node.left.clone().expect("Node without left child").interval) {
            let mut previous_path = Self::get_recursive_path(*node.left.unwrap(), index);
            previous_path.push(node.right.unwrap().node_hash);
            return previous_path;
        } else {
            let mut previous_path = Self::get_recursive_path(*node.right.unwrap(), index);
            previous_path.push(node.left.unwrap().node_hash);
            return previous_path;
        }
    }
    
    pub fn get_with_proof(self, index: usize) -> (T, Vec<u64>) {
        self.clone().check_index_range(index);
        let value = self.values[index].clone().expect("There is no value at that index");
        let path = Self::get_recursive_path(*self.root, index);
        (value, path)
    }

    fn recalculate_hashes(self, node: &mut Node, index: usize) {
        if node.interval.0 == node.interval.1 {
            node.node_hash = hash_values(vec![&self.values[index]]);
            return;
        };
        if Self::in_range(index, node.clone().left.expect("Node without left child").interval) {
            self.recalculate_hashes(node.left.as_mut().unwrap(), index);
        } else {
            self.recalculate_hashes(node.right.as_mut().unwrap(), index);
        }
        node.node_hash = hash_values(vec![
            node.clone().left.unwrap().node_hash,
            node.clone().right.unwrap().node_hash
        ]);
    }

    pub fn change_value(&mut self, index: usize, new_value: T) -> u64 {
        self.clone().check_index_range(index);
        self.values[index] = Some(new_value);
        self.clone().recalculate_hashes(self.root.as_mut(), index);
        self.root.node_hash
    }

    pub fn print_hashes(option_node: Option<Box<Node>>) {
        match option_node {
            None => return,
            Some(node) => {
                Self::print_hashes(node.left);
                Self::print_hashes(node.right);
                println!("{:?} -> {}", node.interval, node.node_hash);
            }
        }
    }
}
