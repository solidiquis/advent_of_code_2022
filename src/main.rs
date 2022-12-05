use std::io;
use std::fmt::{self, Display};
use std::thread;

/// https://adventofcode.com/2022/day/1
mod i;

/// https://adventofcode.com/2022/day/2
mod ii;

/// https://adventofcode.com/2022/day/3
mod iii;

/// https://adventofcode.com/2022/day/4
mod iv;

fn main() {
    let threads = vec![
        thread::spawn(|| Solution::new(1, 1, i::part_one::solution("./input/i.txt"))),
        thread::spawn(|| Solution::new(1, 2, i::part_two::solution("./input/i.txt"))),
        thread::spawn(|| Solution::new(2, 1, ii::part_one::solution("./input/ii.txt"))),
        thread::spawn(|| Solution::new(2, 2, ii::part_two::solution("./input/ii.txt"))),
        thread::spawn(|| Solution::new(3, 1, iii::part_one::solution("./input/iii.txt"))),
        thread::spawn(|| Solution::new(3, 2, iii::part_two::solution("./input/iii.txt", 3))),
        thread::spawn(|| Solution::new(4, 1, iv::part_one::solution("./input/iv.txt"))),
        thread::spawn(|| Solution::new(4, 2, iv::part_two::solution("./input/iv.txt"))),
    ];

    threads.into_iter()
        .map(|thread| thread.join().expect("Thread panicked"))
        .for_each(|solution| println!("{solution}"));
}

struct Solution {
    day: u8,
    part: u8,
    solution: String
}

impl Solution {
    fn new<T: Display>(day: u8, part: u8, possible_solution: Result<T, io::Error>) -> Self {
        let solution = possible_solution
            .map(|answer| format!("{answer}"))
            .unwrap_or_else(|_e| "ERROR".to_owned());

        Solution { day, part, solution }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {}, part {} => \x1b[32m{}\x1b[0m", self.day, self.part, self.solution)
    }
}

