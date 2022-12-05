use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io;
use std::path::Path;

const LOWER_ASCII_OFFSET: usize = 96;
const UPPER_ASCII_OFFSET: usize = 38;

pub fn solution<S: AsRef<Path>>(input: S, elves_per_group: usize) -> Result<usize, io::Error> {
    let raw_items = read_to_string(input)?;

    let items_list = raw_items.strip_suffix("\n").unwrap_or(&raw_items);

    let mut line_items = items_list.lines();
    let num_lines = line_items.clone().count();
    
    assert!(
        num_lines % elves_per_group == 0,
        "Argument 'num_groups' not factor of number of lines of 'input'."
    );

    let num_groups = num_lines / elves_per_group;

    let priority_sum = (0..num_groups).fold(0, |acc, group_num| {
        let mut common_char_count_map: HashMap<char, usize> = HashMap::new();
        let mut common_char = None;

        for _ in 0..elves_per_group {
            let line = line_items.next().expect("Failed to get next line");

            let mut ch_set = HashSet::new();

            for ch in line.chars() {
                if ch_set.contains(&ch) { continue }

                ch_set.insert(ch);

                let count = common_char_count_map.entry(ch)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                if *count == elves_per_group {
                    common_char = Some(ch);
                    break;
                }
            }
        }

        if let Some(ch) = common_char {
            acc + priority(ch)
        } else {
            panic!("Unable to determine item type for a group {group_num}.")
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
    if let Ok(c) = solution("./test_cases/iii.txt", 3) {
        assert_eq!(c, 70);
    } else {
        assert!(false);
    }
}
