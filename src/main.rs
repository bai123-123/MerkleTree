use crate::merkleTree::MerkleTree;

mod merkleTree;
mod treeNode;
mod proof;
mod hashAlgo;

use light_poseidon::{PoseidonBytesHasher, PoseidonHasher};
use ark_ff::{BigInteger, PrimeField};

fn main() {

    // 规定value string大小为32位

    // 创建n层的Merkle树
    let mut merkle_tree = MerkleTree::new(4);


    //根据index插入数据
    merkle_tree.insertLeaf(2, String::from("12345678123456781234567812345678"));

    //根据index获取proof
    let proof = merkle_tree.getProof(2);
    println!("{:?}", proof);
    println!("{}", proof.siblings.len());

    //验证proof
    let res = merkle_tree.validate(proof);
    println!("{}", res)
}


