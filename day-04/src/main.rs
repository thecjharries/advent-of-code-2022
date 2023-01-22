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
    println!("Part 1: {}", count_overlaps(&input));
    // println!("Part 2: {}", input);
}

fn count_overlaps(input: &str) -> u32 {
    let mut count: u32 = 0;
    for line in input.trim().lines() {
        let line = line.trim();
        let groups: Vec<&str> = line.split(",").collect();
        let first_group = groups[0].split("-").collect::<Vec<&str>>();
        let first: HashSet<u32> =
            (first_group[0].parse().unwrap()..=first_group[1].parse().unwrap()).collect();
        let second_group = groups[1].split("-").collect::<Vec<&str>>();
        let second: HashSet<u32> =
            (second_group[0].parse().unwrap()..=second_group[1].parse().unwrap()).collect();
        if first.is_subset(&second) || second.is_subset(&first) {
            count += 1;
        }
    }
    count
}

fn count_intersection(input: &str) -> u32 {
    let mut count: u32 = 0;
    for line in input.trim().lines() {
        let line = line.trim();
        let groups: Vec<&str> = line.split(",").collect();
        let first_group = groups[0].split("-").collect::<Vec<&str>>();
        let first: HashSet<u32> =
            (first_group[0].parse().unwrap()..=first_group[1].parse().unwrap()).collect();
        let second_group = groups[1].split("-").collect::<Vec<&str>>();
        let second: HashSet<u32> =
            (second_group[0].parse().unwrap()..=second_group[1].parse().unwrap()).collect();
        if 0 < first.intersection(&second).count() {
            count += 1;
        }
    }
    count
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_overlaps() {
        assert_eq!(
            2,
            count_overlaps(
                "2-4,6-8
                2-3,4-5
                5-7,7-9
                2-8,3-7
                6-6,4-6
                2-6,4-8
                "
            )
        );
    }

    #[test]
    fn test_count_intersections() {
        assert_eq!(
            4,
            count_intersection(
                "2-4,6-8
                2-3,4-5
                5-7,7-9
                2-8,3-7
                6-6,4-6
                2-6,4-8
                "
            )
        );
    }
}
