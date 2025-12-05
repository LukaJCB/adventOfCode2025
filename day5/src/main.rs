use std::{collections::HashSet, fs, ops::RangeInclusive};

fn main() {
    let input: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect();

    let (ingredients, ranges) = parse_input(input);
    let result = total_fresh_ingredients(&ingredients, &ranges);

    let result_pt2 = size_of_range(merge_ranges(&ranges));

    println!("{result}");
    println!("{}", result_pt2);
}

fn parse_input(lines: Vec<String>) -> (Vec<u64>, Vec<RangeInclusive<u64>>) {
    let vec: Vec<&[String]> = lines.split(|s| s == "").collect();

    let ingredients = vec[1]
        .iter()
        .map(|s| s.parse().expect(&format!("Could not parse {}", s)))
        .collect();
    let ranges = vec[0].iter().map(parse_range).collect();

    (ingredients, ranges)
}

fn size_of_range(ranges: Vec<RangeInclusive<u64>>) -> u64 {
    let mut result: u64 = 0;
    for range in ranges {
        result += range.end() + 1 - range.start()
    }
    result
}

fn merge_ranges(ranges: &Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();
    let mut sorted = ranges.clone();
    sorted.sort_by_key(|r| *r.start());

    for range in sorted {
        if let Some(last) = merged.last_mut() {
            if range.start() <= last.end() {
                let new_end = (*last.end()).max(*range.end());
                *last = *last.start()..=new_end;
                continue;
            }
        }
        merged.push(range);
    }

    merged
}

fn parse_range(s: &String) -> RangeInclusive<u64> {
    let (start, end) = s.split_once("-").unwrap();

    start.parse().unwrap()..=end.parse().unwrap()
}

fn total_fresh_ingredients(ingredients: &Vec<u64>, ranges: &Vec<RangeInclusive<u64>>) -> u32 {
    let mut result = 0;
    for ingredient in ingredients.iter() {
        for range in ranges.iter() {
            if range.contains(ingredient) {
                result += 1;
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_fresh_ingredients_test() {
        let ingredients = vec![1, 5, 8, 11, 17, 32];
        let ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];
        assert_eq!(total_fresh_ingredients(&ingredients, &ranges), 3);
    }

    #[test]
    fn merge_ranges_test() {
        let ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];
        assert_eq!(merge_ranges(&ranges), vec![3..=5, 10..=20]);
    }
}
