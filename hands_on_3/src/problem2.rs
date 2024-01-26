/*
 * Hands-on 3
 * Competitive Programmming course @ UniPi
 * Autumn 2023
 *
 * code for 2nd problem of Hands-on
 */

use std::io;
use std::io::BufRead;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Topic {
    beauty: usize,
    difficulty: usize,
}

// adapted from https://stackoverflow.com/a/31048103
macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() {
    // parse input into variables
    let (_, mut topics) = parse_input();

    // sort topics by beauty
    // n log(n)
    topics.sort_unstable();

    // find the longest increasing subsequence (LIS) on values of difficulty
    // n log(n)
    let lis_len = longest_increasing_subsequence(&topics).len();

    // print found value
    println!("{}", lis_len);
}

fn parse_input() -> (usize, Vec<Topic>) {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line.");

    let n = match scan!(line, char::is_whitespace, usize) {
        (Some(n),) => n,
        _ => panic!("Parse error."),
    };

    let mut topics: Vec<Topic> = Vec::with_capacity(n);
    let mut i = 0;

    for line in io::stdin().lock().lines() {
        // do not trust the input.
        if i == n {
            break;
        }

        let topic: Vec<usize> = line
            .expect("Parse error.")
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error."))
            .collect();

        topics.push(Topic {
            beauty: topic[0],
            difficulty: topic[1],
        });

        i += 1;
    }

    (n, topics)
}

fn longest_increasing_subsequence(array: &Vec<Topic>) -> Vec<Topic> {
    let mut lis: Vec<Topic> = Vec::new();

    lis.push(array[0]);

    // n iterations
    for element in array {
        let lis_head = lis[lis.len() - 1];
        if element.difficulty > lis_head.difficulty {
            lis.push(*element);
        } else {
            // log(n)
            match lis.binary_search_by(|i| i.difficulty.cmp(&element.difficulty)) {
                Ok(pos) => lis[pos] = *element,
                Err(pos) => lis[pos] = *element,
            };
        }
    }

    lis
}
