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
    println!("{}", input);
}

fn compute_priority(available: Vec<char>) -> u32 {
    let mut priority = 0;
    for character in available {
        if character.is_uppercase() {
            priority += (character as u8 - 'A' as u8 + 27) as u32;
        } else {
            priority += (character as u8 - 'a' as u8 + 1) as u32;
        }
    }
    priority
}

fn sum_shared_priority(input: &str) -> u32 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_priority() {
        assert_eq!(1, compute_priority(vec!['a']));
        assert_eq!(3, compute_priority(vec!['a', 'b']));
        assert_eq!(27, compute_priority(vec!['A']));
        assert_eq!(55, compute_priority(vec!['A', 'B']));
    }
}
