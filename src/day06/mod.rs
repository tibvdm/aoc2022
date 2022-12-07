fn find_marker(marker_size: usize) -> u32 {
    include_str!("input.txt")
        .as_bytes()
        .windows(marker_size)
        .take_while(|&w| 
            (0 .. marker_size - 1).any(|i| (i + 1 .. marker_size).any(|j| w[i] == w[j]))
        )
        .count() as u32 + marker_size as u32
}

pub fn part1() -> u32 {
    find_marker(4)
}

pub fn part2() -> u32 {
    find_marker(14)
}

pub fn execute(part: u8) {
    let result = match part {
        1 => part1(),
        2 => part2(),
        _ => panic!("This part is not implemented!")
    };

    println!("Day: {} (Part {}) resulted in {}!", 6, part, result)
}
