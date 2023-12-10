use std::fs;

const NUM_ARRAY: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn find_subsequence<'a>(haystack: &'a [u8], needle: &'a [u8]) -> Option<&'a [u8]> {
    haystack
        .windows(needle.len())
        .find(|haystack| needle == *haystack)
}

fn find_digit(subsequence: &[u8]) -> Option<u32> {
    NUM_ARRAY
        .iter()
        .position(|&x| find_subsequence(subsequence, x).is_some())
        .map(|x| (x + 1) as u32)
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splited = contents.lines();
    let mut sum = 0;

    for line in splited {
        let line = line.as_bytes();
        let mut first_int = 0;
        let mut last_int = 0;

        for i in 0..line.len() {
            if let Some(num) = find_digit(&line[0..i]) {
                first_int = num;
                break;
            } else if let Some(num) = (line[i] as char).to_digit(10) {
                first_int = num;
                break;
            }
        }

        for i in 0..line.len() {
            if let Some(num) = find_digit(&line[line.len() - 1 - i..line.len()]) {
                last_int = num;
                break;
            } else if let Some(num) = (line[line.len() - 1 - i] as char).to_digit(10) {
                last_int = num;
                break;
            }
        }

        sum += first_int * 10 + last_int
    }

    println!("{sum}")
}
