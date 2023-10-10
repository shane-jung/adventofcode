use std::cmp;
use std::io::prelude::*;
use std::io::BufReader;
use std::{fs::File, path::Path};
fn main() {
    let path: &Path = Path::new("inputs/day_01.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open file {}, {}", display, why),
        Ok(file) => file,
    };

    let mut calories = 0;
    let mut max_calories = 0;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let number = line.unwrap();
        if number == "" {
            max_calories = cmp::max(calories, max_calories);
            calories = 0;
        } else {
            calories += number.parse::<i32>().unwrap();
        }
    }
    max_calories = cmp::max(calories, max_calories);
    println!(
        "The elf carrying the most calories is carrying {} calories of food.",
        max_calories
    );
}
