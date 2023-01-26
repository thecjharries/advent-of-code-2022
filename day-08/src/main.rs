// Copyright 2023 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", find_visible_trees(&input));
    // println!("Part 2: {}", input);
}

fn find_visible_trees(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().lines().map(|line| line.trim()).collect();
    let mut grid = vec![vec![0; lines[0].len()]; lines.len()];
    let mut trees = 2 * lines.len() - 2 + (2 * lines[0].len() - 2);
    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            grid[y][x] = character.to_digit(10).unwrap();
        }
    }
    for (y, line) in grid[1..grid.len() - 1].iter().enumerate() {
        for (x, tree) in line[1..line.len() - 1].iter().enumerate() {
            let mut is_visible = true;
            for index in 0..x + 1 {
                if grid[y + 1][index] >= *tree {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                trees += 1;
                continue;
            }
            is_visible = true;
            for index in x + 2..line.len() {
                if grid[y + 1][index] >= *tree {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                trees += 1;
                continue;
            }
            is_visible = true;
            for index in 0..y + 1 {
                if grid[index][x + 1] >= *tree {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                trees += 1;
                continue;
            }
            is_visible = true;
            for index in y + 2..grid.len() {
                if grid[index][x + 1] >= *tree {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                trees += 1;
                continue;
            }
        }
    }
    trees
}

fn find_best_scenic_score(input: &str) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_visible_trees() {
        let input = "30373
        25512
        65332
        33549
        35390

        ";
        assert_eq!(21, find_visible_trees(input));
    }
}
