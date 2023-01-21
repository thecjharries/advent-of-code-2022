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
    println!("Part 1: {}", sum_shared_priority(&input));
    println!("Part 2: {}", sum_grouped_priority(&input));
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
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut first: HashSet<char> = HashSet::new();
        let mut second: HashSet<char> = HashSet::new();
        let line = line.trim();
        for index in 0..line.len() / 2 {
            first.insert(line.chars().nth(index).unwrap());
            second.insert(line.chars().nth(line.len() - index - 1).unwrap());
        }
        let mut raw_intersection: Vec<&char> = first.intersection(&second).collect();
        let intersection: Vec<char> = raw_intersection.drain(..).map(|&x| x).collect();
        sum += compute_priority(intersection);
    }
    sum
}

fn sum_grouped_priority(input: &str) -> u32 {
    let mut sum = 0;
    let lines = input.trim().lines().collect::<Vec<&str>>();
    for index in (0..lines.len()).step_by(3) {
        let mut first: HashSet<char> = HashSet::new();
        let mut second: HashSet<char> = HashSet::new();
        let mut third: HashSet<char> = HashSet::new();
        for character in lines[index].trim().chars() {
            first.insert(character);
        }
        for character in lines[index + 1].trim().chars() {
            second.insert(character);
        }
        for character in lines[index + 2].trim().chars() {
            third.insert(character);
        }
        let first_intersection: HashSet<char> = first.intersection(&second).map(|&x| x).collect();
        let intersection: Vec<char> = first_intersection
            .intersection(&third)
            .map(|&x| x)
            .collect();
        sum += compute_priority(intersection);
    }
    sum
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

    #[test]
    fn test_sum_shared_priority() {
        assert_eq!(
            157,
            sum_shared_priority(
                "vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw

                "
            )
        );
    }

    #[test]
    fn test_sum_grouped_priority() {
        assert_eq!(
            70,
            sum_grouped_priority(
                "vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw

                "
            )
        );
    }
}
