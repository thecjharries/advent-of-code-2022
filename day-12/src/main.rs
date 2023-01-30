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

use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

impl HeightMap {
    fn distance(first: char, second: char) -> Option<i16> {
        let mut first = first;
        let mut second = second;
        if 'S' == first {
            first = 'a';
        } else if 'E' == first {
            first = 'z';
        }
        if 'S' == second {
            second = 'a';
        } else if 'E' == second {
            second = 'z';
        }
        let result = (first as i16 - 'a' as i16 + 1) - (second as i16 - 'a' as i16 + 1);
        if result < -1 {
            return None;
        }
        Some(result)
    }

    fn a_star(&mut self) -> usize {
        let mut open_set: Vec<Point> = Vec::new();
        open_set.push(self.start);
        let mut came_from: HashMap<Point, Point> = HashMap::new();
        let mut g_score: HashMap<Point, i64> = HashMap::new();
        let mut f_score: HashMap<Point, i64> = HashMap::new();
        f_score.insert(self.start, i16::MAX as i64);
        while 0 < open_set.len() {
            open_set.sort_by(|first, second| {
                f_score
                    .get(first)
                    .unwrap()
                    .cmp(f_score.get(second).unwrap())
            });
            let current = open_set.remove(0);
            if 'E' == self.map[current.y][current.x] {
                let mut distance = 0;
                let mut current = current;
                while self.start != current {
                    distance += 1;
                    current = came_from.get(&current).unwrap().clone();
                }
                return distance;
            }
            let mut neighbors: Vec<Point> = Vec::new();
            if 0 < current.y {
                let neighbor = Point::new(current.x, current.y - 1);
                if HeightMap::distance(
                    self.map[current.y][current.x],
                    self.map[neighbor.y][neighbor.x],
                )
                .is_some()
                {
                    neighbors.push(neighbor);
                }
            }
            if current.y < self.map.len() - 1 {
                let neighbor = Point::new(current.x, current.y + 1);
                if HeightMap::distance(
                    self.map[current.y][current.x],
                    self.map[neighbor.y][neighbor.x],
                )
                .is_some()
                {
                    neighbors.push(neighbor);
                }
            }
            if 0 < current.x {
                let neighbor = Point::new(current.x - 1, current.y);
                if HeightMap::distance(
                    self.map[current.y][current.x],
                    self.map[neighbor.y][neighbor.x],
                )
                .is_some()
                {
                    neighbors.push(neighbor);
                }
            }
            if current.x < self.map[current.y].len() - 1 {
                let neighbor = Point::new(current.x + 1, current.y);
                if HeightMap::distance(
                    self.map[current.y][current.x],
                    self.map[neighbor.y][neighbor.x],
                )
                .is_some()
                {
                    neighbors.push(neighbor);
                }
            }
            for neighbor in neighbors {
                let tentative_g_score = g_score.get(&current).unwrap_or(&(i16::MAX as i64)) + 1;
                if !g_score.contains_key(&neighbor)
                    || tentative_g_score < *g_score.get(&neighbor).unwrap()
                {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(
                        neighbor,
                        tentative_g_score
                            + HeightMap::distance(
                                self.map[current.y][current.x],
                                self.map[neighbor.y][neighbor.x],
                            )
                            .unwrap() as i64,
                    );
                    if !open_set.contains(&neighbor) {
                        open_set.push(neighbor);
                    }
                }
            }
        }
        0
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    let mut height_map1 = HeightMap::from_str(&input).expect("Unable to parse input");
    println!("Part 1: {}", height_map1.a_star());
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

    #[test]
    fn test_height_map_distance() {
        assert_eq!(0, HeightMap::distance('a', 'S').unwrap());
        assert!(HeightMap::distance('a', 'E').is_none());
        assert_eq!(-1, HeightMap::distance('a', 'b').unwrap());
        assert_eq!(0, HeightMap::distance('a', 'a').unwrap());
        assert!(HeightMap::distance('a', 'c').is_none());
        assert_eq!(10, HeightMap::distance('k', 'a').unwrap());
    }

    #[test]
    fn test_height_map_a_star() {
        let mut height_map = HeightMap::from_str(
            "
    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi

        ",
        )
        .unwrap();
        assert_eq!(31, height_map.a_star());
    }
}
