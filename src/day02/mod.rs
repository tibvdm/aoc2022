pub fn part1() -> i32 {
    include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .map(|line| match line {
            b"A Y" => 8,
            b"B Z" => 9,
            b"C X" => 7,
            b"A X" => 4,
            b"B Y" => 5,
            b"C Z" => 6,
            b"A Z" => 3,
            b"B X" => 1,
            b"C Y" => 2,
            _     => panic!("Invalid input")
        })
        .sum()
}

pub fn part2() -> i32 {
    include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .map(|line| match line {
            b"A X" => 3,
            b"B X" => 1,
            b"C X" => 2,
            b"A Y" => 4,
            b"B Y" => 5,
            b"C Y" => 6,
            b"A Z" => 8,
            b"B Z" => 9,
            b"C Z" => 7,
            _     => panic!("Invalid input")
        })
        .sum()
}

pub fn execute(part: u8) {
    let result = match part {
        1 => part1(),
        2 => part2(),
        _ => panic!("This part is not implemented!")
    };

    println!("Day: {} (Part {}) resulted in {}!", 2, part, result);
}
