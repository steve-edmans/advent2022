use std::fs;
use advent2022::DailyChallenge;

pub struct Three {}

impl DailyChallenge for Three {
    fn run(&self) {
        println!("Day Three");
        let data: Vec<u16> = fs::read_to_string("contents/day_three.txt")
            .expect("Should have been able to read file")
            .lines()
            // .take(10)
            .map(|rucksack_code| Rucksack::from(rucksack_code))
            .flat_map(|rucksack| rucksack.unwrap().priority())
            .collect();

        let part_one: u16 = data.iter().sum();
        println!("The result of part one is {:?}", part_one);
    }
}

#[derive(Debug)]
struct Rucksack<'a> {
    first_compartment: &'a str,
    second_compartment: &'a str,
}

impl Rucksack<'_> {
    fn from(code: &str) -> Result<Rucksack, &'static str> {
        let (first_component, second_compartment) = code.split_at(code.len() / 2);
        Ok(Rucksack { first_compartment: first_component, second_compartment: second_compartment })
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
}