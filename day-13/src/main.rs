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
}
