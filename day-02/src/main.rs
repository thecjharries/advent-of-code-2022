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
use std::collections::HashMap;
use std::fs::read_to_string;

enum ChoiceScore {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum ResultScore {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

lazy_static! {
    // A,X = Rock
    // B,Y = Paper
    // C,Z = Scissors
    static ref SCORING_MAP: HashMap<char, HashMap<char, u32>> = {
        let mut map = HashMap::new();
        map.insert('A', {
            let mut inner_map = HashMap::new();
            inner_map.insert('X', ResultScore::Draw as u32 + ChoiceScore::Rock as u32);
            inner_map.insert('Y', ResultScore::Win as u32 + ChoiceScore::Paper as u32);
            inner_map.insert('Z', ResultScore::Loss as u32 + ChoiceScore::Scissors as u32);
            inner_map
        });
        map.insert('B', {
            let mut inner_map = HashMap::new();
            inner_map.insert('X', ResultScore::Loss as u32 + ChoiceScore::Rock as u32);
            inner_map.insert('Y', ResultScore::Draw as u32 + ChoiceScore::Paper as u32);
            inner_map.insert('Z', ResultScore::Win as u32 + ChoiceScore::Scissors as u32);
            inner_map
        });
        map.insert('C', {
            let mut inner_map = HashMap::new();
            inner_map.insert('X', ResultScore::Win as u32 + ChoiceScore::Rock as u32);
            inner_map.insert('Y', ResultScore::Loss as u32 + ChoiceScore::Paper as u32);
            inner_map.insert('Z', ResultScore::Draw as u32 + ChoiceScore::Scissors as u32);
            inner_map
        });
        map
    };
    // A = Rock
    // B = Paper
    // C = Scissors
    // X = Loss
    // Y = Draw
    // Z = Win
    static ref CHOICE_MAP: HashMap<char, HashMap<char, u32>> = {
        let mut map = HashMap::new();
        map.insert('A', {
            let mut inner_map = HashMap::new();
            inner_map.insert('X', ResultScore::Loss as u32 + ChoiceScore::Scissors as u32);
            inner_map.insert('Y', ResultScore::Draw as u32 + ChoiceScore::Rock as u32);
            inner_map.insert('Z', ResultScore::Win as u32 + ChoiceScore::Paper as u32);
            inner_map
        });
        map.insert('B', {
            let mut inner_map = HashMap::new();
            inner_map.insert('X', ResultScore::Loss as u32 + ChoiceScore::Rock as u32);
            inner_map.insert('Y', ResultScore::Draw as u32 + ChoiceScore::Paper as u32);
            inner_map.insert('Z', ResultScore::Win as u32 + ChoiceScore::Scissors as u32);
            inner_map
        });
        map.insert('C', {
            let mut inner_map = HashMap::new();
            inner_map.insert('X', ResultScore::Loss as u32 + ChoiceScore::Paper as u32);
            inner_map.insert('Y', ResultScore::Draw as u32 + ChoiceScore::Scissors as u32);
            inner_map.insert('Z', ResultScore::Win as u32 + ChoiceScore::Rock as u32);
            inner_map
        });
        map
    };
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", compute_score(&input));
}

fn compute_score(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        let mut choices = line.trim().split(" ");
        let player_one_choice = choices.next().unwrap().chars().next().unwrap();
        let player_two_choice = choices.next().unwrap().chars().next().unwrap();
        score += SCORING_MAP
            .get(&player_one_choice)
            .unwrap()
            .get(&player_two_choice)
            .unwrap();
    }
    score
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_score() {
        assert_eq!(
            15,
            compute_score(
                "A Y
                B X
                C Z"
            )
        );
    }

    #[test]
    fn test_compute_choosing_score() {
        assert_eq!(
            12,
            compute_choosing_score(
                "A Y
                B X
                C Z"
            )
        );
    }
}
