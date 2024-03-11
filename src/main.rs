// rustc hello.rs && ./hello

#![allow(unused_imports)]
#![allow(unused)]

use std::default;
use std::io;
use std::mem;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::thread::current;

#[derive(Default, Debug, Clone)]
pub struct Node {
    node_hash: u64,
    interval: (usize, usize),
    parent: Option<Box<Node>>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct MerkleTree {
    root: Box<Node>,
    values: Vec<Option<i32>>,
}

impl MerkleTree {
    
    pub fn new() -> MerkleTree {
        MerkleTree {
            root: Box::default(),
            values: Vec::new(),
        }
    }
    
    pub fn commit(&mut self, values: Vec<i32>) -> u64 {
        assert!(self.values.len() == 0, "Cannot initialize a non-empty tree!");
        
        // We populate the self.values array, padding it to reach a power of 2 length
        for value in values.iter() {
            self.values.push(Some(*value));
        }
        let mut pow2: usize = 1;
        while pow2 < values.len() { pow2 *= 2; }
        self.values.append(&mut vec![None; pow2 - self.values.len()]);

        assert_eq!(self.values.len(), pow2);

        let mut nodes = Vec::new();
        for index in 0..self.values.len() {
            let mut hasher = DefaultHasher::new();
            self.values[index].hash(&mut hasher);
            let mut node: Node = Node {
                node_hash: hasher.finish(),
                interval: (index, index),
                parent: None,
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
                let mut new_node: Node = Node {
                    node_hash: hasher.finish(),
                    interval: (left_node.interval.0,right_node.interval.1),
                    parent: None,
                    left: Some(left_node.clone()),
                    right: Some(right_node.clone()),
                };
                // println!("nodo de intervalo {:?} tiene hash: {}",new_node.interval, new_node.node_hash);
                left_node.parent = Some(Box::new(new_node.clone()));
                right_node.parent = Some(Box::new(new_node.clone()));
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

    fn get_recursive_path(node: Node, index: usize) -> Vec<(u64)> {
        if ! Self::in_range(index, node.interval) {
            panic!("Index out of correct path");
        }
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

        Vec::new()
    }
    
    pub fn get_with_proof(self, index: usize) -> (i32, Vec<u64>) {
        let value = self.values[index].expect("There is no value at that index");
        let mut path = Self::get_recursive_path(*self.root, index);
        (value, path)
    }

}

fn main() {
    // TODO:
    // * Hacer un sistema de testing
    // * Chequear la privacidad de las cosas (basically que no puedas acceder a los valores del mt con otra cosa que no sea get_with_proof)
    // * Refactorear para poder hacer un MT con tipos abstractos (Es decir, poner <T> en todos los metodos).

    let vec: Vec<i32> = vec![0,3,4,15];
    let mut mt = MerkleTree::new();
    let root_hash = mt.commit(vec);
    println!("root of the new tree: {:?}", root_hash);

    let query_result = mt.get_with_proof(1);

    assert_eq!(query_result.0, 3);
    let mut hasher = DefaultHasher::new();
    Some(query_result.0).hash(&mut hasher);
    let mut current_hash = hasher.finish();
    let mut index = 1;
    for h in query_result.1.iter() {
        let mut hasher = DefaultHasher::new();
        if (index & 1) != 0 {
            h.hash(&mut hasher);
            current_hash.hash(&mut hasher);
        } else {
            current_hash.hash(&mut hasher);
            h.hash(&mut hasher);
        }
        current_hash = hasher.finish();
        index /= 2;
    }
    assert_eq!(current_hash, root_hash);

    
    // let mut n1: Node = Node {
    //     name: "nodo 1",
    //     child: None
    // };
    // print!("n1 usa {} bytes\n", mem::size_of_val(&n1));
    // println!("\n{:?}", &n1);
    // let _para_debug = mem::size_of_val(n3.name);

}
