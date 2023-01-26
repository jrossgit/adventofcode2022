use crate::utils;
use std::collections::HashSet;

pub fn star_1() -> u32 {
    let s = utils::read_input("inputs/06");

    let mut index = 0u32;
    for i in 0..s.len() {
        let ss: Vec<char> = s.chars().skip(i).take(4).collect();

        if  ss[0] != ss[1] &&
            ss[0] != ss[2] &&
            ss[0] != ss[3] &&
            ss[1] != ss[2] &&
            ss[1] != ss[3] &&
            ss[2] != ss[3] {
            index = (i as u32) + 4;
            break;
        }
    }

    index
}

pub fn star_2() -> u32 {
    let s = utils::read_input("inputs/06");

    let mut index = 0u32;
    'outer: for i in 0..s.len() {
        let ss: Vec<char> = s.chars().skip(i).take(14).collect();

        let mut char_set: HashSet<char> = HashSet::new();
        for c in ss {
            if !char_set.insert(c) {
                continue 'outer;
            }
        }

        index = (i as u32) + 14;
        break;
    }
    index
}

