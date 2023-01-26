use regex::Regex;

use crate::utils;


pub fn star_1() {
    let binding = utils::read_input("inputs/05");
    let mut contents = binding.lines();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; 10];   // Ignore stack 0 to make 1-indexed as per brief
    loop {
        let line = contents.next().expect("Value");
        if line.starts_with(" 1") {
            break;
        }

        for i in 0..((line.chars().count() + 1) / 4) {
            let c: char = line.chars().nth(4 * i + 1).expect("Need a char");
            if c != ' ' {
                stacks[i+1].push(c);
            }
        }

    }
    for i in 1..stacks.len() {
        stacks[i].reverse();
    }
    println!();
    contents.next();

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    loop {
        let line = match contents.next() {
            Some(l) => l,
            None => {break;},
        };

        let cap = re.captures(line).unwrap();
        let number_move = &cap[1].trim().parse::<usize>().unwrap();
        let stack_from = &cap[2].trim().parse::<usize>().unwrap();
        let stack_to = &cap[3].trim().parse::<usize>().unwrap();

        for _ in 0..*number_move {
            let popped_char = stacks[*stack_from].pop().unwrap();
            stacks[*stack_to].push(popped_char);
        }

    }
    for i in 0..10 {
        match stacks[i].last() {
            Some(c) => {print!("{c}")},
            None => (),
        };
    }
    println!();
}



pub fn star_2() {
    let binding = utils::read_input("inputs/05");
    let mut contents = binding.lines();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; 10];   // Ignore stack 0 to make 1-indexed as per brief
    loop {
        let line = contents.next().expect("Value");
        if line.starts_with(" 1") {
            break;
        }

        for i in 0..((line.chars().count() + 1) / 4) {
            let c: char = line.chars().nth(4 * i + 1).expect("Need a char");
            if c != ' ' {
                stacks[i+1].push(c);
            }
        }

    }
    for i in 1..stacks.len() {
        stacks[i].reverse();
    }
    println!();
    contents.next();

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    loop {
        let line = match contents.next() {
            Some(l) => l,
            None => {break;},
        };

        let cap = re.captures(line).unwrap();
        let number_move = *&cap[1].trim().parse::<usize>().unwrap();
        let stack_from = *&cap[2].trim().parse::<usize>().unwrap();
        let stack_to = *&cap[3].trim().parse::<usize>().unwrap();

        let split_index = stacks[stack_from].len() - number_move;
        let temp_stack = stacks[stack_from].split_off(split_index);
        stacks[stack_to].extend_from_slice(&temp_stack);

    }
    for i in 0..10 {
        match stacks[i].last() {
            Some(c) => {print!("{c}")},
            None => (),
        };
    }
    println!();
}
