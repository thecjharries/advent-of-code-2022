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

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<u32>,
    operation: fn(u32) -> u32,
    test: fn(u32, usize, usize) -> (usize, u32),
    true_index: usize,
    false_index: usize,
    inspection_count: usize,
}

impl Monkey {
    fn new(
        starting_items: Vec<u32>,
        operation: fn(u32) -> u32,
        test: fn(u32, usize, usize) -> (usize, u32),
        true_index: usize,
        false_index: usize,
    ) -> Monkey {
        Monkey {
            starting_items,
            operation,
            test,
            true_index,
            false_index,
            inspection_count: 0,
        }
    }

    fn compute_round(&mut self) -> Vec<(usize, u32)> {
        let mut results = Vec::new();
        self.inspection_count += self.starting_items.len();
        for item in self.starting_items.iter() {
            let (new_index, new_item) =
                (self.test)((self.operation)(*item), self.true_index, self.false_index);
            results.push((new_index, new_item / 3));
        }
        results
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", input);
    // println!("Part 2: {}", input);
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_new() {
        let monkey = Monkey::new(vec![1, 2, 3], |x| x + 1, |x, y, z| (y, x), 0, 1);
        assert_eq!(vec![1, 2, 3], monkey.starting_items);
        assert_eq!(2, (monkey.operation)(1));
        assert_eq!((0, 2), (monkey.test)(2, 0, 1));
    }

    #[test]
    fn test_monkey_compute_round() {
        let mut monkey = Monkey::new(
            vec![79, 98],
            |old| old * 19,
            |item, true_index, false_index| {
                if 0 == item % 23 {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            2,
            3,
        );
        assert_eq!(vec![(3, 500), (3, 620),], monkey.compute_round());
        assert_eq!(2, monkey.inspection_count);
    }
}
