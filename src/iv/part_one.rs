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

        let (larger_seg, smaller_seg) = if seg_a.1 - seg_a.0 > seg_b.1 - seg_b.0 {
            (seg_a, seg_b)
        } else if seg_a.1 - seg_a.0 < seg_b.1 - seg_b.0 {
            (seg_b, seg_a)
        } else {
            if seg_a.0 == seg_b.0 && seg_a.1 == seg_b.1 {
                return acc + 1;
            } else {
                return acc; 
            }
        };

        if smaller_seg.0 >= larger_seg.0 && smaller_seg.1 <= larger_seg.1 {
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
        assert_eq!(c, 2);
    } else {
        assert!(false);
    }
}
