mod days;

use advent2022::DailyChallenge;
use days::one::One;
use days::two::Two;
use days::three::Three;
use days::four::Four;
use days::five::Five;

fn main() {
    println!("Advent of code 2022");
    let days: Vec<Box<dyn DailyChallenge>> = vec![
        Box::new(One {}),
        Box::new(Two {}),
        Box::new(Three {}),
        Box::new(Four {}),
        Box::new(Five {}),
    ];
    for day in days {
        day.run();
    }
}
