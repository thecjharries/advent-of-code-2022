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
enum CycleTime {
    Noop = 1,
    Addx = 2,
}

#[derive(Debug, PartialEq)]
enum Action {
    Noop(i32),
    Addx(i32),
}

impl FromStr for Action {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split_input = input.trim().split_whitespace();
        let action = split_input.next().unwrap();
        match action {
            "noop" => Ok(Action::Noop(0)),
            "addx" => {
                let value = split_input.next().unwrap().parse::<i32>().unwrap();
                Ok(Action::Addx(value))
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Program {
    actions: Vec<Action>,
    cycles: u32,
    x: i32,
    previous_action_cycle: u32,
    signal_strength: i32,
    signal_checks: Vec<usize>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            actions: vec![],
            cycles: 0,
            x: 0,
            previous_action_cycle: 0,
            signal_strength: 0,
            signal_checks: vec![20, 60, 100, 140, 180, 220],
        }
    }
}

impl Program {
    fn parse_actions(&mut self, input: &str) {
        self.actions = input
            .trim()
            .lines()
            .map(|action| action.parse::<Action>().unwrap())
            .collect();
        self.actions.reverse();
    }

    fn run_cycle(&mut self) {
        let action = match self.actions.pop() {
            Some(action) => action,
            None => return,
        };
        let cycle_time = match action {
            Action::Noop(_) => CycleTime::Noop,
            Action::Addx(_) => CycleTime::Addx,
        };
        self.cycles += 1;
        if self.cycles == self.previous_action_cycle + cycle_time as u32 {
            self.previous_action_cycle = self.cycles;
            match action {
                Action::Noop(_) => {}
                Action::Addx(value) => self.x += value,
            }
        } else {
            self.actions.push(action);
        }
        if self.signal_checks.contains(&(self.cycles as usize)) {
            self.signal_strength += self.x * self.cycles as i32;
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
    fn test_noop_action_from_str() {
        assert_eq!(Action::Noop(0), Action::from_str("noop").unwrap());
    }

    #[test]
    fn test_addx_action_from_str() {
        assert_eq!(Action::Addx(1), Action::from_str("addx 1").unwrap());
        assert_eq!(Action::Addx(-10), Action::from_str("addx -10").unwrap());
    }

    #[test]
    fn test_default_program() {
        let expected_program = Program {
            actions: vec![],
            cycles: 0,
            x: 0,
            previous_action_cycle: 0,
            signal_strength: 0,
            signal_checks: vec![20, 60, 100, 140, 180, 220],
        };
        assert_eq!(expected_program, Program::default());
    }

    #[test]
    fn test_parse_actions() {
        let mut program = Program::default();
        program.parse_actions(
            "noop
            addx 3
            addx -5

            ",
        );
        let expected_program = Program {
            actions: vec![Action::Addx(-5), Action::Addx(3), Action::Noop(0)],
            cycles: 0,
            x: 0,
            previous_action_cycle: 0,
            signal_strength: 0,
            signal_checks: vec![20, 60, 100, 140, 180, 220],
        };
        assert_eq!(expected_program, program);
    }

    #[test]
    fn test_program_run_cycle() {
        let mut program = Program::default();
        program.parse_actions(
            "noop
            addx 3
            addx -5

            ",
        );
        assert_eq!(0, program.cycles);
        assert_eq!(0, program.x);
        assert_eq!(3, program.actions.len());
        program.run_cycle();
        assert_eq!(1, program.cycles);
        assert_eq!(0, program.x);
        assert_eq!(2, program.actions.len());
        program.run_cycle();
        assert_eq!(2, program.cycles);
        assert_eq!(0, program.x);
        assert_eq!(2, program.actions.len());
        program.run_cycle();
        assert_eq!(3, program.cycles);
        assert_eq!(3, program.x);
        assert_eq!(1, program.actions.len());
        program.run_cycle();
        assert_eq!(4, program.cycles);
        assert_eq!(3, program.x);
        assert_eq!(1, program.actions.len());
        program.run_cycle();
        assert_eq!(5, program.cycles);
        assert_eq!(-2, program.x);
        assert_eq!(0, program.actions.len());
        program.run_cycle();
        assert_eq!(5, program.cycles);
        assert_eq!(-2, program.x);
        assert_eq!(0, program.actions.len());
    }
}
