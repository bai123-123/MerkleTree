use std::ops::Add;

use light_poseidon::{Poseidon, PoseidonBytesHasher, PoseidonHasher};
use ark_bn254::Fr;
use ark_ff::PrimeField;


pub struct HashAlgo {
    algo: Poseidon<Fr>,
}


impl HashAlgo {
    pub fn new() -> Self {
        HashAlgo {
            algo: Poseidon::<Fr>::new_circom(2).unwrap()
        }
    }

    pub fn hash_empty(&mut self) -> String {
        let hash = self.algo.hash_bytes_be(&[&[1u8; 32], &[2u8; 32]]).unwrap();
        let cut_str = String::from_utf8_lossy(&hash).to_string();
        cut_str[0..32].to_string()
    }

    pub fn hash_leaf(&mut self, value: String) -> String {
        assert_eq!(value.len(), 32);
        let bytes: [u8; 32] = value.as_bytes()[0..32].try_into().unwrap();
        let input1 = Fr::from_be_bytes_mod_order(&bytes);
        let input2 = Fr::from_be_bytes_mod_order(&[2u8; 32]);
        let hash = self.algo.hash(&[input1, input2]).unwrap();
        hash.to_string()[0..32].to_string()
    }

    pub fn hash_nodes(&mut self, hash1: &String, hash2: &String) -> String {
        let (small,big);
        if hash1.to_string() < hash2.to_string(){
            small = hash1;
            big = hash2;
        }else {
            small = hash2;
            big = hash1;
        }
        let bytes1: [u8; 32] = small.as_bytes()[0..32].try_into().unwrap();
        let bytes2: [u8; 32] = big.as_bytes()[0..32].try_into().unwrap();
        let input1 = Fr::from_be_bytes_mod_order(&bytes1);
        let input2 = Fr::from_be_bytes_mod_order(&bytes2);
        let hash = self.algo.hash(&[input1, input2]).unwrap();
        hash.to_string()[0..32].to_string()
    }
}


