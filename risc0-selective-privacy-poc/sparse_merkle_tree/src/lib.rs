mod default_hashes;

use default_hashes::DEFAULT_HASHES;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

const TREE_DEPTH: usize = 32;
const ZERO_HASH: [u8; 32] = [
    110, 52, 11, 156, 255, 179, 122, 152, 156, 165, 68, 230, 187, 120, 10, 44, 120, 144, 29, 63,
    179, 55, 56, 118, 133, 17, 163, 6, 23, 175, 160, 29,
];
const ONE_HASH: [u8; 32] = [
    75, 245, 18, 47, 52, 69, 84, 197, 59, 222, 46, 187, 140, 210, 183, 227, 209, 96, 10, 214, 49,
    195, 133, 165, 215, 204, 226, 60, 119, 133, 69, 154,
];

/// Compute parent as the hash of two child nodes
fn hash_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

/// Sparse Merkle Tree with 2^32 leaves
pub struct SparseMerkleTree {
    values: HashSet<u32>,
    node_map: HashMap<(usize, u32), [u8; 32]>,
}

impl SparseMerkleTree {
    pub fn new(values: HashSet<u32>) -> Self {
        let node_map = Self::node_map(&values);
        Self { values, node_map }
    }

    pub fn new_empty() -> Self {
        Self::new(HashSet::new())
    }

    pub fn add_value(&mut self, new_value: u32) {
        if self.values.insert(new_value) {
            self.node_map = Self::node_map(&self.values);
        }
    }

    fn node_map(values: &HashSet<u32>) -> HashMap<(usize, u32), [u8; 32]> {
        let mut nodes: HashMap<(usize, u32), [u8; 32]> = HashMap::new();

        // Start from occupied leaves
        for &leaf_index in values {
            nodes.insert((TREE_DEPTH, leaf_index), ONE_HASH);
        }

        // Build tree bottom-up
        for depth in (0..TREE_DEPTH).rev() {
            let mut next_level = HashMap::new();
            let indices: Vec<u32> = nodes
                .keys()
                .filter(|(d, _)| *d == depth + 1)
                .map(|(_, i)| i >> 1) // parent index
                .collect();

            for &parent_index in indices.iter() {
                let left_index = parent_index << 1;
                let right_index = left_index | 1;

                let left = nodes
                    .get(&(depth + 1, left_index))
                    .unwrap_or(&DEFAULT_HASHES[depth]);
                let right = nodes
                    .get(&(depth + 1, right_index))
                    .unwrap_or(&DEFAULT_HASHES[depth]);

                if left != &DEFAULT_HASHES[depth] || right != &DEFAULT_HASHES[depth] {
                    let h = hash_node(left, right);
                    next_level.insert((depth, parent_index), h);
                }
            }

            nodes.extend(next_level);
        }
        nodes
    }

    pub fn root(&self) -> [u8; 32] {
        self.node_map
            .get(&(0, 0))
            .cloned()
            .unwrap_or(DEFAULT_HASHES[0])
    }

    pub fn get_authentication_path_for_value(&self, value: u32) -> [[u8; 32]; 32] {
        let mut path = [[0u8; 32]; 32];

        let mut current_index = value;

        for depth in (0..32).rev() {
            let sibling_index = current_index ^ 1;

            let sibling_hash = self
                .node_map
                .get(&(depth + 1, sibling_index))
                .cloned()
                .unwrap_or(DEFAULT_HASHES[depth]);

            path[31 - depth] = sibling_hash;
            current_index >>= 1;
        }

        path
    }

    pub fn values(&self) -> HashSet<u32> {
        self.values.clone()
    }

    pub fn verify_value_is_in_set(value: u32, path: [[u8; 32]; 32], root: [u8; 32]) -> bool {
        let mut hash = ONE_HASH;
        let mut current_index = value;
        for path_value in path.iter() {
            if current_index & 1 == 0 {
                hash = hash_node(&hash, path_value);
            } else {
                hash = hash_node(path_value, &hash);
            }
            current_index >>= 1;
        }
        root == hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_hashes() {
        assert_eq!(DEFAULT_HASHES[TREE_DEPTH - 1], ZERO_HASH);
        assert_eq!(
            DEFAULT_HASHES[0],
            [
                157, 148, 193, 146, 141, 23, 128, 25, 196, 90, 21, 193, 179, 235, 209, 157, 146,
                64, 171, 100, 192, 44, 121, 46, 78, 53, 190, 198, 191, 82, 85, 16
            ]
        );
    }

    #[test]
    fn test_empty_tree() {
        let empty_tree = SparseMerkleTree::new_empty();
        assert_eq!(empty_tree.root(), DEFAULT_HASHES[0]);
    }

    #[test]
    fn test_tree_1() {
        let values: HashSet<u32> = vec![0, 1, 2, 3].into_iter().collect();
        let tree = SparseMerkleTree::new(values);
        assert_eq!(
            tree.root(),
            [
                109, 94, 224, 93, 195, 77, 137, 36, 108, 105, 177, 22, 212, 17, 160, 255, 224, 61,
                191, 17, 129, 10, 26, 76, 197, 42, 230, 160, 80, 44, 101, 184
            ]
        );
    }

    #[test]
    fn test_tree_2() {
        let values: HashSet<u32> = vec![2147483648, 2147483649, 2147483650, 2147483651]
            .into_iter()
            .collect();
        let tree = SparseMerkleTree::new(values);

        assert_eq!(
            tree.root(),
            [
                36, 178, 159, 245, 165, 76, 242, 85, 25, 218, 149, 135, 194, 127, 130, 201, 219,
                187, 167, 216, 1, 222, 234, 197, 152, 156, 243, 174, 68, 27, 114, 8
            ]
        );
    }

    #[test]
    fn test_tree_3() {
        let values: HashSet<u32> = vec![2147483648, 0, 1, 2147483649].into_iter().collect();
        let tree = SparseMerkleTree::new(values);

        assert_eq!(
            tree.root(),
            [
                148, 76, 190, 191, 248, 243, 89, 40, 197, 157, 206, 23, 58, 197, 86, 169, 225, 217,
                110, 166, 54, 10, 245, 175, 168, 4, 145, 220, 30, 210, 67, 113
            ]
        );
    }

    #[test]
    fn test_auth_path() {
        let values: HashSet<u32> = vec![0, 1, 2, 3, 1337].into_iter().collect();
        let mut tree = SparseMerkleTree::new(values);
        let root = tree.root();
        let path = tree.get_authentication_path_for_value(0);
        assert!(SparseMerkleTree::verify_value_is_in_set(0, path, root));
        let path = tree.get_authentication_path_for_value(1);
        assert!(SparseMerkleTree::verify_value_is_in_set(1, path, root));
        let path = tree.get_authentication_path_for_value(1337);
        assert!(SparseMerkleTree::verify_value_is_in_set(1337, path, root));
        let path = tree.get_authentication_path_for_value(1338);
        assert!(!SparseMerkleTree::verify_value_is_in_set(1338, path, root));

        tree.add_value(1338);
        let path = tree.get_authentication_path_for_value(1338);
        let root = tree.root();
        assert!(SparseMerkleTree::verify_value_is_in_set(1338, path, root));
    }
}
