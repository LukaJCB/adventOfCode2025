use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect();

    let result = solve_problem(&input);

    let result_pt2 = solve_problem_pt2(&input);

    println!("{result}");
    println!("{result_pt2}");
}

fn solve_problem(rows: &Vec<String>) -> u64 {
    let (numbers, last) = rows.split_at(rows.len() - 1);

    let flatted: Vec<u64> = numbers.iter().flat_map(|s: &String| parse_row(s)).collect();

    let instructions = parse_instruction_row(&last[0]);

    let y_len = rows.len() - 1;
    let x_len = flatted.len() / y_len;

    let mut results = Vec::new();

    let mut x = 0;
    while x < x_len {
        let mut y = 0;
        let instruction = &instructions[x];

        let mut result = match instruction {
            Instruction::Add => 0,
            Instruction::Multiply => 1,
        };
        while y < y_len {
            let index = y * x_len + x;
            let n = flatted[index];

            match instruction {
                Instruction::Add => {
                    result += n;
                }
                Instruction::Multiply => {
                    result *= n;
                }
            }

            y += 1
        }
        results.push(result);
        x += 1;
    }

    results.iter().sum()
}

fn parse_row(row: &str) -> Vec<u64> {
    row.split_ascii_whitespace()
        .map(|n| n.parse().expect(&format!("could not parse {}", n)))
        .collect()
}

fn parse_instruction_row(row: &str) -> Vec<Instruction> {
    row.split_ascii_whitespace()
        .map(|n| {
            if n == "*" {
                Instruction::Multiply
            } else {
                Instruction::Add
            }
        })
        .collect()
}

fn solve_problem_pt2(rows: &Vec<String>) -> u64 {
    let (numbers, last) = rows.split_at(rows.len() - 1);

    let instructions = parse_instruction_row_pt2(&last[0]);

    // outer is the row
    // 2nd is the horizontal number strings
    // inner is the individual digits
    let parsed: Vec<Vec<Vec<Option<u32>>>> = numbers
        .iter()
        .map(|s: &String| parse_row_pt2(s, &instructions))
        .collect();

    let flatted: Vec<&Vec<Option<u32>>> = parsed.iter().flatten().collect();

    let y_len = rows.len() - 1;
    let x_len = flatted.len() / y_len;

    let mut results = Vec::new();

    let mut x = 0;
    while x < x_len {
        let mut y = 0;
        let (instruction, _) = &instructions[x];

        let mut block: Vec<Vec<Option<u32>>> = Vec::new();
        while y < y_len {
            let index = y * x_len + x;
            let n = flatted[index];

            block.push(n.clone());

            match instruction {
                Instruction::Add => {
                    ();
                }
                Instruction::Multiply => {
                    ();
                }
            }

            y += 1
        }
        let numbers = construct_numbers(&block);
        let inital: u64 = match instruction {
            Instruction::Add => 0,
            Instruction::Multiply => 1,
        };
        let result = numbers.iter().fold(inital, |acc, cur| match instruction {
            Instruction::Add => acc + (*cur as u64),
            Instruction::Multiply => acc * (*cur as u64),
        });
        results.push(result);
        x += 1;
    }

    results.iter().sum()
}

fn parse_instruction_row_pt2(row: &str) -> Vec<(Instruction, usize)> {
    let mut result = Vec::new();

    let mut current_instruction = None;
    let mut current_length = 0;
    for char in row.chars() {
        if char == '*' {
            if let Some(ci) = current_instruction {
                result.push((ci, current_length - 1));
                current_length = 0;
            }
            current_instruction = Some(Instruction::Multiply);
        } else if char == '+' {
            if let Some(ci) = current_instruction {
                result.push((ci, current_length - 1));
                current_length = 0;
            }
            current_instruction = Some(Instruction::Add);
        }
        current_length += 1;
    }
    if let Some(ci) = current_instruction {
        result.push((ci, current_length));
    }
    result
}

fn parse_row_pt2(row: &str, lengths: &Vec<(Instruction, usize)>) -> Vec<Vec<Option<u32>>> {
    let chars: Vec<char> = row.chars().collect();

    let mut result = Vec::new();
    let mut offset = 0;
    for (_i, len) in lengths {
        let slice = &chars[offset..offset + len];
        let vec: Vec<Option<u32>> = slice
            .iter()
            .map(|c| {
                if c.is_ascii_whitespace() {
                    None
                } else {
                    c.to_digit(10)
                }
            })
            .collect();

        result.push(vec);
        offset += len + 1;
    }
    result
}

fn construct_numbers(arr: &Vec<Vec<Option<u32>>>) -> Vec<u32> {
    let flatted: Vec<&Option<u32>> = arr.iter().flatten().collect();

    let y_len = arr.len();
    let x_len = flatted.len() / y_len;

    let mut results: Vec<u32> = Vec::new();

    let mut x = 0;
    while x < x_len {
        let mut y = 0;

        let mut result = "".to_string();
        while y < y_len {
            let index = y * x_len + x;
            let n = flatted[index];

            match n {
                Some(a) => result.push_str(&a.to_string()),
                None => (),
            }

            y += 1
        }
        results.push(result.parse().unwrap());
        x += 1;
    }

    results
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Multiply,
    Add,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_row_test() {
        let row = "132 9   56 13";

        let nums: Vec<u64> = parse_row(row);

        assert_eq!(nums, vec![132, 9, 56, 13]);
    }

    #[test]
    fn parse_row_test_part_2() {
        let row = "123 328  51 64 ";

        let nums: Vec<Vec<Option<u32>>> = parse_row_pt2(
            row,
            &vec![
                (Instruction::Add, 3),
                (Instruction::Add, 3),
                (Instruction::Add, 3),
                (Instruction::Add, 3),
            ],
        );

        assert_eq!(
            nums,
            vec![
                vec![Some(1), Some(2), Some(3)],
                vec![Some(3), Some(2), Some(8)],
                vec![None, Some(5), Some(1)],
                vec![Some(6), Some(4), None]
            ]
        );
    }

    #[test]
    fn parse_instruction_row_test_part_2() {
        assert_eq!(
            parse_instruction_row_pt2("*   +   *   +  "),
            vec![
                (Instruction::Multiply, 3),
                (Instruction::Add, 3),
                (Instruction::Multiply, 3),
                (Instruction::Add, 3)
            ]
        );

        assert_eq!(
            parse_instruction_row_pt2("*   +   +    * "),
            vec![
                (Instruction::Multiply, 3),
                (Instruction::Add, 3),
                (Instruction::Add, 4),
                (Instruction::Multiply, 2)
            ]
        );
    }

    #[test]
    fn construct_numbers_test() {
        let nums: Vec<Vec<Option<u32>>> = vec![
            vec![Some(6), Some(4), None],
            vec![Some(2), Some(3), None],
            vec![Some(3), Some(1), Some(4)],
        ];

        assert_eq!(construct_numbers(&nums), vec![623, 431, 4]);

        let nums2: Vec<Vec<Option<u32>>> = vec![
            vec![Some(3), Some(2), Some(8)],
            vec![Some(6), Some(4), None],
            vec![Some(9), Some(8), None],
        ];

        assert_eq!(construct_numbers(&nums2), vec![369, 248, 8]);
    }

    #[test]
    fn solve_problem_test() {
        let problem = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(
            solve_problem(&problem.split("\n").map(String::from).collect()),
            4277556
        );
    }
}
