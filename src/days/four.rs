use std::fs;
use std::ops::Range;
use advent2022::DailyChallenge;

pub struct Four {}

impl DailyChallenge for Four {
    fn run(&self) {
        println!("Day Four");
        let section_assignments: Vec<Assignments> = fs::read_to_string("contents/day_four.txt")
            .expect("Should have been able to read file")
            .lines()
            .map(|x| Assignments::from(x).unwrap())
            .collect();

        let part_one: usize = section_assignments
            .iter()
            .filter(|&assignment| assignment.fully_contains())
            .count();
        println!("The result of part one is {:?}", part_one);

        let part_two = section_assignments
            .iter()
            .filter(|&assignment| assignment.overlaps())
            .count();
        println!("The result of part two is {:?}", part_two);
    }
}

#[derive(Debug, PartialEq)]
struct Assignments { first: Range<u32>, second: Range<u32> }

impl Assignments {
    fn from(code: &str) -> Result<Assignments, &'static str> {
        fn get_range(aaa: &str) -> Range<u32> {
            let mut parts = aaa.split("-").map(|x| x.parse::<u32>().unwrap());
            let start = parts.next().unwrap();
            let end = parts.next().unwrap();
            std::ops::Range { start: start, end: end }
        }

        let section_assignments: Vec<Range<u32>> = code
            .split(",")
            .map(|part| {
                get_range(part)
            }).collect();

        if let Some(first_section) = section_assignments.first() {
            if let Some(second_section) = section_assignments.get(1) {
                return Ok( Assignments {
                    first: first_section.clone(),
                    second: second_section.clone()
                })
            }
        }

        Err("Error")
    }

    fn fully_contains(&self) -> bool {
        fn inner_contain(first: &Range<u32>, second: &Range<u32>) -> bool {
            if first.start <= second.start {
                if first.end >= second.end {
                    return true;
                }
            }
            false
        }
        inner_contain(&self.first, &self.second) || inner_contain(&self.second, &self.first)
    }

    fn overlaps(&self) -> bool {
        fn inner_overlap(first: &Range<u32>, second: &Range<u32>) -> bool {
            if first.start <= second.start && second.start <= first.end {
                return true;
            }
            false
        }
        inner_overlap(&self.first, &self.second) || inner_overlap(&self.second, &self.first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_assignments() {
        let expected: Assignments = Assignments { first: 2..4, second: 6..8 };
        let actual = Assignments::from("2-4,6-8");
        assert_eq!(actual, Ok(expected))
    }

    #[test]
    fn first_fully_contains_second() {
        let actual = Assignments::from("2-8,3-7").unwrap();
        assert_eq!(actual.fully_contains(), true);
    }

    #[test]
    fn second_fully_contains_first() {
        let actual = Assignments::from("3-7,2-8").unwrap();
        assert_eq!(actual.fully_contains(), true);
    }

    #[test]
    fn fully_contain_overlap_start() {
        let actual = Assignments::from("3-7,2-4").unwrap();
        assert_eq!(actual.fully_contains(), false);
    }

    #[test]
    fn fully_contain_overlap_end() {
        let actual = Assignments::from("3-7,5-8").unwrap();
        assert_eq!(actual.fully_contains(), false);
    }

    #[test]
    fn fully_contain_disjoint() {
        let actual = Assignments::from("3-4,5-8").unwrap();
        assert_eq!(actual.fully_contains(), false);
    }

    #[test]
    fn first_fully_overlaps_second() {
        let actual = Assignments::from("2-8,3-7").unwrap();
        assert_eq!(actual.overlaps(), true);
    }

    #[test]
    fn second_fully_overlaps_first() {
        let actual = Assignments::from("3-7,2-8").unwrap();
        assert_eq!(actual.overlaps(), true);
    }

    #[test]
    fn overlap_start() {
        let actual = Assignments::from("3-7,2-4").unwrap();
        assert_eq!(actual.overlaps(), true);
    }

    #[test]
    fn overlap_end() {
        let actual = Assignments::from("3-7,5-8").unwrap();
        assert_eq!(actual.overlaps(), true);
    }

    #[test]
    fn overlap_disjoint() {
        let actual = Assignments::from("52-52,3-51").unwrap();
        assert_eq!(actual.overlaps(), false);
    }
}
