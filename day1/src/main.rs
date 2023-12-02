use core::fmt::Display;
use std::fs;

enum Digits {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Digits::One => write!(f, "one"),
            Digits::Two => write!(f, "two"),
            Digits::Three => write!(f, "three"),
            Digits::Four => write!(f, "four"),
            Digits::Five => write!(f, "five"),
            Digits::Six => write!(f, "six"),
            Digits::Seven => write!(f, "seven"),
            Digits::Eight => write!(f, "eight"),
            Digits::Nine => write!(f, "nine"),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splited = contents.split("\n");
    let mut sum = 0;

    for line in splited {
        let mut first_int = 0;
        let mut last_int = 0;

        for c in line.chars() {
            if let Some(num) = c.to_digit(10) {
                if first_int == 0 {
                    first_int = num;
                }
                last_int = num;
            }
        }
        sum += first_int * 10 + last_int
    }

    println!("{sum}")
}
