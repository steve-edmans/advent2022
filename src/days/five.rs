use std::fs;
use advent2022::DailyChallenge;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::days::five::ColumnRead::*;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Five {}

impl DailyChallenge for Five {
    fn run(&self) {
        println!("Day Five");
        let data = fs::read_to_string("contents/day_five.txt")
            .expect("Should have been able to read file");
        let lines = data.lines();
        let possible_stacks: Vec<&str> = lines
            .clone()
            .take_while(|&line| line.len() > 0)
            .collect();
        let stacks = Five::extract_stack_of_crates(possible_stacks);
        let possible_rows: Vec<MovementOrder> = lines
            .clone()
            .skip_while(|&line| line.len() > 0)
            .skip(1)
            .map(|order_text| MovementOrder::from(order_text).unwrap())
            .collect();
        let mut part_one_stacks = stacks.clone();
        for row in &possible_rows {
            Five::process(&mut part_one_stacks, row);
        }
        let part_one = Five::find_head(&part_one_stacks);
        println!("The result of part one is {:?}", part_one);

        let mut part_two_stacks = stacks.clone();
        for row in &possible_rows {
            Five::process_9001(&mut part_two_stacks, row);
        }
        let part_one = Five::find_head(&part_two_stacks);
        println!("The result of part one is {:?}", part_one);
    }
}

impl Five {
    fn decode_stack_row(row: &str) -> Vec<ColumnRead> {

        let mut data: Vec<ColumnRead> = Vec::new();
        let mut column = 0;

        loop {
            column += 1;

            let crate_in_column = Five::next_column(row, column);

            match crate_in_column {
                RowFinished => break,
                column => data.push(column),
            }

            if column > 15 {
                break;
            }
        }

        data
    }

    fn next_column(row: &str, column: usize) -> ColumnRead {
        match row.chars().nth((column * 4) - 3) {
            Some(' ') => EmptyCrateFound,
            Some(crate_code) => ColumnRead::FoundWithCrate { crate_code },
            None => RowFinished,
        }
    }

    fn extract_stack_of_crates(lines: Vec<&str>) -> HashMap<u8, Box<Vec<char>>> {
        let mut stack_of_crates: HashMap<u8, Box<Vec<char>>> = HashMap::new();

        for line in lines {
            if let Some(first_char) = line.trim().get(0..1) {
                if first_char == "[" {
                    let next_row = Five::decode_stack_row(line);
                    for (data, column) in next_row.iter().zip(1..100).into_iter() {
                        match &data {
                            EmptyCrateFound => {
                                stack_of_crates.entry(column).or_insert(Box::new(Vec::new()));
                            },
                            FoundWithCrate { crate_code } => {
                                stack_of_crates
                                    .entry(column)
                                    .and_modify(|stack| stack.insert(0,*crate_code))
                                    .or_insert(Box::new(vec![*crate_code]));
                            },
                            RowFinished => ()
                        }
                    }
                }
            }
        }

        return stack_of_crates;
    }

    fn process(stacks: &mut HashMap<u8, Box<Vec<char>>>, &order: &MovementOrder) {
        match order {
            MovementOrder { num_crates: num, from_stack: from, to_stack: to} => {
                for _ in 0..num {
                    let mut stack_from = stacks[&from].to_vec();
                    let mut stack_to = stacks[&to].to_vec();
                    let crate_code = stack_from.pop();
                    stack_to.push(crate_code.unwrap());
                    *stacks.get_mut(&from).unwrap() = Box::new(stack_from);
                    *stacks.get_mut(&to).unwrap() = Box::new(stack_to);
                }
            }
        }
    }

    fn process_9001(stacks: &mut HashMap<u8, Box<Vec<char>>>, &order: &MovementOrder) {
        match order {
            MovementOrder { num_crates: num, from_stack: from, to_stack: to } => {
                let mut crates_to_move: Vec<char> = Vec::new();
                let mut stack_from = stacks[&from].to_vec();
                let mut stack_to = stacks[&to].to_vec();

                for _ in 0..num {
                    let crate_code = stack_from.pop().unwrap();
                    crates_to_move.insert(0, crate_code);
                }

                stack_to.append(&mut crates_to_move);

                *stacks.get_mut(&from).unwrap() = Box::new(stack_from);
                *stacks.get_mut(&to).unwrap() = Box::new(stack_to);
            }
        }
    }

    fn find_head(stack: &HashMap<u8, Box<Vec<char>>>) -> String {
        let mut top_of_stack: Vec<char> = Vec::new();
        let mut keys: Vec<&u8> = stack.keys().collect();
        keys.sort();

        for key in keys {
            let mut stack = stack.get(key).unwrap().clone();
            let crate_code = stack.pop().unwrap_or(' ');
            top_of_stack.push(crate_code);
        }

        top_of_stack.iter().collect()
    }
}

#[derive(Debug, PartialEq)]
enum ColumnRead {
    FoundWithCrate { crate_code: char },
    EmptyCrateFound,
    RowFinished
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct MovementOrder {
    num_crates: u8,
    from_stack: u8,
    to_stack: u8
}

impl Display for MovementOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {} crates from statck {} to stack {}", self.num_crates, self.from_stack, self.to_stack)
    }
}

impl MovementOrder {
    fn from(order_text: &str) -> Result<MovementOrder, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d{1,2}) from (\d) to (\d)$").unwrap();
        }

        if let Some(capture) = RE.captures_iter(order_text).next() {
            let num_crates = (&capture[1]).parse::<u8>().unwrap();
            let from = (&capture[2]).parse::<u8>().unwrap();
            let to = (&capture[3]).parse::<u8>().unwrap();
            return Ok(MovementOrder { num_crates: num_crates, from_stack: from, to_stack: to });
        }

        Err("Unable to process")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_stack_of_crates() {
        let data = vec![
            "    [V] [G]             [H]        ",
            "[Z] [H] [Z]         [T] [S]        ",
            " 1   2   3   4   5   6   7   8   9 "
        ];
        let actual = Five::extract_stack_of_crates(data);
        assert_eq!(actual.len(), 9);
        assert_eq!(actual.get(&1).unwrap().len(), 1);
        assert_eq!(actual.get(&2).unwrap().len(), 2);
        assert_eq!(actual.get(&3).unwrap().len(), 2);
        assert_eq!(actual.get(&4).unwrap().len(), 0);
        assert_eq!(actual.get(&5).unwrap().len(), 0);
        assert_eq!(actual.get(&6).unwrap().len(), 1);
        assert_eq!(actual.get(&7).unwrap().len(), 2);
        assert_eq!(actual.get(&8).unwrap().len(), 0);
        assert_eq!(actual.get(&9).unwrap().len(), 0);
        assert_eq!(actual.get(&10).is_none(), true);
    }

    #[test]
    fn find_first_column_from_row() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::next_column(&row, 1);
        assert_eq!(actual, FoundWithCrate { crate_code: 'Z' });
    }

    #[test]
    fn find_second_column_from_row() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::next_column(&row, 2);
        assert_eq!(actual, FoundWithCrate { crate_code: 'H' });
    }

    #[test]
    fn find_third_column_from_row() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::next_column(&row, 3);
        assert_eq!(actual, FoundWithCrate { crate_code: 'Z' });
    }

    #[test]
    fn find_forth_column_from_row() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::next_column(&row, 4);
        assert_eq!(actual, EmptyCrateFound);
    }

    #[test]
    fn find_fifth_column_from_row() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::next_column(&row, 5);
        assert_eq!(actual, EmptyCrateFound);
    }

    #[test]
    fn find_sixth_column_from_row() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::next_column(&row, 6);
        assert_eq!(actual, FoundWithCrate { crate_code: 'T' });
    }

    #[test]
    fn find_seventh_column_from_row() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::next_column(&row, 7);
        assert_eq!(actual, FoundWithCrate { crate_code: 'S' });
    }

    #[test]
    fn find_eight_column_from_row() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::next_column(&row, 8);
        assert_eq!(actual, EmptyCrateFound);
    }

    #[test]
    fn find_ninth_column_from_row() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::next_column(&row, 9);
        assert_eq!(actual, EmptyCrateFound);
    }

    #[test]
    fn find_tenth_column_from_row() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::next_column(&row, 10);
        assert_eq!(actual, RowFinished);
    }

    #[test]
    fn determine_first_layer_of_crates() {
        let row = "    [V] [G]             [H]        ";
        let actual = Five::decode_stack_row(&row);
        assert_eq!(actual.len(), 9);
        assert_eq!(*actual.get(0).unwrap(), EmptyCrateFound);
        assert_eq!(*actual.get(1).unwrap(), FoundWithCrate { crate_code: 'V' });
        assert_eq!(*actual.get(2).unwrap(), FoundWithCrate { crate_code: 'G' });
        assert_eq!(*actual.get(3).unwrap(), EmptyCrateFound);
        assert_eq!(*actual.get(4).unwrap(), EmptyCrateFound);
        assert_eq!(*actual.get(5).unwrap(), EmptyCrateFound);
        assert_eq!(*actual.get(6).unwrap(), FoundWithCrate { crate_code: 'H' });
        assert_eq!(*actual.get(7).unwrap(), EmptyCrateFound);
        assert_eq!(*actual.get(8).unwrap(), EmptyCrateFound);
    }

    #[test]
    fn determine_second_layer_of_crates() {
        let row = "[Z] [H] [Z]         [T] [S]        ";
        let actual = Five::decode_stack_row(&row);
        assert_eq!(actual.len(), 9);
        assert_eq!(*actual.get(0).unwrap(), FoundWithCrate { crate_code: 'Z' });
        assert_eq!(*actual.get(1).unwrap(), FoundWithCrate { crate_code: 'H' });
        assert_eq!(*actual.get(2).unwrap(), FoundWithCrate { crate_code: 'Z' });
        assert_eq!(*actual.get(3).unwrap(), EmptyCrateFound);
        assert_eq!(*actual.get(4).unwrap(), EmptyCrateFound);
        assert_eq!(*actual.get(5).unwrap(), FoundWithCrate { crate_code: 'T' });
        assert_eq!(*actual.get(6).unwrap(), FoundWithCrate { crate_code: 'S' });
        assert_eq!(*actual.get(7).unwrap(), EmptyCrateFound);
        assert_eq!(*actual.get(8).unwrap(), EmptyCrateFound);
    }

    #[test]
    fn extract_movement_command() {
        let sample_movement_order = "move 13 from 2 to 5";
        let expected = MovementOrder { num_crates: 13, from_stack: 2, to_stack: 5 };

        let actual = MovementOrder::from(sample_movement_order);
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn extract_smaller_movement_command() {
        let sample_movement_order = "move 3 from 2 to 5";
        let expected = MovementOrder { num_crates: 3, from_stack: 2, to_stack: 5 };

        let actual = MovementOrder::from(sample_movement_order);
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn fail_to_extract_movement_command() {
        let sample_movement_order = "INVALID ORDER";

        let actual = MovementOrder::from(sample_movement_order);
        assert_eq!(actual, Err("Unable to process"));
    }

    #[test]
    fn find_top_elements_on_each_crate() {
        let data = vec![
            "    [D]     ",
            "[N] [C]     ",
            "[Z] [M] [P] ",
            " 1   2   3  "
        ];
        let stack = Five::extract_stack_of_crates(data);
        assert_eq!(Five::find_head(&stack), "NDP");
    }

    #[test]
    fn check_test_movements2() {
        let data = vec![
            "    [D]     ",
            "[N] [C]     ",
            "[Z] [M] [P] ",
            " 1   2   3  "
        ];
        let mut stacks = Five::extract_stack_of_crates(data);

        let first_order = MovementOrder { num_crates: 1, from_stack: 2, to_stack: 1 };
        let second_order = MovementOrder { num_crates: 3, from_stack: 1, to_stack: 3};
        let third_order = MovementOrder { num_crates: 2, from_stack: 2, to_stack: 1};
        let forth_order = MovementOrder { num_crates: 1, from_stack: 1, to_stack: 2};

        println!("Before we have {:?}", stacks);
        Five::process(&mut stacks, &first_order);
        println!("After first step we have {:?}", stacks);
        Five::process(&mut stacks, &second_order);
        println!("After second step we have {:?}", stacks);
        Five::process(&mut stacks, &third_order);
        println!("After third step we have {:?}", stacks);
        Five::process(&mut stacks, &forth_order);
        println!("After forth step we have {:?}", stacks);

        assert_eq!(Five::find_head(&stacks), "CMZ");
    }

    #[test]
    fn test_crate_mover_9001() {
        let data = vec![
            "    [D]     ",
            "[N] [C]     ",
            "[Z] [M] [P] ",
            " 1   2   3  "
        ];
        let mut stacks = Five::extract_stack_of_crates(data);

        let first_order = MovementOrder { num_crates: 1, from_stack: 2, to_stack: 1 };
        let second_order = MovementOrder { num_crates: 3, from_stack: 1, to_stack: 3};
        let third_order = MovementOrder { num_crates: 2, from_stack: 2, to_stack: 1};
        let forth_order = MovementOrder { num_crates: 1, from_stack: 1, to_stack: 2};

        assert_eq!(Five::find_head(&stacks), "NDP");

        Five::process_9001(&mut stacks, &first_order);
        assert_eq!(Five::find_head(&stacks), "DCP");
        Five::process_9001(&mut stacks, &second_order);
        assert_eq!(Five::find_head(&stacks), " CD");
        Five::process_9001(&mut stacks, &third_order);
        assert_eq!(Five::find_head(&stacks), "C D");
        Five::process_9001(&mut stacks, &forth_order);

        assert_eq!(Five::find_head(&stacks), "MCD");
    }
}
