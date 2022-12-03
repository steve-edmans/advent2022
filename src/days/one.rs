use std::fs;
use advent2022::DailyChallenge;

struct Accumulator {
    total_calories_per_elf: Vec<u32>,
    current_total: u32
}

pub struct One {}

impl DailyChallenge for One {
    fn run(&self) {
        println!("Day One");
        let values = read_file();
        let mut per_elf_calories = extract_totals(&values);
        per_elf_calories.sort();
        per_elf_calories.reverse();
        println!("The result of part one is {:?}", per_elf_calories[0]);
        let top_three:u32 = (&per_elf_calories[0..3]).iter().sum();
        println!("The result of part two is {:#?}", top_three);
    }
}

fn extract_totals(values: &Vec<Option<u32>>) -> Vec<u32> {
    let initial_accumulator = Accumulator {
        total_calories_per_elf: vec![],
        current_total: 0,
    };
    let data = values.iter()
        .fold(initial_accumulator, |accumlator, item| {
            match item {
                Some(value) => {
                    Accumulator {
                        current_total: accumlator.current_total + value,
                        ..accumlator
                    }
                }
                None => {
                    let mut totals = accumlator.total_calories_per_elf;
                    totals.push(accumlator.current_total);
                    Accumulator {
                        current_total: 0,
                        total_calories_per_elf: totals
                    }
                }
            }
        });
    data.total_calories_per_elf
}

fn read_file() -> Vec<Option<u32>> {
    fs::read_to_string("contents/day_one.txt")
        .expect("Should have been able to read file")
        .lines()
        .map(|x| x.parse::<u32>().ok())
        .collect()
}
