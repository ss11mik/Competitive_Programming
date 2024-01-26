/*
 * Hands-on 3
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

fn main() {
    let (_, _, itineraries) = parse_input();
    println!("{}", max_possible_attractions(&itineraries));
}

fn parse_input() -> (usize, usize, Vec<Vec<usize>>) {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line.");

    let (n, d) = match scan!(line, char::is_whitespace, usize, usize) {
        (Some(n), Some(d)) => (n, d),
        _ => panic!("Parse error."),
    };

    let mut itineraries: Vec<Vec<usize>> = Vec::with_capacity(n);

    for line in io::stdin().lock().lines() {
        let itinerary: Vec<usize> = line
            .expect("Parse error.")
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error."))
            .collect();
        itineraries.push(itinerary);
    }

    (n, d, itineraries)
}

fn max_possible_attractions(itineraries: &Vec<Vec<usize>>) -> usize {
    if itineraries.is_empty() {
        return 0;
    }

    let itineraries_cnt = itineraries.len();
    let days = itineraries.first().unwrap().len();

    let mut prev_city = vec![0; days + 1];

    // for the first itinerary, only create prefix sum
    // D iterations
    for day in 0..days {
        prev_city[day + 1] = prev_city[day] + itineraries[0][day];
    }

    // n-1 iterations
    // (n-1) * D^2 overall
    for i in 1..itineraries_cnt {
        let mut current_city = vec![0; days + 1];
        // first cell is a padding for when none of the current row is included in the result
        let mut pref_sum = vec![0; days + 1];

        // D + 1 iterations
        for day in 1..days + 1 {
            pref_sum[day] = pref_sum[day - 1] + itineraries[i][day - 1];

            // D + 1 iterations
            for d in 0..day + 1 {
                current_city[day] = current_city[day].max(pref_sum[d] + prev_city[day - d]);
            }
        }
        prev_city = current_city;
    }

    // return the last cell
    prev_city[days]
}
