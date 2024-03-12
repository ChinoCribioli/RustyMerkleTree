mod main;

mod test {
    use std::hash::{DefaultHasher, Hash, Hasher};
    use main::MerkleTree;
    
    pub fn test_tree_hashes() {
        let vec: Vec<i32> = vec![0,3,4,15];
        let mut mt = MerkleTree::new();
        let root_hash = mt.commit(vec);

        let mut hasher = DefaultHasher::new();
        Some(0).hash(&mut hasher);
        let h0: u64 = hasher.finish();
        assert_eq!(h0, 9307533986124549581);
        
        let mut hasher = DefaultHasher::new();
        Some(3).hash(&mut hasher);
        let h1: u64 = hasher.finish();
        assert_eq!(h1, 8685831180715912077);
        
        let mut hasher = DefaultHasher::new();
        Some(4).hash(&mut hasher);
        let h2: u64 = hasher.finish();
        assert_eq!(h2, 3152564662028086741);
        
        let mut hasher = DefaultHasher::new();
        Some(15).hash(&mut hasher);
        let h3: u64 = hasher.finish();
        assert_eq!(h3, 10836364048419001810);
        
        let mut hasher = DefaultHasher::new();
        h0.hash(&mut hasher);
        h1.hash(&mut hasher);
        let h01 = hasher.finish();
        assert_eq!(h01, 6124764072127861186);

        let mut hasher = DefaultHasher::new();
        h2.hash(&mut hasher);
        h3.hash(&mut hasher);
        let h23 = hasher.finish();
        assert_eq!(h23, 12328351441626597756);
        
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
}

fn main() {
    test::test_tree_hashes();
    test::test_verifying_path(vec![1,5,18,9,43], 0);
    test::test_verifying_path(vec![1,5,18,9,43], 1);
    test::test_verifying_path(vec![1,5,18,9,43], 2);
    test::test_verifying_path(vec![1,5,18,9,43], 3);
    test::test_verifying_path(vec![1,5,18,9,43], 4);

}