mod days;

use advent2022::DailyChallenge;
use days::one::One;
use days::two::Two;

fn main() {
    println!("Advent of code 2022");
    let days: Vec<Box<dyn DailyChallenge>> = vec![
        Box::new(One {}),
        Box::new(Two {})
    ];
    for day in days {
        println!("We have a day");
        day.run();
    }
}
