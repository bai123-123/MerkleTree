use crate::merkleTree::MerkleTree;

mod merkleTree;
mod treeNode;
mod proof;
mod hashAlgo;

use light_poseidon::{Poseidon, PoseidonBytesHasher, PoseidonHasher};
use ark_ff::{BigInteger, PrimeField};

use ark_bn254::Fr;

fn main() {

    // 规定value string大小为32位

    // 创建n层的Merkle树
    let mut merkle_tree = MerkleTree::new(4);


    //根据index插入数据
    merkle_tree.insertLeaf(2, String::from("12345678123456781234567812345678"));

    //根据index获取proof
    let proof = merkle_tree.getProof(2);
    println!("{:?}", proof);



    //验证proof
    let res = merkle_tree.validate(proof);
    println!("{}", res);



    //
    // // let rrr = "19667131596597339518261940875762".to_string() > "12345678123456781234567812345678".to_string();
    // // println!("{}", rrr);
    //
    // let res = merkle_tree.algo.hash_nodes(&"18777063649433184696951059267692".to_string(),&"20779974778918562772626138008413".to_string());
    // println!("{}", res);

}


