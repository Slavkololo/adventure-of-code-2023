use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splited = contents.lines();
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
