mod test {
    use std::hash::{DefaultHasher, Hash, Hasher};
    use crate::main::MerkleTree;

    pub fn check_hashing_sequence() {
        let vec: Vec<i32> = vec![0,3,4,15];
        let mut mt = MerkleTree::new();
        print!("root of the new tree: {:?}\n", mt.commit(vec.clone()));

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
        assert_eq!(h03, 14438217938484679288);
    }
}

fn main() {
    test::check_hashing_sequence();
}