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
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn manhattan_distance(first: &Point, second: &Point) -> usize {
        (first.x as isize - second.x as isize).abs() as usize
            + (first.y as isize - second.y as isize).abs() as usize
    }
}

#[derive(Debug, PartialEq)]
struct HeightMap {
    map: Vec<Vec<char>>,
    start: Point,
    end: Point,
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);

        for line in input.trim().lines() {
            let line = line.trim();
            let mut row = Vec::new();
            for character in line.chars() {
                if 'S' == character {
                    start = Point::new(row.len(), map.len());
                } else if 'E' == character {
                    end = Point::new(row.len(), map.len());
                }
                row.push(character);
            }
            map.push(row);
        }
        Ok(HeightMap { map, start, end })
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
    fn test_point_new() {
        let point = Point::new(1, 2);
        assert_eq!(1, point.x);
        assert_eq!(2, point.y);
    }

    #[test]
    fn test_point_manhattan_distance() {
        let first = Point::new(1, 2);
        let second = Point::new(3, 4);
        assert_eq!(4, Point::manhattan_distance(&first, &second));
    }

    #[test]
    fn test_height_map_from_str() {
        let expected = HeightMap {
            map: vec![
                vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
                vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
                vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k'],
                vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
                vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i'],
            ],
            start: Point::new(0, 0),
            end: Point::new(5, 2),
        };
        assert_eq!(
            expected,
            HeightMap::from_str(
                "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi

            "
            )
            .unwrap()
        );
    }
}
