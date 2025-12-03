use std::fs;

fn main() {
    let banks: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect();

    let result = all_joltages(&banks);
    let result_pt2 = all_joltages_pt2(&banks);
    println!("{result}");
    println!("{result_pt2}");
}

fn all_joltages(banks: &Vec<String>) -> u64 {
    banks
        .iter()
        .fold(0, |acc, cur| acc + largest_joltage(cur, 2))
}

fn all_joltages_pt2(banks: &Vec<String>) -> u64 {
    banks
        .iter()
        .fold(0, |acc, cur| acc + largest_joltage(cur, 12))
}

fn largest_joltage(s: &str, length: usize) -> u64 {
    let digits: Vec<u8> = s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

    let mut start_at = 0;
    let mut result = Vec::new();
    for i in 0..length {
        let end_at = digits.len() - (length - i - 1);

        let largest = *digits[start_at..end_at].iter().max().unwrap();

        result.push(largest);

        let largest_index = &digits[start_at..end_at]
            .iter()
            .position(|d| *d == largest)
            .unwrap()
            + start_at;
        start_at = largest_index + 1;
    }

    vec_to_int(result)
}

fn vec_to_int(v: Vec<u8>) -> u64 {
    let result_string = v
        .iter()
        .map(|u| u.to_string())
        .collect::<Vec<String>>()
        .join("");

    result_string.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_joltage_pt2_test() {
        assert_eq!(largest_joltage("987654321111111", 12), 987654321111);
        assert_eq!(largest_joltage("811111111111119", 12), 811111111119);
        assert_eq!(largest_joltage("234234234234278", 12), 434234234278);
        assert_eq!(largest_joltage("818181911112111", 12), 888911112111);
    }

    #[test]
    fn largest_joltage_test() {
        assert_eq!(largest_joltage("987654321111111", 2), 98);
        assert_eq!(largest_joltage("811111111111119", 2), 89);
        assert_eq!(largest_joltage("234234234234278", 2), 78);
        assert_eq!(largest_joltage("818181911112111", 2), 92);
    }

    #[test]
    fn all_joltages_test() {
        assert_eq!(
            all_joltages(&vec![
                "987654321111111".to_string(),
                "811111111111119".to_string(),
                "234234234234278".to_string(),
                "818181911112111".to_string()
            ]),
            357
        );
    }
}
