use std::mem;
use std::collections::HashSet;

use rayon::prelude::*;

use crate::common::read_file;

pub fn ch5() {
    let polymers = read_file("ch5.txt");
    polymers.trim();

    test_polymers_removal(&polymers);
    react_on_string(polymers);
}

fn test_polymers_removal(polymers: &str) {
    println!("min length after some removal: {}", find_min_length_after_removing_polymer(polymers));
}

fn find_min_length_after_removing_polymer(polymers: &str) -> usize {
    let polymers: Vec<char> = polymers.chars().collect();
    unique_chars(&polymers).par_iter()
        .map(|p| (p, react(remove_polymers(&polymers, *p))))
        .min_by_key(|&(_, length_after)| length_after)
        .map(&|(_, length_after)| length_after)
        .unwrap()
}

fn remove_polymers(initial: &Vec<char>, to_remove: char) -> Vec<char> {
    initial.iter()
        .filter(|c| c.to_ascii_lowercase() != to_remove)
        .map(|c| *c)
        .collect()
}

fn react_on_string(polymers: String) {
    let length_after = react(str_to_char_vec(polymers));
    println!("after reactions, length: {}", length_after);
}

fn react(polymers: Vec<char>) -> usize {
    let mut before: Vec<char> = polymers;
    let mut after: Vec<char> = Vec::with_capacity(before.len());
    let mut reacted = true;
    while reacted {
        after.clear();
        reacted = false;
        let mut bi = 0;
        while bi < before.len() - 1 {
            if !should_react(before[bi], before[bi + 1]) {
                after.push(before[bi]);
                bi += 1;
            } else {
                bi += 2;
                reacted = true;
            }
        }
        if bi == before.len() - 1 {
            after.push(before[bi]);
        }
        if after.is_empty() {
            return 0;
        }
        after = mem::replace(&mut before, after);
    }
    before.len()
}

fn should_react(c1: char, c2: char) -> bool {
    let different_case = (c1.is_ascii_lowercase() && c2.is_ascii_uppercase()) || (c1.is_ascii_uppercase() && c2.is_ascii_lowercase());
    let same_letter = c1.to_ascii_lowercase() == c2.to_ascii_lowercase();
    different_case && same_letter
}

fn str_to_char_vec(str: String) -> Vec<char> {
    str.chars().collect()
}

fn unique_chars(all: &Vec<char>) -> HashSet<char> {
    all.iter()
        .map(|c| c.to_ascii_lowercase())
        .collect()
}