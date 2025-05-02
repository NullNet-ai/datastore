use prost::Message;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub hash: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MerkleTree {
    pub root: Option<Node>,
    leaves: Vec<Node>,
}

#[derive(Clone, prost::Message)]
pub struct ProtoNode {
    #[prost(string, tag = "1")]
    pub hash: String,
    #[prost(string, tag = "2")]
    pub value: String,
}

#[derive(Clone, prost::Message)]
pub struct TreeResponse {
    #[prost(string, tag = "1")]
    pub tree_id: String,
    #[prost(message, optional, tag = "2")]
    pub root: Option<ProtoNode>,
    #[prost(message, repeated, tag = "3")]
    pub leaves: Vec<ProtoNode>,
}

impl Default for MerkleTree {
    fn default() -> Self {
        MerkleTree::new()
    }
}

// Update the to_proto method to use ProtoNode
impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree {
            root: None,
            leaves: Vec::new(),
        }
    }

    pub fn add_leaf(&mut self, data: &str) {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let hash = hex::encode(hasher.finalize());
        let node = Node {
            hash,
            value: data.to_string(),
        };
        // Insert the node in sorted order
        let insert_pos = self
            .leaves
            .binary_search_by(|n| n.value.as_str().cmp(data))
            .unwrap_or_else(|pos| pos);
        self.leaves.insert(insert_pos, node);

        self.update_root();
    }

    pub fn prune(&mut self, n: usize) {
        // Do nothing if empty
        if self.root.is_none() {
            return;
        }

        // Calculate how many leaves to keep (2^n)
        let leaves_to_keep = 1 << n; // 2^n

        // If we have fewer leaves than the target, do nothing
        if self.leaves.len() <= leaves_to_keep {
            return;
        }

        // Keep only the most recent leaves (last n in sorted order)
        let start_idx = self.leaves.len() - leaves_to_keep;
        self.leaves = self.leaves.split_off(start_idx);

        // No need to update the root as we're preserving it
    }
    pub fn prune_to_level_4(&mut self) {
        self.prune(4);
    }

    pub fn update_root(&mut self) {
        let mut current_level = self.leaves.clone();
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            let mut i = 0;

            while i < current_level.len() {
                if i + 1 < current_level.len() {
                    let mut hasher = Sha256::new();
                    hasher.update(current_level[i].hash.as_bytes());
                    hasher.update(current_level[i + 1].hash.as_bytes());
                    let combined_hash = hex::encode(hasher.finalize());
                    let combined_value = format!(
                        "{} + {}",
                        current_level[i].value,
                        current_level[i + 1].value
                    );
                    next_level.push(Node {
                        hash: combined_hash,
                        value: combined_value,
                    });
                } else {
                    next_level.push(current_level[i].clone());
                }
                i += 2;
            }

            current_level = next_level;
        }

        self.root = current_level.first().cloned();
    }

    pub fn find_differences(&self, other: &MerkleTree) -> Vec<(usize, Node, Node)> {
        let mut differences = Vec::new();

        for (index, (self_leaf, other_leaf)) in
            self.leaves.iter().zip(other.leaves.iter()).enumerate()
        {
            if self_leaf.hash != other_leaf.hash {
                differences.push((index, self_leaf.clone(), other_leaf.clone()));
            }
        }

        // Check if one tree has more leaves than the other
        if self.leaves.len() > other.leaves.len() {
            for i in other.leaves.len()..self.leaves.len() {
                differences.push((
                    i,
                    self.leaves[i].clone(),
                    Node {
                        hash: "".to_string(),
                        value: "".to_string(),
                    },
                ));
            }
        }

        differences
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn deserialize(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_proto(&self) -> Vec<u8> {
        let tree_id = Uuid::new_v4().to_string();

        let proto_leaves: Vec<ProtoNode> = self
            .leaves
            .iter()
            .map(|leaf| ProtoNode {
                hash: leaf.hash.clone(),
                value: leaf.value.clone(),
            })
            .collect();

        let proto_root = self.root.as_ref().map(|r| ProtoNode {
            hash: r.hash.clone(),
            value: r.value.clone(),
        });

        let response = TreeResponse {
            tree_id,
            root: proto_root,
            leaves: proto_leaves,
        };

        let mut buf = Vec::new();
        response.encode(&mut buf).unwrap();
        buf
    }
}

impl MerkleTree {
    pub fn from_proto(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        let tree_response = TreeResponse::decode(bytes)?;

        let leaves = tree_response
            .leaves
            .into_iter()
            .map(|proto_node| Node {
                hash: proto_node.hash,
                value: proto_node.value,
            })
            .collect();

        let root = tree_response.root.map(|proto_node| Node {
            hash: proto_node.hash,
            value: proto_node.value,
        });

        Ok(MerkleTree { root, leaves })
    }
}
