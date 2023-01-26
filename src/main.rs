mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod utils;


fn main() {
    // println!("Largest Number of calories: {}", day01::star_1());
    // println!("Largest Number of calories: {}", day01::star_2());

    // println!("Day 2/1 - {}", day02::star_1() );
    // println!("Day 2/2 - {}", day02::star_2() );

    // println!("Day 3/1 - {}", day03::star_1() );
    // println!("Day 3/2 - {}", day03::star_2() );

    // println!("Day 4/1 - {}", day04::star_1() );
    // println!("Day 4/2 - {}", day04::star_2() );

    print!("Day 5/1: ");
    day05::star_1();
    print!("Day 5/2: ");
    day05::star_2();

    println!("Day 6/1: {}", day06::star_1());
    println!("Day 6/2: {}", day06::star_2());

    println!("Day 7/1: {}", day07::star_1());
    println!("Day 7/2: {}", day07::star_2());
}
