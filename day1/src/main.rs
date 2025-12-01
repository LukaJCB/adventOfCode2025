use std::fs;

fn main() {
    let instructions: Vec<Rotation> = fs::read_to_string("./input.txt")
        .unwrap()
        .split("\n")
        .map(|s| parse_rotation(s).unwrap())
        .collect();

    let mut state = 50;
    let mut answer_1 = 0;
    let mut answer_2 = 0;

    for rot in instructions {
        let res = apply_rotation(state, &rot);
        state = res.state;
        answer_2 += res.zero_clicks;

        if state == 0 {
            answer_1 += 1;
        }
    }

    println!("{answer_1}");
    println!("{answer_2}");
}

fn apply_rotation(state: i32, rot: &Rotation) -> RotationResult {
    if let Direction::Left = rot.direction {
        handle_left_rotation(state, rot.amount)
    } else {
        handle_right_rotation(state, rot.amount)
    }
}

fn handle_left_rotation(state: i32, amount: i32) -> RotationResult {
    let subtracted = state - amount;
    if subtracted < 0 {
        RotationResult {
            state: (100 + subtracted % 100) % 100,
            zero_clicks: if state == 0 {
                amount / 100
            } else {
                1 + subtracted / -100
            },
        }
    } else {
        RotationResult {
            state: subtracted,
            zero_clicks: if subtracted == 0 { 1 } else { 0 },
        }
    }
}

fn handle_right_rotation(state: i32, amount: i32) -> RotationResult {
    let added = state + amount;
    RotationResult {
        state: added % 100,
        zero_clicks: added / 100,
    }
}

fn parse_rotation(s: &str) -> Result<Rotation, &str> {
    let (dir, n) = s.split_at(1);
    let amount: i32 = n.parse().unwrap();
    if dir == "R" {
        Ok(Rotation {
            direction: Direction::Right,
            amount: amount,
        })
    } else if dir == "L" {
        Ok(Rotation {
            direction: Direction::Left,
            amount: amount,
        })
    } else {
        Err("Couldn't parse rotation")
    }
}

enum Direction {
    Left,
    Right,
}
struct Rotation {
    direction: Direction,
    amount: i32,
}

struct RotationResult {
    state: i32,
    zero_clicks: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn under_rotation() {
        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Left,
                    amount: 2
                }
            )
            .state,
            98
        );
        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Left,
                    amount: 102
                }
            )
            .state,
            98
        );
        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Left,
                    amount: 402
                }
            )
            .state,
            98
        );
        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Left,
                    amount: 300
                }
            )
            .state,
            0
        );

        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Left,
                    amount: 2
                }
            )
            .zero_clicks,
            0
        );
        assert_eq!(
            apply_rotation(
                2,
                &Rotation {
                    direction: Direction::Left,
                    amount: 2
                }
            )
            .zero_clicks,
            1
        );
        assert_eq!(
            apply_rotation(
                2,
                &Rotation {
                    direction: Direction::Left,
                    amount: 3
                }
            )
            .zero_clicks,
            1
        );
        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Left,
                    amount: 402
                }
            )
            .zero_clicks,
            4
        );
        assert_eq!(
            apply_rotation(
                40,
                &Rotation {
                    direction: Direction::Left,
                    amount: 102
                }
            )
            .zero_clicks,
            1
        );
        assert_eq!(
            apply_rotation(
                40,
                &Rotation {
                    direction: Direction::Left,
                    amount: 302
                }
            )
            .zero_clicks,
            3
        );
        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Left,
                    amount: 300
                }
            )
            .zero_clicks,
            3
        );
    }

    #[test]
    fn over_rotation() {
        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Right,
                    amount: 2
                }
            )
            .state,
            2
        );
        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Right,
                    amount: 102
                }
            )
            .state,
            2
        );
        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Right,
                    amount: 402
                }
            )
            .state,
            2
        );
        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Right,
                    amount: 300
                }
            )
            .state,
            0
        );

        assert_eq!(
            apply_rotation(
                0,
                &Rotation {
                    direction: Direction::Right,
                    amount: 2
                }
            )
            .zero_clicks,
            0
        );
        assert_eq!(
            apply_rotation(
                50,
                &Rotation {
                    direction: Direction::Right,
                    amount: 1000
                }
            )
            .zero_clicks,
            10
        );
    }
}
