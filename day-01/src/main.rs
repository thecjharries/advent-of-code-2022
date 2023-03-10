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
    let input = read_to_string("input-1.txt").expect("Unable to read input file");
    println!("Max single elf: {}", find_max_elf(input.as_str()));
    println!("Max three elves: {}", find_max_elves(input.as_str()));
}

fn find_max_elf(input: &str) -> u32 {
    let mut max_elf = 0;
    let mut current_elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            if current_elf > max_elf {
                max_elf = current_elf;
            }
            current_elf = 0;
        } else {
            let line_elf = line.trim().parse::<u32>().expect("Unable to parse line");
            current_elf += line_elf;
        }
    }
    if current_elf > max_elf {
        max_elf = current_elf;
    }
    max_elf
}

fn find_max_elves(input: &str) -> u32 {
    let mut current_elf = 0;
    let mut elves = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            let line_elf = line.trim().parse::<u32>().expect("Unable to parse line");
            current_elf += line_elf;
        }
    }
    if current_elf > 0 {
        elves.push(current_elf);
    }
    elves.sort();
    elves.reverse();
    let mut max_elf_sum: u32 = 0;
    for elf in &elves[0..3] {
        max_elf_sum += *elf;
    }
    max_elf_sum
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_elf() {
        assert_eq!(
            24000,
            find_max_elf(
                "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000"
            )
        );
        assert_eq!(
            24000,
            find_max_elf(
                "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000"
            )
        );
    }

    #[test]
    fn test_find_max_elves() {
        assert_eq!(
            45000,
            find_max_elves(
                "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000"
            )
        );
    }
}
