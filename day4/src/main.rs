use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect();

    let result = total_forklift_accesible(&input);

    let result_pt2 = remove_total_forklift_accesible(&input);

    println!("{result}");
    println!("{result_pt2}");
}

fn total_forklift_accesible(input: &Vec<String>) -> usize {
    let mut result = 0;
    let grid = parse_input(input);
    for (x, xs) in grid.iter().enumerate() {
        for (y, is_roll) in xs.iter().enumerate() {
            if *is_roll && forklift_accesible(x, y, &grid) {
                result += 1;
            }
        }
    }
    result
}

fn remove_total_forklift_accesible(input: &Vec<String>) -> usize {
    let mut grid = parse_input(input);

    let mut total_removed = 0;
    while let Some(i) = removal_round(&mut grid) {
        total_removed += i;
    }

    total_removed
}

fn removal_round(grid: &mut Vec<Vec<bool>>) -> Option<usize> {
    let mut result = 0;
    for (x, xs) in grid.clone().iter().enumerate() {
        for (y, is_roll) in xs.iter().enumerate() {
            if *is_roll && forklift_accesible(x, y, &grid) {
                grid[x][y] = false;
                result += 1;
            }
        }
    }
    if result > 0 { Some(result) } else { None }
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<bool>> {
    input.iter().map(|s| parse_input_row(s)).collect()
}

fn parse_input_row(s: &str) -> Vec<bool> {
    s.chars()
        .map(|c| if c == '@' { true } else { false })
        .collect()
}

fn forklift_accesible(x: usize, y: usize, grid: &Vec<Vec<bool>>) -> bool {
    count_adjacent_rolls(x, y, grid) < 4
}

fn count_adjacent_rolls(x: usize, y: usize, grid: &Vec<Vec<bool>>) -> u8 {
    let x_len = grid.len();
    let y_len = grid.get(0).unwrap().len();
    adjacent_positions(x, y, x_len, y_len)
        .iter()
        .fold(0, |acc, (x0, y0)| {
            if *grid.get(*x0).unwrap().get(*y0).unwrap() {
                acc + 1
            } else {
                acc
            }
        })
}
fn adjacent_positions(x: usize, y: usize, x_length: usize, y_length: usize) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in dirs {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 && (nx as usize) < x_length && (ny as usize) < y_length {
            positions.push((nx as usize, ny as usize));
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_adjacent_rolls_test() {
        let grid = vec![vec![true, false, true, false]];
        assert_eq!(count_adjacent_rolls(0, 0, &grid), 0);
        assert_eq!(count_adjacent_rolls(0, 1, &grid), 2);
        assert_eq!(count_adjacent_rolls(0, 2, &grid), 0);
        assert_eq!(count_adjacent_rolls(0, 3, &grid), 1);
    }
}
