#[cfg(test)]
pub mod test {
    use std::hash::{DefaultHasher, Hash, Hasher};
    use crate::merkle_tree::MerkleTree;
    use rand::thread_rng;
    use rand::Rng;

    #[test]
    pub fn test_tree_hashes() {
        let mut rng = thread_rng();
        let vec: Vec<i32> = vec![rng.gen(), rng.gen(), rng.gen()];
        let mt = MerkleTree::new(&vec);
        let root_hash = mt.get_root_hash();

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

    #[test]
    pub fn test_verifying_path() {
        let mut rng = thread_rng();
        let random_size = rng.gen_range(0..10000);
        let mut random_vec = Vec::<i64>::new();
        for _ in 0..random_size {
            random_vec.push(rng.gen::<i64>());
        }
        let mut index = rng.gen_range(0..random_size);

        let mt = MerkleTree::new(&random_vec);
        let root_hash = mt.clone().get_root_hash();
        let query_result = mt.get_with_proof(index);

        assert_eq!(query_result.0, random_vec[index]);
        let mut hasher = DefaultHasher::new();
        Some(query_result.0).hash(&mut hasher);
        let mut current_hash = hasher.finish();
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

    #[test]
    pub fn test_hashes_after_change() {
        let mut rng = thread_rng();
        let mut vec= Vec::<u128>::new();
        for _ in 0..5 {
            vec.push(rng.gen::<u128>());
        }
        let mut mt = MerkleTree::new(&vec);
        
        // Now, we are going to change the whole array of values to see if the final hash is the expected one
        let mut new_vec= Vec::<u128>::new();
        for _ in 0..7 {
            new_vec.push(rng.gen::<u128>());
        }
        let mut last_root_hash = 0;
        for i in 0..7 {
            last_root_hash = mt.change_value(i, new_vec[i])
        }

        let new_mt = MerkleTree::new(&new_vec);
        assert_eq!(last_root_hash, new_mt.get_root_hash());
    }
}