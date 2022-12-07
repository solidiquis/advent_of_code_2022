use std::collections::hash_set::HashSet;
use std::fs::read;
use std::io;
use std::path::Path;

const MSG_LEN: usize = 14;

pub fn solution<S: AsRef<Path>>(input: S) -> Result<usize, io::Error> {
    let input = read(input)?;

    let mut set = HashSet::new();

    'outer: for i in 0..(input.len() - MSG_LEN) {
        for byte in &input[i..(i + MSG_LEN)] {
            if !set.insert(*byte) {
                set.clear();
                continue 'outer;
            }
        }
        return Ok(i + MSG_LEN);
    }

    panic!("Marker not found.")
}

#[test]
fn test_solution() {
    if let Ok(c) = solution("./test_cases/vi.txt") {
        assert_eq!(c, 26);
    } else {
        assert!(false);
    }
}
