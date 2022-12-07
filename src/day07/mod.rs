pub fn part1() -> u32 {
    0
}

pub fn part2() -> u32 {
    0
}

pub fn execute(part: u8) {
    let result = match part {
        1 => part1(),
        2 => part2(),
        _ => panic!("This part is not implemented!")
    };

    println!("Day: {} (Part {}) resulted in {}!", 7, part, result)
}
