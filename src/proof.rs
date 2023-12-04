

use crate::hashAlgo::HashAlgo;
use crate::treeNode::MerkleNode;



#[derive(Debug)]
pub struct Proof {
    pub index: usize,
    pub value: String,
    pub siblings: Vec<String>,
    pub root_hash: String,
    pub empty: bool,
}




impl Proof {
    pub fn new(index: usize, value: String, root_hash: String) -> Self {
        Proof {
            index,
            value,
            siblings: Vec::new(),
            root_hash,
            empty:true,
        }
    }


    pub fn getProof<'a>(&'a mut self, merkleNode: &'a MerkleNode, count: usize) -> Option<&String> {
        self.empty = false;
        self.getSiblingsByIndex(merkleNode,self.index,count)
    }


    pub fn getSiblingsByIndex<'a>(&'a mut self, merkleNode: &'a MerkleNode, idx: usize, count: usize) -> Option<&String> {
        if idx >= count {
            return None;
        }
        match merkleNode {
            MerkleNode::Empty { .. } => None,

            MerkleNode::Leaf {
                ref hash,
                ref value,
                ..
            } => {
                if count != 1 {
                    return None;
                }

                // self.siblings.push(hash.clone());

                Some(value)
            }

            MerkleNode::Node {
                ref hash,
                ref left,
                ref right,
            } => {
                let left_count = count.next_power_of_two() / 2;

                if idx < left_count {
                   Proof::getSiblingsByIndex(self,left, idx, left_count);
                    self.siblings.push(right.hash().clone());
                } else {
                     Proof::getSiblingsByIndex(self, right, idx - left_count, count - left_count);
                    self.siblings.push(left.hash().clone());
                }
                // self.siblings.push(hash.clone());

               None
            }
        }
    }



}