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
    worry_level: u32,
    starting_items: Vec<u32>,
    operation: fn(u32) -> u32,
    test: fn(u32, usize, usize) -> (usize, u32),
}

impl Monkey {
    fn new(
        starting_items: Vec<u32>,
        operation: fn(u32) -> u32,
        test: fn(u32, usize, usize) -> (usize, u32),
    ) -> Monkey {
        Monkey {
            worry_level: 0,
            starting_items,
            operation,
            test,
        }
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
        let monkey = Monkey::new(vec![1, 2, 3], |x| x + 1, |x, y, z| (y, x));
        assert_eq!(0, monkey.worry_level);
        assert_eq!(vec![1, 2, 3], monkey.starting_items);
        assert_eq!(2, (monkey.operation)(1));
        assert_eq!((1, 2), (monkey.test)(2, 1, 3));
    }
}
