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

use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

lazy_static! {
    static ref MOVE_PATTERN: Regex =
        Regex::new(r"move (?P<count>\d+) from (?P<start>\d+) to (?P<end>\d+)").unwrap();
}

// I don't feel like messing with the regex for parsing this
// Here's my input stacks
//                         [R] [J] [W]
//             [R] [N]     [T] [T] [C]
// [R]         [P] [G]     [J] [P] [T]
// [Q]     [C] [M] [V]     [F] [F] [H]
// [G] [P] [M] [S] [Z]     [Z] [C] [Q]
// [P] [C] [P] [Q] [J] [J] [P] [H] [Z]
// [C] [T] [H] [T] [H] [P] [G] [L] [V]
// [F] [W] [B] [L] [P] [D] [L] [N] [G]
//  1   2   3   4   5   6   7   8   9
#[cfg(not(tarpaulin_include))]
fn main() {
    let stacks = vec![
        vec!['F', 'C', 'P', 'G', 'Q', 'R'],
        vec!['W', 'T', 'C', 'P'],
        vec!['B', 'H', 'P', 'M', 'C'],
        vec!['L', 'T', 'Q', 'S', 'M', 'P', 'R'],
        vec!['P', 'H', 'J', 'Z', 'V', 'G', 'N'],
        vec!['D', 'P', 'J'],
        vec!['L', 'G', 'P', 'Z', 'F', 'J', 'T', 'R'],
        vec!['N', 'L', 'H', 'C', 'F', 'P', 'T', 'J'],
        vec!['G', 'V', 'Z', 'Q', 'H', 'T', 'C', 'W'],
    ];
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!(
        "Part 1: {}",
        determine_final_stack_tops(&input, stacks.clone())
    );
    // println!("Part 2: {}", input);
}

fn determine_final_stack_tops(input: &str, stacks: Vec<Vec<char>>) -> String {
    let mut stacks = stacks;
    for line in input.trim().lines() {
        if let Some(captures) = MOVE_PATTERN.captures(line) {
            let count = captures["count"].parse::<usize>().unwrap();
            let start = captures["start"].parse::<usize>().unwrap();
            let end = captures["end"].parse::<usize>().unwrap();
            let mut start_stack = stacks.get(start - 1).unwrap().clone();
            let mut end_stack = stacks.get(end - 1).unwrap().clone();
            for _ in 0..count {
                end_stack.push(start_stack.pop().unwrap());
            }
            stacks[start - 1] = start_stack;
            stacks[end - 1] = end_stack;
        }
    }
    let mut final_stack_tops = String::new();
    for mut stack in stacks {
        final_stack_tops.push(stack.pop().unwrap().clone());
    }
    final_stack_tops
}

fn determine_final_stack_tops_9001(input: &str, stacks: Vec<Vec<char>>) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_final_stack_top() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(
            "CMZ".to_string(),
            determine_final_stack_tops(
                "    [D]
                [N] [C]
                [Z] [M] [P]
                 1   2   3

                move 1 from 2 to 1
                move 3 from 1 to 3
                move 2 from 2 to 1
                move 1 from 1 to 2
                ",
                stacks
            )
        );
    }

    #[test]
    fn test_determine_final_stack_tops_9001() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(
            "MCD".to_string(),
            determine_final_stack_tops_9001(
                "    [D]
                [N] [C]
                [Z] [M] [P]
                 1   2   3

                move 1 from 2 to 1
                move 3 from 1 to 3
                move 2 from 2 to 1
                move 1 from 1 to 2
                ",
                stacks
            )
        );
    }
}
