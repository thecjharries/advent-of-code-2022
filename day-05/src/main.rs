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

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", input);
    println!("Part 2: {}", input);
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
}
