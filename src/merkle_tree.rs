use std::hash::{DefaultHasher, Hash, Hasher};

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
    
    pub fn new() -> MerkleTree<T> {
        MerkleTree {
            root: Box::default(),
            values: Vec::new(),
        }
    }
    
    pub fn commit(&mut self, values: Vec<T>) -> u64 {
        assert!(self.values.len() == 0, "Cannot initialize a non-empty tree!");
        
        // We populate the self.values array, padding it to reach a power of 2 length
        for value in values.iter() {
            self.values.push(Some(value.clone()));
        }
        let mut pow2: usize = 1;
        while pow2 < values.len() { pow2 *= 2; }
        self.values.append(&mut vec![None; pow2 - self.values.len()]);

        assert_eq!(self.values.len(), pow2);

        let mut nodes = Vec::new();
        for index in 0..self.values.len() {
            let mut hasher = DefaultHasher::new();
            self.values[index].hash(&mut hasher);
            let node: Node = Node {
                node_hash: hasher.finish(),
                interval: (index, index),
                left: None,
                right: None,
            };
            nodes.push(Box::new(node.clone()));
        }
        
        while nodes.len() > 1 {
            let mut new_level: Vec<Box<Node>> = Vec::new();
            for index in (0..nodes.len()).step_by(2) {
                let mut left_node = nodes[index].clone();
                let mut right_node = nodes[index+1].clone();
                let mut hasher = DefaultHasher::new();
                left_node.node_hash.hash(&mut hasher);
                right_node.node_hash.hash(&mut hasher);
                let new_node: Node = Node {
                    node_hash: hasher.finish(),
                    interval: (left_node.interval.0,right_node.interval.1),
                    left: Some(left_node.clone()),
                    right: Some(right_node.clone()),
                };
                new_level.push(Box::new(new_node.clone()));
            };
            nodes = new_level;
        }
        self.root = nodes[0].clone();
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
        let value = self.values[index].as_ref().expect("There is no value at that index");
        let path = Self::get_recursive_path(*self.root, index);
        (value.clone(), path)
    }

    fn recalculate_hashes(self, node: &mut Node, index: usize) {
        if node.interval.0 == node.interval.1 {
            let mut hasher = DefaultHasher::new();
            self.values[index].hash(&mut hasher);
            node.node_hash = hasher.finish();
            return;
        };
        if Self::in_range(index, node.clone().left.expect("Node without left child").interval) {
            self.recalculate_hashes(node.left.as_mut().unwrap(), index);
        } else {
            self.recalculate_hashes(node.right.as_mut().unwrap(), index);
        }
        let mut hasher = DefaultHasher::new();
        node.clone().left.unwrap().node_hash.hash(&mut hasher);
        node.clone().right.unwrap().node_hash.hash(&mut hasher);
        node.node_hash = hasher.finish();
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

fn main() {
    // TODO:
    // * Sacar los hashers a un metodo extra que sea tipo hash(a: T, b: T) y te devuelve el hash. El valor b puede ser opcional para hashear cosas solas
    // * Chequear la privacidad de las cosas (basically que no puedas acceder a los valores del mt con otra cosa que no sea get_with_proof)
    
    let vec: Vec<u64> = vec![1,2342142,8];
    let mut mt = MerkleTree::new();
    mt.commit(vec);
    let root_hash = mt.change_value(3, 24);
    
    let mut hasher = DefaultHasher::new();
    Some(1 as u64).hash(&mut hasher);
    let h0: u64 = hasher.finish();
    
    let mut hasher = DefaultHasher::new();
    Some(2342142 as u64).hash(&mut hasher);
    let h1: u64 = hasher.finish();
    
    let mut hasher = DefaultHasher::new();
    Some(8 as u64).hash(&mut hasher);
    let h2: u64 = hasher.finish();
    
    let mut hasher = DefaultHasher::new();
    Some(24 as u64).hash(&mut hasher);
    let h3: u64 = hasher.finish();
    
    let mut hasher = DefaultHasher::new();
    h0.hash(&mut hasher);
    h1.hash(&mut hasher);
    let h01 = hasher.finish();

    let mut hasher = DefaultHasher::new();
    h2.hash(&mut hasher);
    h3.hash(&mut hasher);
    let h23 = hasher.finish();
    
    let mut hasher = DefaultHasher::new();
    h01.hash(&mut hasher);
    h23.hash(&mut hasher);
    let h03 = hasher.finish();
    assert_eq!(h03, root_hash);
    
    

    
    // let mut n1: Node = Node {
    //     name: "nodo 1",
    //     child: None
    // };
    // print!("n1 usa {} bytes\n", mem::size_of_val(&n1));
    // println!("\n{:?}", &n1);
    // let _para_debug = mem::size_of_val(n3.name);

}
