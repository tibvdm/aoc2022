pub fn part1() -> i32 {
    include_str!("input.txt")
        .lines()
        .map(|line| match line {
            "A Y" => 8,
            "B Z" => 9,
            "C X" => 7,
            "A X" => 4,
            "B Y" => 5,
            "C Z" => 6,
            "A Z" => 3,
            "B X" => 1,
            "C Y" => 2,
            _     => panic!("Invalid input")
        })
        .sum()
}

pub fn part2() -> i32 {
    include_str!("input.txt")
        .lines()
        .map(|line| match line {
            "A X" => 3,
            "B X" => 1,
            "C X" => 2,
            "A Y" => 4,
            "B Y" => 5,
            "C Y" => 6,
            "A Z" => 8,
            "B Z" => 9,
            "C Z" => 7,
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

    println!("Day: {} (Part {}) resulted in {}!", 1, part, result);
}
