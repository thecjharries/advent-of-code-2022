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

use std::collections::BTreeSet;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rope {
    head: Point,
    tail: Point,
    tail_visited: BTreeSet<Point>,
}

impl Default for Rope {
    fn default() -> Self {
        let mut tail_visited = BTreeSet::new();
        tail_visited.insert(Point::default());
        Rope {
            head: Point::default(),
            tail: Point::default(),
            tail_visited,
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
    fn test_default_point() {
        let expected = Point { x: 0, y: 0 };
        assert_eq!(expected, Point::default());
    }

    #[test]
    fn test_new_point() {
        let expected = Point { x: 1, y: 2 };
        assert_eq!(expected, Point::new(1, 2));
    }

    #[test]
    fn test_default_rope() {
        let mut tail_visited = BTreeSet::new();
        tail_visited.insert(Point::default());
        let expected = Rope {
            head: Point::default(),
            tail: Point::default(),
            tail_visited,
        };
        assert_eq!(expected, Rope::default());
    }
}
