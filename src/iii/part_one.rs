use std::collections::HashSet;
use std::fs::read_to_string;
use std::io;
use std::path::Path;

const LOWER_ASCII_OFFSET: usize = 96;
const UPPER_ASCII_OFFSET: usize = 38;

pub fn solution<S: AsRef<Path>>(input: S) -> Result<usize, io::Error> {
    let input = read_to_string(input)?;

    let items_list = input
        .strip_suffix("\n")
        .unwrap_or(&input)
        .lines();

    let priority_sum = items_list.fold(0, |acc, list| {
        let (former, latter) = list.split_at(list.len() / 2);

        let mut ch_set = HashSet::new();
        let mut common_ch = None;

        for ch in former.chars() {
            if ch_set.contains(&ch) { continue }

            if latter.contains(ch) {
                common_ch = Some(ch);
                break;
            }

            ch_set.insert(ch);
        }

        if let Some(ch) = common_ch {
            acc + priority(ch)
        } else {
            panic!("Couldn't find common letter in {list}")
        }
    });

    Ok(priority_sum)
}

fn priority(ch: char) -> usize {
    if ch.is_ascii_uppercase() {
        (ch as usize) - UPPER_ASCII_OFFSET
    } else if ch.is_ascii_lowercase() {
        (ch as usize) - LOWER_ASCII_OFFSET
    } else {
        panic!("Provided '{ch}' is not an ASCII alpha char")
    }
}

#[test]
fn test_solution() {
    if let Ok(c) = solution("./test_cases/iii.txt") {
        assert_eq!(c, 157);
    } else {
        assert!(false);
    }
}
