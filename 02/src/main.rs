#![allow(dead_code)]

use std::io::Result;
use std::path::Path;

fn is_repeat_twice(n: &u64) -> bool {
    let ns = n.to_string();
    if ns.len().is_multiple_of(2) {
        let (aa, bb) = ns.split_at(ns.len() / 2);
        if aa == bb {
            return true;
        }
    }
    false
}

fn is_repeat_n(n: &u64) -> bool {
    let ns = n.to_string();
    let len = ns.len();

    (1..len / 2 + 1)
        .rev()
        .filter(|n| len.is_multiple_of(*n))
        .map(|n| ns[..n].repeat(len / n))
        .any(|part| part == ns)
}

fn invalid_id_sum<P: AsRef<Path>>(filename: P, filter_predicate: fn(&u64) -> bool) -> Result<u64> {
    Ok(std::fs::read_to_string(filename)?
        .lines()
        .next()
        .unwrap()
        .split(',')
        .flat_map(|product_id| {
            let (a, b) = product_id.split_once('-').unwrap();
            let na: u64 = a.parse().unwrap();
            let nb: u64 = b.parse().unwrap();
            (na..nb + 1).filter(filter_predicate)
        })
        .sum())
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day02_gift_shop {
        use super::super::*;

        #[test]
        fn sample() {
            assert_eq!(
                invalid_id_sum("data/sample.txt", is_repeat_twice).unwrap(),
                1227775554
            );
            assert_eq!(
                invalid_id_sum("data/sample.txt", is_repeat_n).unwrap(),
                4174379265
            );
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            assert_eq!(
                invalid_id_sum("data/input.txt", is_repeat_twice).unwrap(),
                34_826_702_005
            );
            assert_eq!(
                invalid_id_sum("data/input.txt", is_repeat_n).unwrap(),
                43_287_141_963
            );
        }
    }
}
