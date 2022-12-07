use std::collections::hash_set::HashSet;
use std::fs::read;
use std::io;
use std::path::Path;

const MARKER_LEN: usize = 4;

pub fn solution<S: AsRef<Path>>(input: S) -> Result<usize, io::Error> {
    let input = read(input)?;

    let mut set = HashSet::new();

    'outer: for i in 0..(input.len() - MARKER_LEN) {
        for byte in &input[i..(i + MARKER_LEN)] {
            if !set.insert(*byte) {
                set.clear();
                continue 'outer;
            }
        }
        return Ok(i + MARKER_LEN);
    }

    panic!("Marker not found.")
}

#[test]
fn test_solution() {
    if let Ok(c) = solution("./test_cases/vi.txt") {
        assert_eq!(c, 11);
    } else {
        assert!(false);
    }
}
