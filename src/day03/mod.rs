fn char_to_u32(c: char) -> u32 {
    if c >= 'a' {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

pub fn part1() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| b
            .chars()
            .filter(|&c| a.contains(c))
            .map(char_to_u32)
            .last()
            .unwrap()
        )
        .sum::<u32>()
}

pub fn part2() -> u32 {
    include_str!("input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|line_trio| line_trio[0]
            .chars()
            .filter(|&c| line_trio[1].contains(c) && line_trio[2].contains(c))
            .map(char_to_u32)
            .last()
            .unwrap()
        )
        .sum::<u32>()
}

pub fn execute(part: u8) {
    let result = match part {
        1 => part1(),
        2 => part2(),
        _ => panic!("This part is not implemented!")
    };

    println!("Day: {} (Part {}) resulted in {}!", 1, part, result);
}
