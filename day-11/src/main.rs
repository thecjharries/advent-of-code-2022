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

#![allow(unused_imports)]
use num_bigint::BigUint;
use std::ops::{Add, Div, Mul, Rem};

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<BigUint>,
    operation: fn(BigUint) -> BigUint,
    test: fn(BigUint, usize, usize) -> (usize, BigUint),
    true_index: usize,
    false_index: usize,
    inspection_count: usize,
}

impl Monkey {
    fn new(
        starting_items: Vec<BigUint>,
        operation: fn(BigUint) -> BigUint,
        test: fn(BigUint, usize, usize) -> (usize, BigUint),
        true_index: usize,
        false_index: usize,
    ) -> Monkey {
        Monkey {
            starting_items,
            operation,
            test,
            true_index,
            false_index,
            inspection_count: 0,
        }
    }

    fn compute_round(
        &mut self,
        factor: BigUint,
        reduction_factor: BigUint,
    ) -> Vec<(usize, BigUint)> {
        let mut results = Vec::new();
        self.inspection_count += self.starting_items.len();
        self.starting_items.reverse();
        while let Some(item) = self.starting_items.pop() {
            let worry_level = (self.operation)(item % reduction_factor.clone()).div(factor.clone());
            let (new_index, new_item) = (self.test)(worry_level, self.true_index, self.false_index);
            results.push((new_index, new_item));
        }
        results
    }
}

struct Monkeys(Vec<Monkey>);

impl Monkeys {
    fn round(&mut self, factor: BigUint, reduction_factor: BigUint) {
        let mut results: Vec<(usize, BigUint)> = Vec::new();
        for (index, monkey) in self.0.iter_mut().enumerate() {
            for item in results.iter() {
                if index == item.0 {
                    monkey.starting_items.push(item.1.clone());
                }
            }
            for item in monkey.compute_round(factor.clone(), reduction_factor.clone()) {
                results.push(item);
            }
            results.retain(|item| index != item.0);
        }
        for item in results {
            self.0[item.0].starting_items.push(item.1.clone());
        }
    }

    fn monkey_business(
        &mut self,
        rounds: u32,
        factor: BigUint,
        reduction_factor: BigUint,
    ) -> usize {
        for _ in 0..rounds {
            self.round(factor.clone(), reduction_factor.clone());
        }
        let mut inspection_counts = Vec::new();
        for monkey in self.0.iter() {
            inspection_counts.push(monkey.inspection_count);
        }
        inspection_counts.sort();
        inspection_counts.reverse();
        inspection_counts[0] * inspection_counts[1]
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let mut monkeys1: Monkeys = Monkeys(vec![
        //         Monkey 0:
        //   Starting items: 66, 71, 94
        //   Operation: new = old * 5
        //   Test: divisible by 3
        //     If true: throw to monkey 7
        //     If false: throw to monkey 4
        Monkey::new(
            vec![
                BigUint::from(66u32),
                BigUint::from(71u32),
                BigUint::from(94u32),
            ],
            |old| old * BigUint::from(5u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(3u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            7,
            4,
        ),
        //         Monkey 1:
        //   Starting items: 70
        //   Operation: new = old + 6
        //   Test: divisible by 17
        //     If true: throw to monkey 3
        //     If false: throw to monkey 0
        Monkey::new(
            vec![BigUint::from(70u32)],
            |old| old + BigUint::from(6u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(17u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            3,
            0,
        ),
        //         Monkey 2:
        //   Starting items: 62, 68, 56, 65, 94, 78
        //   Operation: new = old + 5
        //   Test: divisible by 2
        //     If true: throw to monkey 3
        //     If false: throw to monkey 1
        Monkey::new(
            vec![
                BigUint::from(62u32),
                BigUint::from(68u32),
                BigUint::from(56u32),
                BigUint::from(65u32),
                BigUint::from(94u32),
                BigUint::from(78u32),
            ],
            |old| old + BigUint::from(5u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(2u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            3,
            1,
        ),
        //         Monkey 3:
        //   Starting items: 89, 94, 94, 67
        //   Operation: new = old + 2
        //   Test: divisible by 19
        //     If true: throw to monkey 7
        //     If false: throw to monkey 0
        Monkey::new(
            vec![
                BigUint::from(89u32),
                BigUint::from(94u32),
                BigUint::from(94u32),
                BigUint::from(67u32),
            ],
            |old| old + BigUint::from(2u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(19u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            7,
            0,
        ),
        //         Monkey 4:
        //   Starting items: 71, 61, 73, 65, 98, 98, 63
        //   Operation: new = old * 7
        //   Test: divisible by 11
        //     If true: throw to monkey 5
        //     If false: throw to monkey 6
        Monkey::new(
            vec![
                BigUint::from(71u32),
                BigUint::from(61u32),
                BigUint::from(73u32),
                BigUint::from(65u32),
                BigUint::from(98u32),
                BigUint::from(98u32),
                BigUint::from(63u32),
            ],
            |old| old * BigUint::from(7u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(11u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            5,
            6,
        ),
        //         Monkey 5:
        //   Starting items: 55, 62, 68, 61, 60
        //   Operation: new = old + 7
        //   Test: divisible by 5
        //     If true: throw to monkey 2
        //     If false: throw to monkey 1
        Monkey::new(
            vec![
                BigUint::from(55u32),
                BigUint::from(62u32),
                BigUint::from(68u32),
                BigUint::from(61u32),
                BigUint::from(60u32),
            ],
            |old| old + BigUint::from(7u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(5u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            2,
            1,
        ),
        //         Monkey 6:
        //   Starting items: 93, 91, 69, 64, 72, 89, 50, 71
        //   Operation: new = old + 1
        //   Test: divisible by 13
        //     If true: throw to monkey 5
        //     If false: throw to monkey 2
        Monkey::new(
            vec![
                BigUint::from(93u32),
                BigUint::from(91u32),
                BigUint::from(69u32),
                BigUint::from(64u32),
                BigUint::from(72u32),
                BigUint::from(89u32),
                BigUint::from(50u32),
                BigUint::from(71u32),
            ],
            |old| old + BigUint::from(1u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(13u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            5,
            2,
        ),
        //         Monkey 7:
        //   Starting items: 76, 50
        //   Operation: new = old * old
        //   Test: divisible by 7
        //     If true: throw to monkey 4
        //     If false: throw to monkey 6
        Monkey::new(
            vec![BigUint::from(76u32), BigUint::from(50u32)],
            |old| {
                let factor = old.clone();
                old * factor
            },
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(7u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            4,
            6,
        ),
    ]);
    println!(
        "Part 1: {}",
        monkeys1.monkey_business(20, BigUint::from(3u32), BigUint::from(1u32))
    );
    let mut monkeys2: Monkeys = Monkeys(vec![
        //         Monkey 0:
        //   Starting items: 66, 71, 94
        //   Operation: new = old * 5
        //   Test: divisible by 3
        //     If true: throw to monkey 7
        //     If false: throw to monkey 4
        Monkey::new(
            vec![
                BigUint::from(66u32),
                BigUint::from(71u32),
                BigUint::from(94u32),
            ],
            |old| old * BigUint::from(5u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(3u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            7,
            4,
        ),
        //         Monkey 1:
        //   Starting items: 70
        //   Operation: new = old + 6
        //   Test: divisible by 17
        //     If true: throw to monkey 3
        //     If false: throw to monkey 0
        Monkey::new(
            vec![BigUint::from(70u32)],
            |old| old + BigUint::from(6u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(17u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            3,
            0,
        ),
        //         Monkey 2:
        //   Starting items: 62, 68, 56, 65, 94, 78
        //   Operation: new = old + 5
        //   Test: divisible by 2
        //     If true: throw to monkey 3
        //     If false: throw to monkey 1
        Monkey::new(
            vec![
                BigUint::from(62u32),
                BigUint::from(68u32),
                BigUint::from(56u32),
                BigUint::from(65u32),
                BigUint::from(94u32),
                BigUint::from(78u32),
            ],
            |old| old + BigUint::from(5u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(2u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            3,
            1,
        ),
        //         Monkey 3:
        //   Starting items: 89, 94, 94, 67
        //   Operation: new = old + 2
        //   Test: divisible by 19
        //     If true: throw to monkey 7
        //     If false: throw to monkey 0
        Monkey::new(
            vec![
                BigUint::from(89u32),
                BigUint::from(94u32),
                BigUint::from(94u32),
                BigUint::from(67u32),
            ],
            |old| old + BigUint::from(2u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(19u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            7,
            0,
        ),
        //         Monkey 4:
        //   Starting items: 71, 61, 73, 65, 98, 98, 63
        //   Operation: new = old * 7
        //   Test: divisible by 11
        //     If true: throw to monkey 5
        //     If false: throw to monkey 6
        Monkey::new(
            vec![
                BigUint::from(71u32),
                BigUint::from(61u32),
                BigUint::from(73u32),
                BigUint::from(65u32),
                BigUint::from(98u32),
                BigUint::from(98u32),
                BigUint::from(63u32),
            ],
            |old| old * BigUint::from(7u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(11u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            5,
            6,
        ),
        //         Monkey 5:
        //   Starting items: 55, 62, 68, 61, 60
        //   Operation: new = old + 7
        //   Test: divisible by 5
        //     If true: throw to monkey 2
        //     If false: throw to monkey 1
        Monkey::new(
            vec![
                BigUint::from(55u32),
                BigUint::from(62u32),
                BigUint::from(68u32),
                BigUint::from(61u32),
                BigUint::from(60u32),
            ],
            |old| old + BigUint::from(7u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(5u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            2,
            1,
        ),
        //         Monkey 6:
        //   Starting items: 93, 91, 69, 64, 72, 89, 50, 71
        //   Operation: new = old + 1
        //   Test: divisible by 13
        //     If true: throw to monkey 5
        //     If false: throw to monkey 2
        Monkey::new(
            vec![
                BigUint::from(93u32),
                BigUint::from(91u32),
                BigUint::from(69u32),
                BigUint::from(64u32),
                BigUint::from(72u32),
                BigUint::from(89u32),
                BigUint::from(50u32),
                BigUint::from(71u32),
            ],
            |old| old + BigUint::from(1u32),
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(13u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            5,
            2,
        ),
        //         Monkey 7:
        //   Starting items: 76, 50
        //   Operation: new = old * old
        //   Test: divisible by 7
        //     If true: throw to monkey 4
        //     If false: throw to monkey 6
        Monkey::new(
            vec![BigUint::from(76u32), BigUint::from(50u32)],
            |old| {
                let factor = old.clone();
                old * factor
            },
            |item, true_index, false_index| {
                if BigUint::from(0u32) == item.clone().rem(BigUint::from(7u32)) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            4,
            6,
        ),
    ]);
    let factor = BigUint::from(3u32)
        * BigUint::from(17u32)
        * BigUint::from(2u32)
        * BigUint::from(19u32)
        * BigUint::from(11u32)
        * BigUint::from(5u32)
        * BigUint::from(13u32)
        * BigUint::from(7u32);

    println!(
        "Part 2: {}",
        monkeys2.monkey_business(10000, BigUint::from(1u32), factor)
    );
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_new() {
        let monkey = Monkey::new(
            vec![BigUint::new(vec![1]), BigUint::new(vec![2])],
            |x| x.add(&BigUint::new(vec![1])),
            |x, y, _| (y, x),
            0,
            1,
        );
        assert_eq!(
            vec![BigUint::new(vec![1]), BigUint::new(vec![2])],
            monkey.starting_items
        );
        assert_eq!(
            BigUint::new(vec![2]),
            (monkey.operation)(BigUint::new(vec![1]))
        );
        assert_eq!(
            (0, BigUint::new(vec![2])),
            (monkey.test)(BigUint::new(vec![2]), 0, 1)
        );
    }

    #[test]
    fn test_monkey_compute_round() {
        let mut monkey = Monkey::new(
            vec![BigUint::new(vec![79]), BigUint::new(vec![98])],
            |old| old.mul(&BigUint::new(vec![19])),
            |item, true_index, false_index| {
                if BigUint::new(vec![0]) == item.clone().rem(&BigUint::new(vec![23])) {
                    (true_index, item)
                } else {
                    (false_index, item)
                }
            },
            2,
            3,
        );
        monkey.compute_round(BigUint::from(3u32), BigUint::from(23u32));
        // assert_eq!(
        //     vec![(3, BigUint::new(vec![500])), (3, BigUint::new(vec![620])),],
        //     monkey.compute_round(BigUint::new(vec![3]), BigUint::from(23u32))
        // );
        assert_eq!(2, monkey.inspection_count);
    }

    #[test]
    fn test_monkeys_round() {
        let mut monkeys = Monkeys(vec![
            Monkey::new(
                vec![BigUint::new(vec![79]), BigUint::new(vec![98])],
                |old| old.mul(&BigUint::new(vec![19])),
                |item, true_index, false_index| {
                    if BigUint::new(vec![0]) == item.clone().rem(&BigUint::new(vec![23])) {
                        (true_index, item)
                    } else {
                        (false_index, item)
                    }
                },
                2,
                3,
            ),
            Monkey::new(
                vec![
                    BigUint::new(vec![54]),
                    BigUint::new(vec![65]),
                    BigUint::new(vec![75]),
                    BigUint::new(vec![74]),
                ],
                |old| old.add(&BigUint::new(vec![6])),
                |item, true_index, false_index| {
                    if BigUint::new(vec![0]) == item.clone().rem(&BigUint::new(vec![19])) {
                        (true_index, item)
                    } else {
                        (false_index, item)
                    }
                },
                2,
                0,
            ),
            Monkey::new(
                vec![
                    BigUint::new(vec![79]),
                    BigUint::new(vec![60]),
                    BigUint::new(vec![97]),
                ],
                |old| {
                    let factor = old.clone();
                    old.mul(factor)
                },
                |item, true_index, false_index| {
                    if BigUint::new(vec![0]) == item.clone().rem(&BigUint::new(vec![13])) {
                        (true_index, item)
                    } else {
                        (false_index, item)
                    }
                },
                1,
                3,
            ),
            Monkey::new(
                vec![BigUint::new(vec![74])],
                |old| old.add(&BigUint::new(vec![3])),
                |item, true_index, false_index| {
                    if BigUint::new(vec![0]) == item.clone().rem(&BigUint::new(vec![17])) {
                        (true_index, item)
                    } else {
                        (false_index, item)
                    }
                },
                0,
                1,
            ),
        ]);
        let reduction_factor = BigUint::from(13u32)
            * BigUint::from(17u32)
            * BigUint::from(19u32)
            * BigUint::from(23u32);
        monkeys.round(BigUint::new(vec![3]), reduction_factor.clone());
        assert_eq!(
            vec![
                BigUint::new(vec![20]),
                BigUint::new(vec![23]),
                BigUint::new(vec![27]),
                BigUint::new(vec![26])
            ],
            monkeys.0[0].starting_items
        );
        assert_eq!(
            vec![
                BigUint::new(vec![2080]),
                BigUint::new(vec![25]),
                BigUint::new(vec![167]),
                BigUint::new(vec![207]),
                BigUint::new(vec![401]),
                BigUint::new(vec![1046])
            ],
            monkeys.0[1].starting_items
        );
        assert_eq!(vec![] as Vec<BigUint>, monkeys.0[2].starting_items);
        assert_eq!(vec![] as Vec<BigUint>, monkeys.0[3].starting_items);
    }

    #[test]
    fn test_monkeys_monkey_business() {
        let mut monkeys = Monkeys(vec![
            Monkey::new(
                vec![BigUint::new(vec![79]), BigUint::new(vec![98])],
                |old| old.mul(&BigUint::new(vec![19])),
                |item, true_index, false_index| {
                    if BigUint::new(vec![0]) == item.clone().rem(&BigUint::new(vec![23])) {
                        (true_index, item)
                    } else {
                        (false_index, item)
                    }
                },
                2,
                3,
            ),
            Monkey::new(
                vec![
                    BigUint::new(vec![54]),
                    BigUint::new(vec![65]),
                    BigUint::new(vec![75]),
                    BigUint::new(vec![74]),
                ],
                |old| old.add(&BigUint::new(vec![6])),
                |item, true_index, false_index| {
                    if BigUint::new(vec![0]) == item.clone().rem(&BigUint::new(vec![19])) {
                        (true_index, item)
                    } else {
                        (false_index, item)
                    }
                },
                2,
                0,
            ),
            Monkey::new(
                vec![
                    BigUint::new(vec![79]),
                    BigUint::new(vec![60]),
                    BigUint::new(vec![97]),
                ],
                |old| {
                    let factor = old.clone();
                    old.mul(factor)
                },
                |item, true_index, false_index| {
                    if BigUint::new(vec![0]) == item.clone().rem(&BigUint::new(vec![13])) {
                        (true_index, item)
                    } else {
                        (false_index, item)
                    }
                },
                1,
                3,
            ),
            Monkey::new(
                vec![BigUint::new(vec![74])],
                |old| old.add(&BigUint::new(vec![3])),
                |item, true_index, false_index| {
                    if BigUint::new(vec![0]) == item.clone().rem(&BigUint::new(vec![17])) {
                        (true_index, item)
                    } else {
                        (false_index, item)
                    }
                },
                0,
                1,
            ),
        ]);
        let reduction_factor = BigUint::from(13u32)
            * BigUint::from(17u32)
            * BigUint::from(19u32)
            * BigUint::from(23u32);
        assert_eq!(
            10605,
            monkeys.monkey_business(20, BigUint::new(vec![3]), reduction_factor.clone())
        );
    }
}
