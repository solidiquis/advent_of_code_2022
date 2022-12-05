use std::fs::read_to_string;
use std::io;
use std::path::Path;

pub fn solution<S: AsRef<Path>>(input: S) -> Result<usize, io::Error> {
    let input = read_to_string(input)?;

    let assignments = input
        .strip_suffix("\n")
        .unwrap_or(&input)
        .lines();

    let compute_bounds = |(left, right)| (usize::from_str_radix(left, 10).unwrap(), usize::from_str_radix(right, 10).unwrap());

    let overlaps = assignments.fold(0, |acc, assignment| {
        let (elf_a, elf_b) = assignment.split_once(",")
            .expect("Malformed input");

        let seg_a = elf_a.split_once("-")
            .map(compute_bounds)
            .expect("Malformed input");

        let seg_b = elf_b.split_once("-")
            .map(compute_bounds)
            .expect("Malformed input");

        let (left, right) = if seg_a.0 < seg_b.0 {
            (seg_a, seg_b)
        } else if seg_a.0 > seg_b.0 {
            (seg_b, seg_a)
        } else {
            return acc + 1;
        };

        if left.1 >= right.0 {
            acc + 1
        } else {
            acc
        }
    });

    Ok(overlaps)
}

#[test]
fn test_solution() {
    if let Ok(c) = solution("./test_cases/iv.txt") {
        assert_eq!(c, 4);
    } else {
        assert!(false);
    }
}
