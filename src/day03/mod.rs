fn byte_to_u32(&c: &u8) -> u32 {
    if c >= b'a' {
        c as u32 - b'a' as u32 + 1
    } else {
        c as u32 - b'A' as u32 + 27
    }
}

pub fn part1() -> u32 {
    include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| b
            .iter()
            .find(|&c| a.contains(c))
            .map(byte_to_u32)
            .unwrap()
        )
        .sum::<u32>()
}

pub fn part2() -> u32 {
    include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|line_trio| line_trio[0]
            .iter()
            .find(|&c| line_trio[1].contains(c) && line_trio[2].contains(c))
            .map(byte_to_u32)
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

    println!("Day: {} (Part {}) resulted in {}!", 3, part, result);
}
