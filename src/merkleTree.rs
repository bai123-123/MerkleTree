use crate::treeNode::{MerkleNode};
use crate::hashAlgo::HashAlgo;
use num::pow;
use crate::proof::Proof;

pub struct MerkleTree {
    pub algo: Box<HashAlgo>,

    /// The root of the inner binary tree
    root: MerkleNode,

    /// The height of the tree
    height: usize,

    /// The number of leaf nodes in the tree
    count: usize,
    pub leaf_nodes: Vec<String>,

}


impl MerkleTree {
    pub(crate) fn new(height: usize) -> Self {
        let count = pow(2, height - 1);
        let mut empty_vec = Vec::with_capacity(count);
        for i in 0..count {
            empty_vec.push("".to_string())
        }
        MerkleTree {
            algo: Box::from(HashAlgo::new()),
            root: MerkleNode::empty(String::from("")),
            height,
            count,
            leaf_nodes: empty_vec,
        }
    }

    pub fn insertLeaf(&mut self, index: usize, value: String) -> bool {
        assert_eq!(value.len(), 32);
        let max_index = pow(2, self.count - 1);
        if index > max_index {
            false
        } else {
            self.leaf_nodes[index] = value;
            self.from_vec();
            true
        }
    }

    fn from_vec(&mut self) {
        let mut cur = Vec::with_capacity(self.count);
        let algo = &mut self.algo;

        for v in self.leaf_nodes.clone() {
            let leaf = MerkleNode::new_leaf(algo, v);
            cur.push(leaf);
        }

        while cur.len() > 1 {
            let mut next = Vec::new();
            while !cur.is_empty() {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                } else {
                    let left = cur.remove(0);
                    let right = cur.remove(0);

                    let combined_hash = self.algo.hash_nodes(left.hash(), right.hash());

                    let node = MerkleNode::Node {
                        hash: combined_hash,
                        left: Box::new(left),
                        right: Box::new(right),
                    };

                    next.push(node);
                }
            }
            cur = next;
        }

        debug_assert!(cur.len() == 1);

        let root = cur.remove(0);

        self.root = root;
    }


    pub fn getProof(&self, index: usize) -> Proof {
        let mut proof = Proof::new(index, self.leaf_nodes[index].clone(), self.root.hash().to_string());
        let res = proof.getProof(&self.root, self.count);
        proof
    }

    pub fn validate(&mut self, proof: Proof) -> bool {

        let leaf_hash = self.algo.hash_leaf(proof.value);

        let siblings = proof.siblings;
        let mut final_hash = leaf_hash.clone();
        for sib in siblings{
            final_hash = self.algo.hash_nodes(&final_hash, &sib);
        }

        if final_hash == proof.root_hash{
            return true
        }

        return false;
    }

    fn calTwo(&mut self, str1: &String, str2: &String) -> (String, String) {
        let res1 = self.algo.hash_nodes(str1, str2);
        let res2 = self.algo.hash_nodes(str2, str1);
        (res1, res2)
    }

    fn compareTwo(str1: &String, str2: &String, str3: &String, str4: &String) -> bool {
        if str1 == str3 {
            return true;
        } else if str1 == str4 {
            return true;
        } else if str2 == str3 {
            return true;
        } else if str2 == str4 {
            return true;
        }
        return false;
    }


    pub fn root_hash(&self) -> &String {
        self.root.hash()
    }

    fn height(&self) -> usize {
        self.height
    }

    fn count(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        self.count() == 0
    }
}