pub fn part1() -> u32 {
    include_str!("input.txt")
        .as_bytes()
        .windows(4)
        .take_while(|&w| 
            w[0] == w[1] || w[0] == w[2] || w[0] == w[3] || w[1] == w[2] || w[1] == w[3] || w[2] == w[3]
        )
        .count() as u32 + 4
}

pub fn part2() -> u32 {
    include_str!("input.txt")
        .as_bytes()
        .windows(14)
        .take_while(|&w| 
            (0 .. 13).any(|i| (i + 1 .. 14).any(|j| w[i] == w[j]))
        )
        .count() as u32 + 14
}

pub fn execute(part: u8) {
    let result = match part {
        1 => part1(),
        2 => part2(),
        _ => panic!("This part is not implemented!")
    };

    println!("Day: {} (Part {}) resulted in {}!", 6, part, result)
}
