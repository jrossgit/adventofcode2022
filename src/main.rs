use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn star_1() -> u32 {
    let path = Path::new("inputs/01");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("Read file successfully"),
    }

    let mut largest_elf_value: u32 = 0;
    let mut current_elf_value: u32 = 0;

    for line in s.lines() {
        let trimmed_line = line.trim();
        // println!("ln {i}: {trimmed_line}");
        if trimmed_line != "" {
            current_elf_value += trimmed_line.parse::<u32>().unwrap();
        } else {
            if current_elf_value > largest_elf_value {
                largest_elf_value = current_elf_value;
                // println!("Current value {current_elf_value}");
            }
            current_elf_value = 0;
        }
    }

    largest_elf_value
}


fn replace_elf_value(elf_calories: &mut [u32; 3], value: u32) -> () {
    let mut insert_value = value;
    for i in 0..3 {
        let elf = elf_calories[i];
        if insert_value > elf {
            let swap_var = elf_calories[i];
            elf_calories[i] = insert_value;
            insert_value = swap_var;
        }
    }
}


fn star_2() -> u32 {
    let path = Path::new("inputs/01");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("Read file successfully"),
    }

    let mut largest_elf_values: [u32; 3] = [0, 0, 0];
    let mut current_elf_value: u32 = 0;

    for line in s.lines() {
        let trimmed_line = line.trim();
        // println!("ln: {trimmed_line}");
        if trimmed_line != "" {
            current_elf_value += trimmed_line.parse::<u32>().unwrap();
        } else {
            replace_elf_value(&mut largest_elf_values, current_elf_value);
            current_elf_value = 0;
        }
    }
    replace_elf_value(&mut largest_elf_values, current_elf_value);

    largest_elf_values[2] + largest_elf_values[1] + largest_elf_values[0]
}


fn main() {
    let value = star_1();
    println!("Largest Number of calories: {value}");
    let value = star_2();
    println!("Largest Number of calories: {value}");
}
