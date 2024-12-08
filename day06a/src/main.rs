use core::str;
use std::{fmt::Display, fs};

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct GridState {
    grid: Vec<Vec<char>>,
    x_pos: usize,
    y_pos: usize,
    current_direction: Direction,
}

impl GridState {
    fn is_valid_position(&self, y: usize, x: usize) -> bool {
        y < self.grid.len() && x < self.grid[0].len()
    }

    fn can_move_up(&self) -> bool {
        self.y_pos > 0 && self.is_valid_position(self.y_pos - 1, self.x_pos)
    }

    fn can_move_right(&self) -> bool {
        self.is_valid_position(self.y_pos, self.x_pos + 1)
    }

    fn can_move_down(&self) -> bool {
        self.is_valid_position(self.y_pos + 1, self.x_pos)
    }

    fn can_move_left(&self) -> bool {
        self.x_pos > 0 && self.is_valid_position(self.y_pos, self.x_pos - 1)
    }

    fn update_cursor_value(&mut self) {
        self.grid[self.y_pos][self.x_pos] = 'X';
    }

    fn get_distinct_grid_positions(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|val| **val == 'X')
            .count()
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            writeln!(f, "{}", row.iter().collect::<String>());
        }
        Ok(())
    }
}

fn distinct_positions(input: &str) -> usize {
    let start_grid = convert_input_to_grid(input);

    let current_position =
        get_starting_position(&start_grid).expect("No starting position in the input grid");

    let mut active_grid = GridState {
        grid: start_grid,
        y_pos: current_position.0,
        x_pos: current_position.1,
        current_direction: Direction::UP,
    };

    while active_grid.is_valid_position(active_grid.y_pos, active_grid.x_pos) {
        active_grid.update_cursor_value();

        match active_grid.current_direction {
            Direction::UP => {
                if !active_grid.can_move_up() {
                    break;
                } else if active_grid.grid[active_grid.y_pos - 1][active_grid.x_pos] != '#' {
                    active_grid.y_pos -= 1;
                } else {
                    active_grid.current_direction = Direction::RIGHT;
                }
            }
            Direction::RIGHT => {
                if !active_grid.can_move_right() {
                    break;
                } else if active_grid.grid[active_grid.y_pos][active_grid.x_pos + 1] != '#' {
                    active_grid.x_pos += 1;
                } else {
                    active_grid.current_direction = Direction::DOWN;
                }
            }
            Direction::DOWN => {
                if !active_grid.can_move_down() {
                    break;
                } else if active_grid.grid[active_grid.y_pos + 1][active_grid.x_pos] != '#' {
                    active_grid.y_pos += 1;
                } else {
                    active_grid.current_direction = Direction::LEFT;
                }
            }
            Direction::LEFT => {
                if !active_grid.can_move_left() {
                    break;
                } else if active_grid.grid[active_grid.y_pos][active_grid.x_pos - 1] != '#' {
                    active_grid.x_pos -= 1;
                } else {
                    active_grid.current_direction = Direction::UP;
                }
            }
        }
    }

    println!(" === FINAL GRID STATE ===");
    print!("{active_grid}");

    active_grid.get_distinct_grid_positions()
}

fn convert_input_to_grid(input: &str) -> Vec<Vec<char>> {
    let output = input.lines().map(|line| line.chars().collect()).collect();

    output
}

fn get_starting_position(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (y_pos, x_vec) in grid.iter().enumerate() {
        if let Some(x_pos) = x_vec.iter().position(|x_pos| *x_pos == '^') {
            return Some((y_pos, x_pos));
        }
    }
    None
}

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();

    println!("Final Value: {}", distinct_positions(&f));
}

#[cfg(test)]
#[test]
fn test_case() {
    let payload = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    assert_eq!(distinct_positions(payload), 41);
}
