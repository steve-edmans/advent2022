use std::fs;
use advent2022::DailyChallenge;

pub struct Three {}

impl DailyChallenge for Three {
    fn run(&self) {
        println!("Day Three");
        let rucksacks: Vec<Rucksack> = fs::read_to_string("contents/day_three.txt")
            .expect("Should have been able to read file")
            .lines()
            // .take(12)
            .map(|rucksack_code| Rucksack::from(rucksack_code).unwrap())
            .collect();

        let part_one_data: Vec<u16> = rucksacks.iter()
            .flat_map(|rucksack| rucksack.priority())
            .collect();

        let part_one: u16 = part_one_data.iter().sum();
        println!("The result of part one is {:?}", part_one);

        let mut rucksacks = rucksacks.iter();
        let mut part_two = 0;
        while let (Some(first), Some(second), Some(third)) = (rucksacks.next(), rucksacks.next(), rucksacks.next()) {
            let badge_type =  Rucksack::badge_item_type(first, second, third).unwrap();
            let priority = Rucksack::priority_from_char(badge_type).unwrap();
            part_two += priority;
        }
        println!("The result of part_two is {:?}", part_two);
    }
}

#[derive(Debug)]
struct Rucksack {
    first_compartment: String,
    second_compartment: String,
}

impl Rucksack {
    fn from(code: &str) -> Result<Rucksack, &'static str> {
        let (first_component, second_compartment) = code.split_at(code.len() / 2);
        Ok(Rucksack {
            first_compartment: String::from(first_component),
            second_compartment: String::from(second_compartment)
        })
    }

    fn badge_item_type(first: &Rucksack, second: &Rucksack, third: &Rucksack) -> Option<char> {
        if let Some(badge_item_type) = Rucksack::check_compartment(&first.first_compartment, second, third) {
            return Some(badge_item_type)
        } else {
            return Rucksack::check_compartment(&first.second_compartment, second, third);
        }
    }

    fn check_compartment(compartment: &String, second: &Rucksack, third: &Rucksack) -> Option<char> {
        for item in compartment.chars() {
            if second.first_compartment.contains(item) || second.second_compartment.contains(item) {
                if third.first_compartment.contains(item) || third.second_compartment.contains(item) {
                    return Some(item);
                }
            }
        }
        None
    }

    fn first_shared_item(&self) -> Option<char> {
        for item in self.first_compartment.chars() {
            if self.second_compartment.contains(item) {
                return Some(item);
            }
        }

        None
    }

    fn priority_from_char(code: char) -> Option<u16> {
        match code {
            'a'..='z' => {
                let ascii = code as u16;
                Some(ascii - 96)
            },
            'A'..='Z' => {
                let ascii = code as u16;
                Some(ascii - 38)
            }
            _ => None
        }
    }

    fn priority(&self) -> Option<u16> {
        self.first_shared_item().map(|shared_item| Rucksack::priority_from_char(shared_item).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_rucksack() {
        let rucksack = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        assert_eq!(rucksack.first_compartment, "vJrwpWtwJgWr");
        assert_eq!(rucksack.second_compartment, "hcsFMMfFFhFp");
    }

    #[test]
    fn find_first_shared_item() {
        let rucksack = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        assert_eq!(rucksack.first_shared_item(), Some('p'));
    }

    #[test]
    fn priority_of_an_illegal_char() {
        assert_eq!(Rucksack::priority_from_char(';'), None);
    }
    #[test]
    fn priority_of_a() {
        assert_eq!(Rucksack::priority_from_char('a'), Some(1));
    }

    #[test]
    fn priority_of_j() {
        assert_eq!(Rucksack::priority_from_char('j'), Some(10));
    }

    #[test]
    fn priority_of_z() {
        assert_eq!(Rucksack::priority_from_char('z'), Some(26));
    }

    #[test]
    fn priority_of_uppercase_a() {
        assert_eq!(Rucksack::priority_from_char('A'), Some(27));
    }

    #[test]
    fn priority_of_uppercase_j() {
        assert_eq!(Rucksack::priority_from_char('J'), Some(36));
    }

    #[test]
    fn priority_of_uppercase_z() {
        assert_eq!(Rucksack::priority_from_char('Z'), Some(52));
    }

    #[test]
    fn first_rucksack() {
        let rucksack = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        assert_eq!(rucksack.priority(), Some(16));
    }

    #[test]
    fn second_rucksack() {
        let rucksack = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap();
        assert_eq!(rucksack.priority(), Some(38));
    }

    #[test]
    fn third_rucksack() {
        let rucksack = Rucksack::from("PmmdzqPrVvPwwTWBwg").unwrap();
        assert_eq!(rucksack.priority(), Some(42));
    }

    #[test]
    fn forth_rucksack() {
        let rucksack = Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap();
        assert_eq!(rucksack.priority(), Some(22));
    }

    #[test]
    fn fifth_rucksack() {
        let rucksack = Rucksack::from("ttgJtRGJQctTZtZT").unwrap();
        assert_eq!(rucksack.priority(), Some(20));
    }

    #[test]
    fn sixth_rucksack() {
        let rucksack = Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap();
        assert_eq!(rucksack.priority(), Some(19));
    }

    #[test]
    fn check_first_group_of_rucksacks() {
        let first = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        let second = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap();
        let third = Rucksack::from("PmmdzqPrVvPwwTWBwg").unwrap();

        assert_eq!(Rucksack::badge_item_type(&first, &second, &third), Some('r'));
    }

    #[test]
    fn check_second_group_of_rucksacks() {
        let first = Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap();
        let second = Rucksack::from("ttgJtRGJQctTZtZT").unwrap();
        let third = Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap();

        assert_eq!(Rucksack::badge_item_type(&first, &second, &third), Some('Z'));
    }
}