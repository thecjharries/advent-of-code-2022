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
    println!("{}", find_max_elf(input.as_str()));
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
    max_elf
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
    }
}
