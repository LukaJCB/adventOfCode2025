use std::{fs, ops::Range};

fn main() {
    let ranges = fs::read_to_string("./input.txt").unwrap();

    let sum = process_string(&ranges);
    let sum_pt2 = process_string_pt2(&ranges);
    println!("{sum}");
    println!("{sum_pt2}");
}

fn enumerate_range(s: &str) -> Range<i64> {
    let (start, end) = s.split_once("-").unwrap();
    let s: i64 = start.parse().expect(&format!("could not parse {}", start));
    let e: i64 = end.parse().expect(&format!("could not parse {}", end));

    s..e + 1
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

fn process_string_pt2(s: &str) -> i64 {
    sum_all_invalid_pt2(s.split(",").collect())
}

fn sum_all_invalid_pt2(s: Vec<&str>) -> i64 {
    s.iter().fold(0, |acc, cur| {
        acc + get_all_invalid_pt2(cur).iter().sum::<i64>()
    })
}

fn get_all_invalid_pt2(s: &str) -> Vec<i64> {
    get_invalid_values_from_range_pt2(enumerate_range(s))
}

fn get_invalid_values_from_range_pt2(r: Range<i64>) -> Vec<i64> {
    r.filter(|n| !is_valid_pt2(&n.to_string())).collect()
}

fn is_valid_pt2(id: &str) -> bool {
    let len = id.len();
    let sizes = find_divisors(len);

    sizes.iter().all(|size| valid_under_size(id, *size))
}

fn valid_under_size(id: &str, size: usize) -> bool {
    !all_strings_equal(split_string(size, id))
}

fn find_divisors(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 1..=n / 2 {
        if n % i == 0 {
            res.push(i);
        }
    }
    res
}

fn split_string(n: usize, s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();

    chars
        .chunks(n)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn all_strings_equal(s: Vec<String>) -> bool {
    let (first, rest) = s.split_at(1);
    let base = first.get(0).unwrap();
    for s in rest {
        if s != base {
            return false;
        }
    }

    true
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

    #[test]
    fn divisors() {
        assert_eq!(find_divisors(12), vec![1, 2, 3, 4, 6]);
        assert_eq!(find_divisors(20), vec![1, 2, 4, 5, 10]);
        assert_eq!(find_divisors(10), vec![1, 2, 5]);
        assert_eq!(find_divisors(9), vec![1, 3]);
        assert_eq!(find_divisors(7), vec![1]);
        assert_eq!(find_divisors(1), vec![]);
    }

    #[test]
    fn string_split() {
        assert_eq!(split_string(2, "222222"), vec!["22", "22", "22"]);
        assert_eq!(split_string(3, "222222"), vec!["222", "222"]);
        assert_eq!(split_string(4, "12341234"), vec!["1234", "1234"]);
    }

    #[test]
    fn all_strings_equal_test() {
        assert!(all_strings_equal(
            vec!["22", "22", "22"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ));
        assert!(all_strings_equal(
            vec!["222", "222"].iter().map(|s| s.to_string()).collect()
        ));
        assert!(all_strings_equal(
            vec!["1234", "1234"].iter().map(|s| s.to_string()).collect()
        ));
        assert!(all_strings_equal(
            vec!["1", "1", "1", "1"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ));

        assert!(!all_strings_equal(
            vec!["1", "1", "1", "2"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ));
        assert!(!all_strings_equal(
            vec!["1", "1", "2", "1"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ));
        assert!(!all_strings_equal(
            vec!["1234", "123"].iter().map(|s| s.to_string()).collect()
        ));
    }

    #[test]
    fn valid_under_size_test() {
        assert!(!valid_under_size("12341234", 4));
        assert!(!valid_under_size("123123123", 3));
        assert!(!valid_under_size("1212121212", 2));
        assert!(!valid_under_size("1111111", 1));

        assert!(valid_under_size("12341234", 2));
        assert!(valid_under_size("123123123", 4));
        assert!(valid_under_size("1212121212", 1));
    }

    #[test]
    fn invalid_inputs_pt2() {
        assert!(!is_valid_pt2("12341234"));
        assert!(!is_valid_pt2("123123123"));
        assert!(!is_valid_pt2("1212121212"));
        assert!(!is_valid_pt2("1111111"));

        assert!(is_valid_pt2("12"));
        assert!(is_valid_pt2("21"));
        assert!(is_valid_pt2("1012"));
        assert!(is_valid_pt2("1698528"));
        assert!(is_valid_pt2("38593862"));
    }

    #[test]
    fn invalid_ranges_pt2() {
        assert_eq!(get_all_invalid_pt2("11-22"), vec![11, 22]);
        assert_eq!(get_all_invalid_pt2("95-115"), vec![99, 111]);
        assert_eq!(get_all_invalid_pt2("998-1012"), vec![999, 1010]);
        assert_eq!(
            get_all_invalid_pt2("1188511880-1188511890"),
            vec![1188511885]
        );
        assert_eq!(get_all_invalid_pt2("222220-222224"), vec![222222]);
        assert_eq!(get_all_invalid_pt2("1698522-1698528"), vec![]);
        assert_eq!(get_all_invalid_pt2("446443-446449"), vec![446446]);
        assert_eq!(get_all_invalid_pt2("38593856-38593862"), vec![38593859]);

        assert_eq!(get_all_invalid_pt2("565653-565659"), vec![565656]);
        assert_eq!(get_all_invalid_pt2("824824821-824824827"), vec![824824824]);
        assert_eq!(
            get_all_invalid_pt2("2121212118-2121212124"),
            vec![2121212121]
        );
    }

    #[test]
    fn total_sum_pt2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(process_string_pt2(input), 4174379265);
    }
}
