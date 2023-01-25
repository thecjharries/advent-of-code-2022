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

#[derive(Debug, PartialEq)]
enum ItemType {
    File(u32),
    Directory,
}

#[derive(Debug, PartialEq)]
struct NodeId {
    index: usize,
}

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    item_type: ItemType,
    children: Vec<Node>,
    parent: Option<NodeId>,
}

#[derive(Debug, PartialEq)]
struct FileSystem {
    nodes: Vec<Node>,
    current_node: NodeId,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            nodes: vec![Node {
                name: String::from("/"),
                item_type: ItemType::Directory,
                children: vec![],
                parent: None,
            }],
            current_node: NodeId { index: 0 },
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", input);
    // println!("Part 2: {}", input);
}

fn build_file_system(input: &str) -> FileSystem {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_file_system() {
        let expected = FileSystem {
            nodes: vec![Node {
                name: String::from("/"),
                item_type: ItemType::Directory,
                children: vec![],
                parent: None,
            }],
            current_node: NodeId { index: 0 },
        };
        assert_eq!(expected, FileSystem::new());
    }
}
