use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::*;

const FIRST_KIND_COUNT: usize = 2;
const SECOND_KIND_COUNT: usize = 3;

pub fn ch2() {
    println!("{}", checksum_for_ids_in_file("ch2.txt"));
    for common_part in common_parts_of_closest_strings("ch2.txt") {
        println!("{}", common_part);
    }
}

pub fn checksum_for_ids_in_file(file_name: &str) -> usize {
    let ids = read_lines_from_file(file_name);

    let mut first_kind_appeared = 0;
    let mut second_kind_appeared = 0;

    for id in ids {
        let counts = unique_letter_counts(&id);
        if counts.contains(&FIRST_KIND_COUNT) {
            first_kind_appeared += 1;
        }
        if counts.contains(&SECOND_KIND_COUNT) {
            second_kind_appeared += 1;
        }
    }

    first_kind_appeared * second_kind_appeared
}

fn unique_letter_counts(str: &str) -> HashSet<usize> {
    let mut counts_by_letters: HashMap<char, usize> = HashMap::new();
    for c in str.chars() {
        let counter = counts_by_letters.entry(c).or_insert(0);
        *counter += 1;
    }
    counts_by_letters.values().map(|i| *i).collect()
}

pub fn common_parts_of_closest_strings(file_name: &str) -> Vec<String> {
    let ids = read_lines_from_file(file_name);
    find_strings_with_distance_less_than(&ids, 1).iter().map(|string_pair| string_pair.common_part()).collect()
}

struct StringPair<'a> {
    pub s1: &'a str,
    pub s2: &'a str,
}

impl <'a> StringPair<'a> {
    fn common_part(&self) -> String {
        let mut result = String::new();
        for (c1, c2) in self.s1.chars().zip(self.s2.chars()) {
            if c1 == c2 {
                result.push(c1);
            }
        }
        result
    }
}

fn find_strings_with_distance_less_than<'a>(
    strings: &'a Vec<String>,
    min_dist: usize,
) -> Vec<StringPair<'a>> {
    let mut result = Vec::new();
    for i in 0..(strings.len() - 1) {
        for j in (i + 1)..(strings.len()) {
            if distance(&strings[i], &strings[j]) <= min_dist {
                result.push(StringPair {
                    s1: &strings[i],
                    s2: &strings[j],
                });
            }
        }
    }
    result
}

fn distance(s1: &str, s2: &str) -> usize {
    assert!(s1.len() == s2.len());

    let mut distance = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            distance += 1;
        }
    }

    distance
}
