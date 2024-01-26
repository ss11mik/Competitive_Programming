/*
 * Hands-on 2
 * Competitive Programmming course @ UniPi
 * Autumn 2023
 *
 * code for 1st problem of Hands-on
 */

use std::io;
use std::io::BufRead;

// adapted from https://stackoverflow.com/a/31048103
macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

struct SegmentTree {
    value: usize,
    left: Option<Box<SegmentTree>>,
    right: Option<Box<SegmentTree>>,
    from: usize,
    to: usize,
    lazy_update: Option<usize>,
}

enum QueryType {
    Max,
    Update,
}

pub struct Query {
    query_type: QueryType,
    from: usize,
    to: usize,
    value: usize,
}

fn main() {
    // parse input into variables
    let (_, _, array, queries) = parse_input();

    // construct a segment tree out of an array
    // O(n*log(n))
    let mut root = SegmentTree::create_tree(array);

    // iterate over queries in O(m)
    // the whole loop in O(m*log(n))
    for query in queries {
        match query.query_type {
            QueryType::Update => {
                // uses lazy update to keep under O(log(n))
                root.update(query.from, query.to, query.value);
            }
            QueryType::Max => {
                let result = root.max(query.from, query.to);
                println!("{}", result);
            }
        }
    }
}

pub fn parse_input() -> (usize, usize, Vec<usize>, Vec<Query>) {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line.");

    let (n, m) = match scan!(line, char::is_whitespace, usize, usize) {
        (Some(n), Some(m)) => (n, m),
        _ => panic!("Parse error."),
    };

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let arr_size: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error."))
        .collect();

    let mut queries: Vec<Query> = Vec::new();

    for line in io::stdin().lock().lines() {
        let a = line.expect("Parse error.");
        let query: Vec<usize> = a
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error."))
            .collect();

        queries.push(match query[0] {
            0 => Query {
                query_type: QueryType::Update,
                from: query[1],
                to: query[2],
                value: query[3],
            },
            1 => Query {
                query_type: QueryType::Max,
                from: query[1],
                to: query[2],
                value: 0,
            },
            _ => panic!("Parse error."),
        });
    }

    (n, m, arr_size, queries)
}

impl SegmentTree {
    fn new() -> Self {
        Self {
            value: 0,
            left: None,
            right: None,
            from: 0,
            to: 0,
            lazy_update: None,
        }
    }

    fn new_with_bounds(value: usize, from: usize, to: usize) -> Self {
        Self {
            value,
            left: None,
            right: None,
            from,
            to,
            lazy_update: None,
        }
    }

    fn insert_left(&mut self, child: Self) {
        self.value = self.value.max(child.value);
        self.from = child.from;
        self.left = Some(Box::new(child));
    }

    fn insert_right(&mut self, child: Self) {
        self.value = self.value.max(child.value);
        self.to = self.from.max(child.to);
        self.right = Some(Box::new(child));
    }

    pub fn create_tree(array: Vec<usize>) -> Self {
        // encapsulate array values into leaves
        let mut leaves: Vec<Self> = Vec::with_capacity(array.len());
        let mut i = 1;
        for leaf in array {
            leaves.push(Self::new_with_bounds(leaf, i, i));
            i += 1;
        }

        // create new level of nodes until there is only one in the level.
        let mut old_tree_level = leaves;
        while old_tree_level.len() > 1 {
            let mut new_tree_level: Vec<Self> = Vec::new();

            for node in old_tree_level {
                let potential_parent = new_tree_level.last_mut();
                match potential_parent {
                    Some(parent) => {
                        match parent.right {
                            Some(_) => {
                                // The last node does not have any free space, create another.
                                let mut new_parent = Self::new();
                                new_parent.insert_left(node);
                                new_tree_level.push(new_parent);
                            }
                            None => {
                                // The last node does have free space in right subtree.
                                parent.insert_right(node);
                            }
                        }
                    }
                    None => {
                        // there is no node in the new level. Create first.
                        let mut new_parent = Self::new();
                        new_parent.insert_left(node);
                        new_tree_level.push(new_parent);
                    }
                }
            }

            // prevent chaining of nodes with only one child on right side of the tree.
            if let Some(last) = new_tree_level.last() {
                if last.right.is_none() {
                    if let Some(last) = new_tree_level.pop() {
                        if let Some(child) = last.left {
                            new_tree_level.push(*child);
                        }
                    }
                }
            }

            old_tree_level = new_tree_level;
        }

        // return the root node
        match old_tree_level.pop() {
            Some(root) => root,
            None => panic!("Error!"),
        }
    }

    pub fn max(&mut self, from: usize, to: usize) -> usize {
        // no overlap
        if self.from > to || self.to < from {
            return 0;
        }

        //if there is a pending update, apply it.
        self.apply_lazy_update();

        // examined segment of tree is fully contained in the query.
        if self.from >= from && self.to <= to {
            return self.value;
        }

        let mut max = 0;
        if let Some(ref mut left_child) = self.left {
            max = max.max(left_child.max(from, to));
        }
        if let Some(ref mut right_child) = self.right {
            max = max.max(right_child.max(from, to));
        }
        max
    }

    pub fn update(&mut self, from: usize, to: usize, value: usize) -> Option<usize> {
        //if there is a pending update, apply it.
        self.apply_lazy_update();

        // no overlap
        if self.from > to || self.to < from {
            return Some(self.value);
        }

        // examined segment of tree is fully contained in the query.
        if self.from >= from && self.to <= to {
            self.value = self.value.min(value);
            self.add_lazy_update_to_children(value);
            return Some(self.value);
        }

        let mut new_left_max: usize = 0;
        let mut new_right_max: usize = 0;
        if let Some(left_child) = &mut self.left {
            if let Some(new_max) = left_child.update(from, to, value) {
                new_left_max = new_max;
            }
        }
        if let Some(right_child) = &mut self.right {
            if let Some(new_max) = right_child.update(from, to, value) {
                new_right_max = new_max;
            }
        }

        self.value = new_right_max.max(new_left_max);
        Some(self.value)
    }

    pub fn apply_lazy_update(&mut self) {
        if let Some(update) = self.lazy_update {
            self.value = self.value.min(update);
            self.add_lazy_update_to_children(update);
            self.lazy_update = None;
        }
    }

    pub fn add_lazy_update_to_children(&mut self, value: usize) {
        if let Some(left_child) = &mut self.left {
            left_child.lazy_update = match left_child.lazy_update {
                Some(orig_update) => Some(orig_update.min(value)),
                None => Some(value),
            };
        }
        if let Some(right_child) = &mut self.right {
            right_child.lazy_update = match right_child.lazy_update {
                Some(orig_update) => Some(orig_update.min(value)),
                None => Some(value),
            };
        }
    }
}
