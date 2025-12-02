use std::{fs, ops::Range};

fn main() {
    let ranges = fs::read_to_string("./input.txt").unwrap();

    let sum = process_string(&ranges);
    println!("{sum}");
}

fn process_string(s: &str) -> i64 {
    sum_all_invalid(s.split(",").collect())
}

fn sum_all_invalid(s: Vec<&str>) -> i64 {
    s.iter()
        .fold(0, |acc, cur| acc + get_all_invalid(cur).iter().sum::<i64>())
}

fn get_all_invalid(s: &str) -> Vec<i64> {
    get_invalid_values_from_range(enumerate_range(s))
}

fn enumerate_range(s: &str) -> Range<i64> {
    let (start, end) = s.split_once("-").unwrap();
    let s: i64 = start.parse().expect(&format!("could not parse {}", start));
    let e: i64 = end.parse().expect(&format!("could not parse {}", end));

    s..e + 1
}

fn get_invalid_values_from_range(r: Range<i64>) -> Vec<i64> {
    r.filter(|n| !is_valid(&n.to_string())).collect()
}

fn is_valid(id: &str) -> bool {
    if id.len() % 2 == 1 {
        true
    } else {
        let (a, b) = id.split_at(id.len() / 2);
        a != b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_inputs() {
        assert!(!is_valid("11"));
        assert!(!is_valid("22"));
        assert!(!is_valid("1010"));
        assert!(!is_valid("1188511885"));
        assert!(!is_valid("38593859"));

        assert!(is_valid("12"));
        assert!(is_valid("21"));
        assert!(is_valid("1012"));
        assert!(is_valid("1698528"));
        assert!(is_valid("38593862"));
    }

    #[test]
    fn invalid_ranges() {
        assert_eq!(get_all_invalid("11-22"), vec![11, 22]);
        assert_eq!(get_all_invalid("95-115"), vec![99]);
        assert_eq!(get_all_invalid("998-1012"), vec![1010]);
        assert_eq!(get_all_invalid("1188511880-1188511890"), vec![1188511885]);
        assert_eq!(get_all_invalid("222220-222224"), vec![222222]);
        assert_eq!(get_all_invalid("1698522-1698528"), vec![]);
        assert_eq!(get_all_invalid("446443-446449"), vec![446446]);
        assert_eq!(get_all_invalid("38593856-38593862"), vec![38593859]);
    }

    #[test]
    fn total_sum() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(process_string(input), 1227775554);
    }
}
