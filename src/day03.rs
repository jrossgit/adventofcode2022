use crate::utils;


pub fn star_1() -> u32 {

    let mut total = 0u32;
    for line in utils::read_input("inputs/03").lines() {
        let mut match_char = '?';

        let left = &line[..line.len()/2];
        let right = &line[line.len()/2..];

        for c in left.chars() {
            match right.find(c) {
                None => (),
                Some(_) => {match_char = c;}
            }
        }

        let ord = match_char as u32;
        if ord >= 97 {
            total += ord - 96;    // Lowercase
        } else {
            total += ord - 38;    // Uppercase
        }
    }
    total
}



pub fn star_2() -> u32 {

    let mut total = 0u32;
    let mut lines = vec!("", "", "");
    for (i, line) in utils::read_input("inputs/03").lines().enumerate() {

        lines[i%3] = line;
        let mut match_char = '?';
        if i%3 == 2 {
            for c in lines[0].chars() {
                if lines[1].contains(c) && lines[2].contains(c) {
                    match_char = c;
                    break;
                }
            }

            let ord = match_char as u32;
            if ord >= 97 {
                total += ord - 96;    // Lowercase
            } else {
                total += ord - 38;    // Uppercase
            }
        }
    }
    total
}
