mod merkle_tree;

mod test {
    use std::hash::{DefaultHasher, Hash, Hasher};
    use merkle_tree::MerkleTree;
    // use crate::rand::thread_rng;
    
    pub fn test_tree_hashes() {
        // let mut rng = thread_rng();
        // let vec: Vec<i32> = vec![rng.gen(), rng.gen(), rng.gen()];
        let vec: Vec<i32> = vec![2, 4, 13451];
        // println!("{:?}", vec);
        let mut mt = MerkleTree::new();
        let root_hash = mt.commit(vec.clone());

        let mut hasher = DefaultHasher::new();
        Some(vec[0]).hash(&mut hasher);
        let h0: u64 = hasher.finish();
        
        let mut hasher = DefaultHasher::new();
        Some(vec[1]).hash(&mut hasher);
        let h1: u64 = hasher.finish();
        
        let mut hasher = DefaultHasher::new();
        Some(vec[2]).hash(&mut hasher);
        let h2: u64 = hasher.finish();
        
        let mut hasher = DefaultHasher::new();
        None::<i32>.hash(&mut hasher);
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
    }

    pub fn test_verifying_path(vec: Vec<i32>, index: usize) {
        let mut mt = MerkleTree::new();
        let root_hash = mt.commit(vec.clone());
        
        let query_result = mt.get_with_proof(index);

        assert_eq!(query_result.0, vec[index]);
        let mut hasher = DefaultHasher::new();
        Some(query_result.0).hash(&mut hasher);
        let mut current_hash = hasher.finish();
        let mut index = index;
        for h in query_result.1.iter() {
            // Here, we need to see if the node is its parent left node or right node in order to compute the hashes in the correct order.
            // Luckily, every bit of the binary representation of the index tells us which kind of child is the node in each case.
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
    }

    pub fn test_hashes_after_change() {
        let vec: Vec<i32> = vec![1,-2,8];
        let mut mt = MerkleTree::new();
        mt.commit(vec);
        let root_hash = mt.change_value(3, 24);

        let mut hasher = DefaultHasher::new();
        Some(1).hash(&mut hasher);
        let h0: u64 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        Some(-2).hash(&mut hasher);
        let h1: u64 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        Some(8).hash(&mut hasher);
        let h2: u64 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        Some(24).hash(&mut hasher);
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
    }
}

fn main() {
    test::test_tree_hashes();
    test::test_verifying_path(vec![1,5,18,9,43], 0);
    test::test_verifying_path(vec![1,5,18,9,43], 1);
    test::test_verifying_path(vec![1,5,18,9,43], 2);
    test::test_verifying_path(vec![1,5,18,9,43], 3);
    test::test_verifying_path(vec![1,5,18,9,43], 4);
    test::test_hashes_after_change();
}