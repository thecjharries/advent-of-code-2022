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

// https://fasterthanli.me/series/advent-of-code-2022/part-13

use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(n), Self::Number(o)) => n.partial_cmp(o),
            (n, o) => Some(n.with_slice(|n| {
                o.with_slice(|o| {
                    n.iter()
                        .zip(o.iter())
                        .map(|(nn, oo)| nn.cmp(oo))
                        .find(|&ord| ord != std::cmp::Ordering::Equal)
                        .unwrap_or_else(|| n.len().cmp(&o.len()))
                })
            })),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", sum_correct_packet_indices(&input));
    println!("Part 2: {}", find_decoder_key(&input));
}

fn sum_correct_packet_indices(input: &str) -> usize {
    let mut sum = 0;
    for (index, group) in input.trim().split("\n\n").enumerate() {
        let index = index + 1;
        let mut nodes = group
            .lines()
            .map(|line| serde_json::from_str(line.trim()).unwrap());
        let first: Node = nodes.next().unwrap();
        let second: Node = nodes.next().unwrap();
        if first < second {
            sum += index;
        }
    }
    sum
}

fn find_decoder_key(input: &str) -> usize {
    let dividers = vec![
        Node::List(vec![Node::Number(2)]),
        Node::List(vec![Node::Number(6)]),
    ];
    let mut nodes = input
        .trim()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<Node>(line.trim()).unwrap())
        .chain(dividers.iter().cloned())
        .collect::<Vec<_>>();
    nodes.sort();
    dividers
        .iter()
        .map(|d| nodes.binary_search(d).unwrap() + 1)
        .product::<usize>()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_node() {
        let input = "[[4,4],4,4]";
        let expected_output = Node::List(vec![
            Node::List(vec![Node::Number(4), Node::Number(4)]),
            Node::Number(4),
            Node::Number(4),
        ]);
        assert_eq!(expected_output, serde_json::from_str(input).unwrap());
    }

    #[test]
    fn test_node_with_slice() {
        let input = Node::List(vec![
            Node::List(vec![Node::Number(4), Node::Number(4)]),
            Node::Number(4),
            Node::Number(4),
        ]);
        let expected_output = vec![
            Node::List(vec![Node::Number(4), Node::Number(4)]),
            Node::Number(4),
            Node::Number(4),
        ];
        assert_eq!(expected_output, input.with_slice(|n| n.to_vec()));
        let input = Node::Number(4);
        let expected_output = vec![Node::Number(4)];
        assert_eq!(expected_output, input.with_slice(|n| n.to_vec()));
    }

    #[test]
    fn test_node_partial_ord() {
        let first = Node::List(vec![
            Node::List(vec![Node::Number(4), Node::Number(4)]),
            Node::Number(4),
            Node::Number(4),
        ]);
        let second = Node::List(vec![
            Node::List(vec![Node::Number(4), Node::Number(4)]),
            Node::Number(4),
            Node::Number(4),
        ]);
        assert_eq!(Some(std::cmp::Ordering::Equal), first.partial_cmp(&second));
        let first = Node::List(vec![
            Node::List(vec![Node::Number(4), Node::Number(4)]),
            Node::Number(4),
            Node::Number(4),
        ]);
        let second = Node::Number(3);
        assert_eq!(
            Some(std::cmp::Ordering::Greater),
            first.partial_cmp(&second)
        );
    }

    #[test]
    fn test_sum_correct_packet_indices() {
        let input = "[1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]

        ";
        assert_eq!(13, sum_correct_packet_indices(input));
    }

    #[test]
    fn test_find_decoder_key() {
        let input = "[1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]

        ";
        assert_eq!(140, find_decoder_key(input));
    }
}
