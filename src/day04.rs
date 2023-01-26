use regex::Regex;

use crate::utils;


pub fn star_1() -> u16 {
    let contents = utils::read_input("inputs/04");

    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut count = 0u16;
    for line in contents.lines() {
        for cap in re.captures_iter(line) {
            let a_low = &cap[1].trim().parse::<u8>().unwrap();
            let a_hi = &cap[2].trim().parse::<u8>().unwrap();
            let b_low = &cap[3].trim().parse::<u8>().unwrap();
            let b_hi = &cap[4].trim().parse::<u8>().unwrap();
            if (a_low <= b_low && a_hi >= b_hi)
                || (a_low >= b_low && a_hi <= b_hi){
                count += 1;
            }
        }
    }
    count
}


pub fn star_2() -> u16 {
    let contents = utils::read_input("inputs/04");

    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut count = 0u16;
    for line in contents.lines() {
        for cap in re.captures_iter(line) {
            let a_low = &cap[1].trim().parse::<u8>().unwrap();
            let a_hi = &cap[2].trim().parse::<u8>().unwrap();
            let b_low = &cap[3].trim().parse::<u8>().unwrap();
            let b_hi = &cap[4].trim().parse::<u8>().unwrap();
            if (a_low <= b_low && a_hi >= b_low) || (b_low < a_low && b_hi >= a_low) {
                count += 1;
            }
        }
    }
    count
}
