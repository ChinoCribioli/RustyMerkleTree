mod merkle_tree;
// use merkle_tree::*;
// use std::hash::{DefaultHasher, Hash, Hasher};
// use rand::thread_rng;
// use rand::Rng;
mod test;
use test::test::main_tests;

fn main() {
    // TODO:
    // * Chequear la privacidad de las cosas (basically que no puedas acceder a los valores del mt con otra cosa que no sea get_with_proof)
    
    
    main_tests();
}
