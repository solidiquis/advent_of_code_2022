use std::io;
use std::iter::IntoIterator;
use std::fs::read_to_string;
use std::path::Path;

pub fn solution<S: AsRef<Path>>(input: S) -> Result<usize, io::Error> {
    let mut top_three: [usize; 3] = [0, 0, 0];

    read_to_string(input)?.split("\n").fold(0, |mut acc, item| {
        if item.is_empty() {
            let (i, min) = compute_min_with_index(&top_three);
            if acc > min { top_three[i] = acc }
            acc = 0;
            acc
        } else {
            acc + usize::from_str_radix(item, 10)
                .expect(&format!("Failed to convert {item} to usize base 10"))
        }
    });

    let total = top_three.into_iter()
        .reduce(|acc, item| acc + item)
        .expect("Failed to sum top 3");

    Ok(total)
}

fn compute_min_with_index(top: &[usize; 3]) -> (usize, usize) {
    let mut min = usize::MAX;
    let mut min_i = usize::MAX;

    for (i, j) in top.into_iter().enumerate() {
        if *j < min {
            min = j.clone();
            min_i = i.clone();
        }
    }

    (min_i, min)
}

#[test]
fn test_solutions() {
    if let Ok(c) = solution("./test_cases/i.txt") {
        assert_eq!(c, 45);
    } else {
        assert!(false);
    }
}
