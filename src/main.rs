use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: u8,

    #[arg(short, long, default_value_t = 1)]
    part: u8
}

fn main() {
    let args = Args::parse();
    
    match args.day {
        1 => day01::execute(args.part),
        2 => day02::execute(args.part),
        3 => day03::execute(args.part),
        4 => day04::execute(args.part),
        5 => day05::execute(args.part),
        6 => day06::execute(args.part),
        7 => day07::execute(args.part),
        _ => panic!("Day not implemented")
    }
}
