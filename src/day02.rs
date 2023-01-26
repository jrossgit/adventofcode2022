use crate::utils;


pub fn star_1() -> i32 {

    let mut total_score: i32 = 0;
    for line in utils::read_input("inputs/02").lines() {
        match line.trim() {
            "A X" => total_score += 4,
            "A Y" => total_score += 8,
            "A Z" => total_score += 3,
            "B X" => total_score += 1,
            "B Y" => total_score += 5,
            "B Z" => total_score += 9,
            "C X" => total_score += 7,
            "C Y" => total_score += 2,
            "C Z" => total_score += 6,
            other => panic!("Unexpected value found: {other}"),
        }
    }
    total_score
}


pub fn star_2() -> i32 {

    let mut total_score: i32 = 0;
    for line in utils::read_input("inputs/02").lines() {
        match line.trim() {
            "A X" => total_score += 3,
            "A Y" => total_score += 4,
            "A Z" => total_score += 8,
            "B X" => total_score += 1,
            "B Y" => total_score += 5,
            "B Z" => total_score += 9,
            "C X" => total_score += 2,
            "C Y" => total_score += 6,
            "C Z" => total_score += 7,
            other => panic!("Unexpected value found: {other}"),
        }
    }
    total_score
}
