use crate::hashAlgo::HashAlgo;



pub enum MerkleNode {
    Empty {
        hash: String,
    },

    Leaf {
        hash: String,
        value: String,
    },

    Node {
        hash: String,
        left: Box<MerkleNode>,
        right: Box<MerkleNode>,
    },
}


impl MerkleNode {
    pub fn empty(hash: String) -> Self {
        MerkleNode::Empty {
            hash,
        }
    }


    pub fn new(hash: String, value: String) -> Self {
        MerkleNode::Leaf {
            hash,
            value,
        }
    }


    pub fn new_leaf(algo: &mut HashAlgo, value: String) -> MerkleNode
    {
        let value_tmp = value.clone();
        let hash;
        if value.len() > 1 {
            hash = algo.hash_leaf(value);
        } else {
            hash = algo.hash_empty();
        }
        MerkleNode::new(hash, value_tmp)
    }


    pub fn hash(&self) -> &String {
        match self {
            MerkleNode::Empty { ref hash } => hash,
            MerkleNode::Leaf { ref hash, .. } => hash,
            MerkleNode::Node { ref hash, .. } => hash,
        }
    }
}

