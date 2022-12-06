use instructions::Instruction;
use std::fs::read_to_string;
use std::io;
use std::path::Path;

pub fn solution<S: AsRef<Path>>(input: S) -> Result<String, io::Error> {
    let (mut stacks, instructions) = parse_input(input)?;

    instructions.into_iter().for_each(|i| i.execute(&mut stacks));

    let answer = stacks.into_iter()
        .map(|stack| stack[stack.len() - 1])
        .fold("".to_owned(), |mut acc, item| {
            acc.push(item);
            acc
        });

    Ok(answer)
}

fn parse_input<S: AsRef<Path>>(input: S) -> Result<(Vec<Vec<char>>, Vec<Instruction>), io::Error> {
    let input = read_to_string(input)?;

    let (stacks_txt, instructions_txt) = input.split_once("\n\n")
        .expect("Malformed input");

    let stacks = stacks_from_raw_txt(stacks_txt);
    let instructions = instructions_from_raw_txt(instructions_txt);
    
    Ok((stacks, instructions))
}

fn stacks_from_raw_txt(raw_txt: &str) -> Vec<Vec<char>> {
    let mut rev_stacks_txt = raw_txt.lines().rev();

    let stack_numbers = rev_stacks_txt.next().expect("Malformed input");

    let mut num_stacks = None;

    for i in stack_numbers.rsplit("") {
        if let Ok(num) = i.parse::<usize>() {
            num_stacks = Some(num);
            break;
        }
    }

    let mut stacks = num_stacks
        .map(|num| Vec::with_capacity(num))
        .and_then(|mut stacks| {
            for _ in 0..stacks.capacity() {
                stacks.push(vec![])
            }

            Some(stacks)
        })
        .expect("Malformed input");

    for row in rev_stacks_txt {
        let mut i = 0;

        for ch in row.chars().skip(1).step_by(4) {
            let stack = stacks.get_mut(i).expect("Index out of bounds");

            if ch.is_alphabetic() { stack.push(ch) }

            i += 1;
        }
    }

    stacks
}

fn instructions_from_raw_txt(raw_txt: &str) -> Vec<Instruction> {
    raw_txt
        .lines()
        .map(|ln| Instruction::from_raw_txt(ln))
        .collect()
}

mod instructions {
    #[derive(Debug)]
    pub struct Instruction {
        amount_to_move: usize,
        from: usize,
        to: usize
    }

    impl Instruction {
        pub fn from_raw_txt(txt: &str) -> Self {
            let fields = txt.split(" ")
                .filter_map(|sub| sub.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            assert!(fields.len() == 3, "Malformed input");

            Self {
                amount_to_move: fields[0],
                // zero-based indexing
                from: fields[1] - 1,
                to: fields[2] - 1
            }
        }

        pub fn execute(&self, stacks: &mut Vec<Vec<char>>) {
            for _ in 0..self.amount_to_move {
                let item = stacks.get_mut(self.from)
                    .map(|stack| stack.pop())
                    .flatten()
                    .expect("Index out of bounds.");

                stacks.get_mut(self.to)
                    .map(|stack| stack.push(item))
                    .expect("Index out of bounds.");
            }
        }
    }
}

#[test]
fn test_solution() {
    if let Ok(c) = solution("./test_cases/v.txt") {
        assert_eq!(c, "CMZ".to_owned());
    } else {
        assert!(false);
    }
}
