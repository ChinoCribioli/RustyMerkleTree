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

impl<T: Clone + Copy + Hash> MerkleTree<T> {
    
    pub fn new(values: &Vec<T>) -> MerkleTree<T> {
        let mut option_values: Vec<Option<T>> = Vec::new();
        
        // We populate the self.values array, padding it to reach a power of 2 length
        let values_len = values.len();
        for value in values {
            option_values.push(Some(*value));
        }
        let mut pow2: usize = 1;
        while pow2 < values_len { pow2 *= 2; }
        option_values.append(&mut vec![None; pow2 - values_len]);
        assert_eq!(option_values.len(), pow2);

        let mut nodes = Vec::new();
        for index in 0..option_values.len() {
            nodes.push(Box::new(
                Node {
                    node_hash: hash_values(vec![&option_values[index]]),
                    interval: (index, index),
                    left: None,
                    right: None,
                }
            ));
        }
        
        while nodes.len() > 1 {
            let mut new_level: Vec<Box<Node>> = Vec::new();
            for index in (0..nodes.len()).step_by(2) {
                let left_node = nodes[index].clone();
                let right_node = nodes[index+1].clone();
                let new_node: Node = Node {
                    node_hash: hash_values(vec![left_node.node_hash, right_node.node_hash]),
                    interval: (left_node.interval.0, right_node.interval.1),
                    left: Some(left_node),
                    right: Some(right_node),
                };
                new_level.push(Box::new(new_node));
            };
            nodes = new_level;
        }
        MerkleTree {
            root: nodes[0].clone(),
            values: option_values,
        }
    }

    pub fn get_root_hash(&self) -> u64 {
        self.root.node_hash
    }

    fn in_range(index: usize, interval: (usize, usize)) -> bool {
        interval.0 <= index && index <= interval.1
    }

    fn check_index_range(&self, index: usize) {
        if index >= self.values.len() {
            panic!("Index out of range of the tree.");
        }
    }

    fn get_recursive_path(node: Node, index: usize) -> Vec<u64> {
        if node.interval.0 == node.interval.1 {
            return Vec::new();
        };
        let left_ref = node.left.as_ref().expect("Found node without left child when getting proof");
        let right_ref = node.right.as_ref().expect("Found node without right child when getting proof");
        if Self::in_range(index, left_ref.interval) {
            let mut previous_path = Self::get_recursive_path(*node.left.unwrap(), index);
            previous_path.push(right_ref.node_hash);
            previous_path
        } else if Self::in_range(index, right_ref.interval) {
            let mut previous_path = Self::get_recursive_path(*node.right.unwrap(), index);
            previous_path.push(left_ref.node_hash);
            previous_path
        } else {
            panic!("Proof took the incorrect path when getting proof.")
        }
    }
    
    pub fn get_with_proof(self, index: usize) -> (T, Vec<u64>) {
        self.check_index_range(index);
        let value = self.values[index].as_ref().expect("There is no value at that index");
        let path = Self::get_recursive_path(*self.root, index);
        (*value, path)
    }

    fn recalculate_hashes(node: &mut Node, index: usize, new_value: T) {
        if node.interval.0 == node.interval.1 {
            node.node_hash = hash_values(vec![Some(new_value)]);
            return;
        };
        let left_ref = node.left.as_ref().expect("Found node without left child when recalculating hashes");
        let right_ref = node.right.as_ref().expect("Found node without right child when recalculating hashes");
        if Self::in_range(index, left_ref.interval) {
            Self::recalculate_hashes(node.left.as_mut().unwrap(), index, new_value);
        } else if Self::in_range(index, right_ref.interval) {
            Self::recalculate_hashes(node.right.as_mut().unwrap(), index, new_value);
        } else {
            panic!("Proof took the incorrect path when recalculating hashes.")
        }
        node.node_hash = hash_values(vec![
            node.left.as_ref().unwrap().node_hash,
            node.right.as_ref().unwrap().node_hash
        ]);
    }

    pub fn change_value(&mut self, index: usize, new_value: T) -> u64 {
        self.check_index_range(index);
        self.values[index] = Some(new_value);
        Self::recalculate_hashes(self.root.as_mut(), index, new_value);
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
