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
struct SystemItem {
    name: String,
    parent: Option<Box<SystemItem>>,
    item_type: ItemType,
    children: Vec<SystemItem>,
}

impl SystemItem {
    fn new(name: String, parent: Option<Box<SystemItem>>, item_type: ItemType) -> SystemItem {
        SystemItem {
            name,
            parent,
            item_type,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: SystemItem) {
        self.children.push(child);
    }

    fn get_size(&self) -> u32 {
        match self.item_type {
            ItemType::File(size) => size,
            ItemType::Directory => self.children.iter().map(|child| child.get_size()).sum(),
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", input);
    // println!("Part 2: {}", input);
}

fn build_file_system(input: &str) -> SystemItem {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_item_new_directory() {
        let name = String::from("test");
        let parent = None;
        let item_type = ItemType::Directory;
        let system_item = SystemItem::new(name, parent, item_type);
        assert_eq!("test", system_item.name);
        assert_eq!(None, system_item.parent);
        assert_eq!(ItemType::Directory, system_item.item_type);
        assert_eq!(0, system_item.children.len());
    }

    #[test]
    fn test_system_item_new_file() {
        let name = String::from("test");
        let parent = None;
        let item_type = ItemType::File(1);
        let system_item = SystemItem::new(name, parent, item_type);
        assert_eq!("test", system_item.name);
        assert_eq!(None, system_item.parent);
        assert_eq!(ItemType::File(1), system_item.item_type);
        assert_eq!(0, system_item.children.len());
    }

    #[test]
    fn test_system_item_add_child() {
        let mut parent = SystemItem::new(String::from("parent"), None, ItemType::Directory);
        let child = SystemItem::new(String::from("child"), None, ItemType::File(1));
        parent.add_child(child);
        assert_eq!(1, parent.children.len());
    }

    #[test]
    fn test_system_item_get_size_directory() {
        let mut parent = SystemItem::new(String::from("parent"), None, ItemType::Directory);
        let child = SystemItem::new(String::from("child"), None, ItemType::File(1));
        parent.add_child(child);
        assert_eq!(1, parent.get_size());
    }

    #[test]
    fn test_system_item_get_size_file() {
        let parent = SystemItem::new(String::from("parent"), None, ItemType::File(1));
        assert_eq!(1, parent.get_size());
    }
}
