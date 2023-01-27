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
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
struct Movement {
    x: i32,
    y: i32,
}

impl FromStr for Movement {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let items = input.trim().split_whitespace().collect::<Vec<&str>>();
        let direction = items[0].chars().next().unwrap();
        let magnitude = items[1].parse::<i32>().unwrap();
        let (x, y) = match direction {
            'U' => (0, magnitude),
            'D' => (0, -magnitude),
            'L' => (-magnitude, 0),
            'R' => (magnitude, 0),
            _ => (0, 0),
        };
        Ok(Movement { x, y })
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

impl Rope {
    fn move_rope(&mut self, movement: Movement) {
        let new_head = Point::new(self.head.x + movement.x, self.head.y + movement.y);
        while self.head != new_head {
            self.head = Point::new(
                self.head.x + movement.x.signum(),
                self.head.y + movement.y.signum(),
            );
            let mut head_in_range = false;
            for y in vec![-1, 0, 1] {
                for x in vec![-1, 0, 1] {
                    let point = Point::new(self.tail.x + x, self.tail.y + y);
                    if point == self.head {
                        head_in_range = true;
                    }
                }
                if head_in_range {
                    break;
                }
            }
            if !head_in_range {
                if self.head.x == self.tail.x {
                    if self.head.y > self.tail.y {
                        self.tail.y += 1;
                    } else {
                        self.tail.y -= 1;
                    }
                } else if self.head.y == self.tail.y {
                    if self.head.x > self.tail.x {
                        self.tail.x += 1;
                    } else {
                        self.tail.x -= 1;
                    }
                } else {
                    if self.head.x > self.tail.x {
                        self.tail.x += 1;
                    } else {
                        self.tail.x -= 1;
                    }
                    if self.head.y > self.tail.y {
                        self.tail.y += 1;
                    } else {
                        self.tail.y -= 1;
                    }
                }
                self.tail_visited.insert(self.tail);
            }
        }
    }

    fn parse_movements(&mut self, input: &str) {
        let lines = input.trim().lines().collect::<Vec<&str>>();
        for line in lines.iter() {
            let movement = line.parse::<Movement>().unwrap();
            self.move_rope(movement);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct KnottedRope {
    knots: Vec<Point>,
    tail_visited: BTreeSet<Point>,
}

impl Default for KnottedRope {
    fn default() -> Self {
        let mut tail_visited = BTreeSet::new();
        tail_visited.insert(Point::default());
        let mut knots = Vec::new();
        for _ in 0..10 {
            knots.push(Point::default());
        }
        KnottedRope {
            knots,
            tail_visited,
        }
    }
}

impl KnottedRope {
    fn move_rope(&mut self, movement: Movement) {
        let new_head = Point::new(self.knots[0].x + movement.x, self.knots[0].y + movement.y);
        while self.knots[0] != new_head {
            self.knots[0] = Point::new(
                self.knots[0].x + movement.x.signum(),
                self.knots[0].y + movement.y.signum(),
            );
            for index in 1..self.knots.len() {
                let mut in_range = false;
                for y in vec![-1, 0, 1] {
                    for x in vec![-1, 0, 1] {
                        let point = Point::new(self.knots[index].x + x, self.knots[index].y + y);
                        if point == self.knots[index - 1] {
                            in_range = true;
                            break;
                        }
                    }
                    if in_range {
                        break;
                    }
                }
                if !in_range {
                    if self.knots[index - 1].x == self.knots[index].x {
                        if self.knots[index - 1].y > self.knots[index].y {
                            self.knots[index].y += 1;
                        } else {
                            self.knots[index].y -= 1;
                        }
                    } else if self.knots[index - 1].y == self.knots[index].y {
                        if self.knots[index - 1].x > self.knots[index].x {
                            self.knots[index].x += 1;
                        } else {
                            self.knots[index].x -= 1;
                        }
                    } else {
                        if self.knots[index - 1].x > self.knots[index].x {
                            self.knots[index].x += 1;
                        } else {
                            self.knots[index].x -= 1;
                        }
                        if self.knots[index - 1].y > self.knots[index].y {
                            self.knots[index].y += 1;
                        } else {
                            self.knots[index].y -= 1;
                        }
                    }
                }
            }
            self.tail_visited.insert(self.knots[9]);
        }
    }

    fn parse_movements(&mut self, input: &str) {
        let lines = input.trim().lines().collect::<Vec<&str>>();
        for line in lines.iter() {
            let movement = line.parse::<Movement>().unwrap();
            self.move_rope(movement);
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    let mut rope = Rope::default();
    rope.parse_movements(&input);
    println!("Part 1: {}", rope.tail_visited.len());
    let mut knotted_rope = KnottedRope::default();
    knotted_rope.parse_movements(&input);
    println!("Part 2: {}", knotted_rope.tail_visited.len());
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
    fn test_movement_from_str() {
        let mut expected = Movement { x: 0, y: 1 };
        assert_eq!(expected, "U 1".parse::<Movement>().unwrap());
        assert_eq!(
            expected,
            Movement::from_str("U 1").expect("Unable to parse movement")
        );
        expected = Movement { x: 0, y: -1 };
        assert_eq!(expected, "D 1".parse::<Movement>().unwrap());
        expected = Movement { x: -1, y: 0 };
        assert_eq!(expected, "L 1".parse::<Movement>().unwrap());
        expected = Movement { x: 1, y: 0 };
        assert_eq!(expected, "R 1".parse::<Movement>().unwrap());
        expected = Movement { x: 0, y: 0 };
        assert_eq!(expected, "X 1".parse::<Movement>().unwrap());
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

    #[test]
    fn test_rope_move() {
        let mut rope = Rope::default();
        rope.move_rope(Movement::from_str("R 4").expect("Unable to parse movement"));
        assert_eq!(4, rope.tail_visited.len());
        rope.move_rope(Movement::from_str("U 4").expect("Unable to parse movement"));
        assert_eq!(7, rope.tail_visited.len());
        rope.move_rope(Movement::from_str("L 3").expect("Unable to parse movement"));
        assert_eq!(9, rope.tail_visited.len());
        rope.move_rope(Movement::from_str("D 1").expect("Unable to parse movement"));
        rope.move_rope(Movement::from_str("R 4").expect("Unable to parse movement"));
        rope.move_rope(Movement::from_str("D 1").expect("Unable to parse movement"));
        rope.move_rope(Movement::from_str("L 5").expect("Unable to parse movement"));
        rope.move_rope(Movement::from_str("R 2").expect("Unable to parse movement"));
        assert_eq!(13, rope.tail_visited.len());
    }

    #[test]
    fn test_rope_parse_movements() {
        let mut rope = Rope::default();
        rope.parse_movements(
            "R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2

            ",
        );
        assert_eq!(13, rope.tail_visited.len());
    }

    #[test]
    fn test_default_knotted_rope() {
        let knotted_rope = KnottedRope::default();
        assert_eq!(10, knotted_rope.knots.len());
        assert_eq!(1, knotted_rope.tail_visited.len());
    }

    #[test]
    fn test_knotted_rope_move_rope() {
        let mut knotted_rope = KnottedRope::default();
        knotted_rope.move_rope(Movement::from_str("R 4").expect("Unable to parse movement"));
        assert_eq!(1, knotted_rope.tail_visited.len());
        knotted_rope.move_rope(Movement::from_str("U 4").expect("Unable to parse movement"));
        knotted_rope.move_rope(Movement::from_str("L 3").expect("Unable to parse movement"));
        knotted_rope.move_rope(Movement::from_str("D 1").expect("Unable to parse movement"));
        knotted_rope.move_rope(Movement::from_str("R 4").expect("Unable to parse movement"));
        knotted_rope.move_rope(Movement::from_str("D 1").expect("Unable to parse movement"));
        knotted_rope.move_rope(Movement::from_str("L 5").expect("Unable to parse movement"));
        knotted_rope.move_rope(Movement::from_str("R 2").expect("Unable to parse movement"));
        assert_eq!(1, knotted_rope.tail_visited.len());
    }

    #[test]
    fn test_knotted_rope_parse_movements() {
        let mut knotted_rope = KnottedRope::default();
        knotted_rope.parse_movements(
            "R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2

            ",
        );
        assert_eq!(1, knotted_rope.tail_visited.len());
        knotted_rope = KnottedRope::default();
        knotted_rope.parse_movements(
            "

        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
        ",
        );
        assert_eq!(36, knotted_rope.tail_visited.len());
    }
}
