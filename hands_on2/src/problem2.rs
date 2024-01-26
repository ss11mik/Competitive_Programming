/*
 * Hands-on 2
 * Competitive Programmming course @ UniPi
 * Autumn 2023
 *
 * code for 2nd problem of Hands-on
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

struct Segment {
    from: usize,
    to: usize,
}

struct Query {
    from: usize,
    to: usize,
    nr_segments: usize,
}

fn main() {
    // parse input into variables
    let (n, _, segments, queries) = parse_input();

    let mut array = vec![0; n + 1];

    // mark beginnings and ends of segments into array
    // O(n)
    for i in segments {
        array[i.from] += 1;
        array[i.to + 1] -= 1;
    }

    // perform prefix sum in-situ
    // O(n)
    for i in 0..n {
        array[i + 1] += array[i];
    }

    // create reversed index, where number of overlaps is index
    // and original index is the new value
    // O(n)
    let mut hits_indexed = vec![Vec::<usize>::new(); n];
    for i in 0..n + 1 {
        hits_indexed[array[i] as usize].push(i);
    }

    // iterate over queries in O(m)
    // the whole loop in O(m log(n))
    for query in queries {
        // O(1) direct access to cell of array,
        // binary search takes at most O(log(n))
        let result = find_binary_search(&hits_indexed[query.nr_segments], query.from, query.to);
        println!("{}", if result { 1 } else { 0 });
    }
}

fn parse_input() -> (usize, usize, Vec<Segment>, Vec<Query>) {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let (n, m) = match scan!(line, char::is_whitespace, usize, usize) {
        (Some(n), Some(m)) => (n, m),
        _ => panic!("Parse error."),
    };

    let mut segments: Vec<Segment> = Vec::with_capacity(n);
    let mut queries: Vec<Query> = Vec::with_capacity(m);
    let mut i = 0;

    for line in io::stdin().lock().lines() {
        let a = line.expect("Parse error.");
        let line_split: Vec<usize> = a
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error."))
            .collect();

        if i < n {
            segments.push(Segment {
                from: line_split[0],
                to: line_split[1],
            });
        } else {
            queries.push(Query {
                from: line_split[0],
                to: line_split[1],
                nr_segments: line_split[2],
            });
        }
        i += 1;
    }

    (n, m, segments, queries)
}

fn find_binary_search(array: &Vec<usize>, find_from: usize, find_to: usize) -> bool {
    let mut from = 0;
    let mut to = (&array).len();

    if to == 0 {
        return false;
    }

    while from < to {
        let candidate_index = ((to - from) / 2) + from;
        let candidate = (&array)[candidate_index];

        if find_from <= candidate && candidate <= find_to {
            return true;
        }

        if to - from == 1 {
            return false;
        }

        if find_from > candidate {
            from = candidate_index;
        }
        if candidate > find_to {
            to = candidate_index;
        }
    }
    false
}
