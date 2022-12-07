pub fn part1() -> u32 {
    include_str!("input.txt")
        .split("\n\n")
        .map(|group| group
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .sum::<u32>()
        )
        .max()
        .unwrap()
}

pub fn part2() -> u32 {
    let mut sums = include_str!("input.txt")
        .split("\n\n")
        .map(|group| group
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .sum::<u32>()
        )
        .collect::<Vec<u32>>();

    sums.sort_unstable();
    sums.into_iter().rev().take(3).sum()
}

pub fn execute(part: u8) {
    let result = match part {
        1 => part1(),
        2 => part2(),
        _ => panic!("This part is not implemented!")
    };

    println!("Day: {} (Part {}) resulted in {}!", 1, part, result);
}
