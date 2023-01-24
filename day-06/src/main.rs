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

use std::collections::HashSet;
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    // println!("Part 1: {}", find_sop_marker(&input));
    // println!("Part 2: {}", input);
}

fn find_marker(input: &str, marker_length: usize) -> u32 {
    if marker_length > input.len() {
        return 0;
    }
    for index in (marker_length - 1)..input.len() {
        let set = input
            .chars()
            .skip((index + 1) - marker_length)
            .take(marker_length)
            .collect::<HashSet<char>>();
        if marker_length == set.len() {
            return index as u32 + 1;
        }
    }
    0
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_marker() {
        assert_eq!(0, find_marker("abc", 4));
        assert_eq!(
            0,
            find_marker(
                "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc",
                4
            )
        );
        assert_eq!(7, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }
}
