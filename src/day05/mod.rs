pub fn part1() -> String {
    let (config, moves) = include_str!("input.txt").split_once("\n\n").unwrap();

    let amount_of_stacks: usize = config.lines().last().unwrap().len() / 4 + 1;

    let mut stacks= vec![Vec::<char>::new(); amount_of_stacks];

    config
        .lines()
        .rev()
        .skip(1)
        .for_each(|line| line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i].push(c))
        );

    moves
        .lines()
        .map(|line| line
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
        )
        .for_each(|m| {
            for _ in 0 .. m[0] {
                let tmp = stacks[m[1] - 1].pop().unwrap();
                stacks[m[2] - 1].push(tmp);
            }
        });

    stacks.iter().map(|s| *s.last().unwrap()).collect()
}

pub fn part2() -> String {
    let (config, moves) = include_str!("input.txt").split_once("\n\n").unwrap();

    let amount_of_stacks: usize = config.lines().last().unwrap().len() / 4 + 1;

    let mut stacks= vec![Vec::<char>::new(); amount_of_stacks];

    config
        .lines()
        .rev()
        .skip(1)
        .for_each(|line| line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i].push(c))
        );

    moves
        .lines()
        .map(|line| line
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
        )
        .for_each(|m| {
            // This could be done in a more efficient way, but I'm lazy
            let n = stacks[m[1] - 1].len();
            let tmp = stacks[m[1] - 1].drain(n - m[0] .. n).collect::<Vec<char>>();
            stacks[m[2] - 1].extend_from_slice(tmp.as_slice())
        });

    stacks.iter().map(|s| *s.last().unwrap()).collect()
}

pub fn execute(part: u8) {
    let result = match part {
        1 => part1(),
        2 => part2(),
        _ => panic!("This part is not implemented!")
    };

    println!("Day: {} (Part {}) resulted in {}!", 5, part, result)
}
