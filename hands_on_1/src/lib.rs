/*
 * Hands-on 1
 * Competitive Programmming course @ UniPi
 * Autumn 2023
 */

struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

/// This a representation of a tree.
/// Every node has an implicit id, which is its position on the vector `nodes`.
/// Every node has a key and at most two children. The ids of the children are
/// stored in `id_left` and `id_right`. These ids are `None` if the child does not exit.
impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    pub const ROOT_NODE: usize = 0;

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left child of the node `parent_id`
    /// if `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    // A private method that, for given index, returns a Node
    // and handles potential error states
    fn get_node(&self, node_id: Option<usize>) -> Option<&Node> {
        match node_id {
            Some(id) => {
                assert!(id < self.nodes.len(), "Node id is out of range");
                Some(&self.nodes[id])
            }
            None => None,
        }
    }

    // a method to check if the binary tree is a Binary Search Tree
    //
    // parameters:
    //  - node_id: ID of node to check
    pub fn is_bst(&self, node_id: usize) -> bool {
        self.rec_is_bst(Some(node_id)).0
    }

    // returns:
    //  - whether the processed node is BST
    //  - maximum value in subtree
    //  - minimum value in subtree
    fn rec_is_bst(&self, node_id: Option<usize>) -> (bool, Option<u32>, Option<u32>) {
        if let Some(node) = self.get_node(node_id) {
            let (left_is_bst, max_left, min_left) = self.rec_is_bst(node.id_left);
            let (right_is_bst, max_right, min_right) = self.rec_is_bst(node.id_right);

            // calculate minimum and maximum of node and its subtree
            let node_max = match max_right {
                Some(max_right) => max_right,
                None => node.key,
            };
            let node_min = match min_left {
                Some(min_left) => min_left,
                None => node.key,
            };

            // chceck for BST property of current node
            let left_satisfies_bst = match max_left {
                Some(max_left) => max_left < node.key,
                None => true,
            };
            let right_satisfies_bst = match min_right {
                Some(min_right) => node.key < min_right,
                None => true,
            };
            let node_is_bst = left_satisfies_bst && right_satisfies_bst;

            (
                left_is_bst && right_is_bst && node_is_bst,
                Some(node_max),
                Some(node_min),
            )
        } else {
            (true, None, None)
        }
    }

    // a method to check if the binary tree is balanced
    // A tree is considered balanced if, for each of its nodes, the heights of its left and right subtrees differ by at most one.
    //
    // parameters:
    //  - node_id: ID of node to check
    pub fn is_balanced(&self, node_id: usize) -> bool {
        self.rec_is_balanced(Some(node_id)).0
    }

    // returns:
    //  - whether the processed subtree is balanced
    //  - maximum distance to leaf (height)
    fn rec_is_balanced(&self, node_id: Option<usize>) -> (bool, usize) {
        if let Some(node) = self.get_node(node_id) {
            let (left_is_balanced, max_height_left) = self.rec_is_balanced(node.id_left);
            let (right_is_balanced, max_height_right) = self.rec_is_balanced(node.id_right);

            let node_is_balanced = max_height_left.abs_diff(max_height_right) <= 1;
            let node_max_height = max_height_left.max(max_height_right) + 1;

            (
                left_is_balanced && right_is_balanced && node_is_balanced,
                node_max_height,
            )
        } else {
            (true, 0)
        }
    }

    // a method to check if the binary tree is a max-heap
    // A max-heap is a complete binary tree in which every node satisfies the max-heap property. A node satisfies the max-heap property if its key is greater than or equal to the keys of its children.
    // A complete binary tree is a binary tree in which every level, except possibly the last, is completely filled, and all nodes in the last level are as far left as possible
    //
    // parameters:
    //  - node_id: ID of node to check
    pub fn is_max_heap(&self, node_id: usize) -> bool {
        self.rec_is_max_heap(Some(node_id)).0
    }

    // returns:
    //  - whether the processed node satisfies max heap property
    //  - value of the processed node
    //  - max height of the tree
    //  - min height of the tree
    fn rec_is_max_heap(&self, node_id: Option<usize>) -> (bool, Option<u32>, usize, usize) {
        if let Some(node) = self.get_node(node_id) {
            let (left_is_max_heap, left_value, min_height_left, max_height_left) =
                self.rec_is_max_heap(node.id_left);
            let (right_is_max_heap, right_value, min_height_right, max_height_right) =
                self.rec_is_max_heap(node.id_right);

            // check for max-heap property
            let left_max_heap = match left_value {
                Some(left_value) => left_value <= node.key,
                None => true,
            };
            let right_max_heap = match right_value {
                Some(right_value) => right_value <= node.key,
                None => true,
            };

            // check for completeness property
            let is_complete = (max_height_left.wrapping_sub(min_height_right) <= 1)
                && (max_height_left >= min_height_right);

            let node_is_max_heap = left_max_heap && right_max_heap && is_complete;

            let min_node_height = min_height_left.min(min_height_right) + 1;
            let max_node_height = max_height_left.max(max_height_right) + 1;

            (
                left_is_max_heap && right_is_max_heap && node_is_max_heap,
                Some(node.key),
                min_node_height,
                max_node_height,
            )
        } else {
            (true, None, 0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
    }

    #[test]
    fn test_is_bst() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.is_bst(Tree::ROOT_NODE), true);

        tree.add_node(Tree::ROOT_NODE, 5, true); // id 1

        assert_eq!(tree.is_bst(Tree::ROOT_NODE), true);

        tree.add_node(Tree::ROOT_NODE, 22, false); // id 2

        assert_eq!(tree.is_bst(Tree::ROOT_NODE), true);

        tree.add_node(1, 7, false); // id 3

        assert_eq!(tree.is_bst(Tree::ROOT_NODE), true);

        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.is_bst(Tree::ROOT_NODE), true);

        tree.add_node(4, 21, true); // id 5
        tree.add_node(5, 10, true); // id 6

        assert_eq!(tree.is_bst(Tree::ROOT_NODE), false);
    }

    #[test]
    fn test_is_balanced() {
        let mut tree = Tree::with_root(10);

        // o
        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), true);

        tree.add_node(Tree::ROOT_NODE, 5, true);

        //   o
        // o  \
        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), true);

        let new_node = tree.add_node(Tree::ROOT_NODE, 22, false);

        //   o
        // o  o
        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), true);

        let new_node = tree.add_node(new_node, 22, true);

        //    o
        //  o   o
        // / \ o \
        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), true);

        tree.add_node(new_node, 22, true);

        //         o
        //     o       o
        //   /   \   o   \
        //  / \ / \ o \ / \
        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), false);

        let new_node = tree.add_node(1, 22, true);

        //         o
        //     o       o
        //   o   \   o   \
        //  / \ / \ o \ / \
        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), false);

        tree.add_node(new_node, 22, true);

        // root is balanced, but both level 2 node are not.
        //
        //         o
        //     o       o
        //   o   \   o   \
        //  / o / \ o \ / \
        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), false);

        tree.add_node(1, 22, false);

        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), false);
        // left subtree is balanced now.
        assert_eq!(tree.is_balanced(1), true);

        tree.add_node(2, 22, false);

        //         o
        //     o       o
        //   o   o   o   o
        //  / o / \ o \ / \
        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), true);

        let new_node = tree.add_node(3, 22, false);
        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), true);

        tree.add_node(new_node, 22, false);

        // root by itself is balanced, but subtree 2 is not.
        //                o
        //        o               o
        //    o       o       o       o
        //  /   o   /   \   o   o   /   \
        // / \ / \ / \ / \ / \ o \ / \ / \
        assert_eq!(tree.is_balanced(Tree::ROOT_NODE), false);
        assert_eq!(tree.is_balanced(1), true);
        assert_eq!(tree.is_balanced(2), false);
    }

    #[test]
    fn test_is_max_heap() {
        let mut tree = Tree::with_root(128);

        // o
        assert_eq!(tree.is_max_heap(Tree::ROOT_NODE), true);

        // break the completeness property
        tree.add_node(Tree::ROOT_NODE, 15, false);
        //   o
        // /  o
        assert_eq!(tree.is_max_heap(Tree::ROOT_NODE), false);

        let mut tree = Tree::with_root(128);
        tree.add_node(Tree::ROOT_NODE, 110, true);
        //   o
        // o  \
        assert_eq!(tree.is_max_heap(Tree::ROOT_NODE), true);

        let new_node = tree.add_node(Tree::ROOT_NODE, 50, false);
        //   o
        // o  o
        assert_eq!(tree.is_max_heap(Tree::ROOT_NODE), true);

        let new_node = tree.add_node(1, 20, true);
        //    o
        //  o   o
        // o \ / \
        assert_eq!(tree.is_max_heap(Tree::ROOT_NODE), true);

        // break the completeness property
        tree.add_node(new_node, 2, true);
        //         o
        //     o       o
        //   o   \   /   \
        //  o \ / \ / \ / \
        assert_eq!(tree.is_max_heap(Tree::ROOT_NODE), false);

        // make the tree complete again
        tree.add_node(1, 15, false);
        assert_eq!(tree.is_max_heap(Tree::ROOT_NODE), false);
        tree.add_node(2, 15, true);
        assert_eq!(tree.is_max_heap(Tree::ROOT_NODE), false);
        tree.add_node(2, 15, false);
        assert_eq!(tree.is_max_heap(Tree::ROOT_NODE), true);
        //         o
        //     o       o
        //   o   o   o   o
        //  o \ / \ / \ / \

        // child can have same value as parent
        tree.add_node(new_node, 20, false);
        assert_eq!(tree.is_max_heap(Tree::ROOT_NODE), true);
    }
}
