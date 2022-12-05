use std::io;
use std::fs::read_to_string;
use std::path::Path;

pub fn solution<S: AsRef<Path>>(input: S) -> Result<usize, io::Error> {
    let mut max_calories = 0;

    read_to_string(input)?.split("\n").fold(0, |acc, item| {
        if item.is_empty() {
            if acc > max_calories { max_calories = acc }
            0
        } else {
            acc + usize::from_str_radix(item, 10).expect("Line is not a number")
        }
    });

    Ok(max_calories)
}

#[test]
fn test_solutions() {
    if let Ok(c) = solution("./test_cases/i.txt") {
        assert_eq!(c, 21);
    } else {
        assert!(false);
    }
}
