
pub fn star_1() -> u32 {
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
