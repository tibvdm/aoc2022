pub fn part1() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| (a.split_once("-").unwrap(), b.split_once("-").unwrap()))
        .map(|((a, b), (c, d))| (
            a.parse::<u32>().unwrap(), 
            b.parse::<u32>().unwrap(), 
            c.parse::<u32>().unwrap(), 
            d.parse::<u32>().unwrap())
        )
        .filter(|(a, b, c, d)| (c <= a && b <= d) || (a <= c && d <= b))
        .count() as u32
}

pub fn part2() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| (a.split_once("-").unwrap(), b.split_once("-").unwrap()))
        .map(|((a, b), (c, d))| (
            a.parse::<u32>().unwrap(), 
            b.parse::<u32>().unwrap(), 
            c.parse::<u32>().unwrap(), 
            d.parse::<u32>().unwrap())
        )
        .filter(|(a, b, c, d)| a <= d && c <= b)
        .count() as u32
}

pub fn execute(part: u8) {
    let result = match part {
        1 => part1(),
        2 => part2(),
        _ => panic!("This part is not implemented!")
    };

    println!("Day: {} (Part {}) resulted in {}!", 4, part, result);
}
