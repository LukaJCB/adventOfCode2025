use std::fs;

fn main() {
    let banks: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect();

    let result = all_joltages(banks);
    println!("{result}")
}

fn all_joltages(banks: Vec<String>) -> u32 {
    banks.iter().fold(0, |acc, cur| acc + largest_joltage(cur))
}

fn largest_joltage(s: &str) -> u32 {
    let digits: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut largest = 0;
    for d in digits.iter().take(s.len() - 1) {
        if *d >= largest {
            largest = *d;
        }
    }

    let largest_index = digits.iter().position(|d| *d == largest).unwrap();
    let mut largest_second = 0;
    for d in digits.iter().skip(largest_index + 1) {
        if *d >= largest_second {
            largest_second = *d;
        }
    }
    return largest * 10 + largest_second;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_joltage_test() {
        assert_eq!(largest_joltage("987654321111111"), 98);
        assert_eq!(largest_joltage("811111111111119"), 89);
        assert_eq!(largest_joltage("234234234234278"), 78);
        assert_eq!(largest_joltage("818181911112111"), 92);
    }

    #[test]
    fn all_joltages_test() {
        assert_eq!(
            all_joltages(vec![
                "987654321111111".to_string(),
                "811111111111119".to_string(),
                "234234234234278".to_string(),
                "818181911112111".to_string()
            ]),
            357
        );
    }
}
