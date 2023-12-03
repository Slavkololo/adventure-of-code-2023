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
    match subsequence {
        s if find_subsequence(s, NUM_ARRAY[0]).is_some() => Some(1),
        s if find_subsequence(s, NUM_ARRAY[1]).is_some() => Some(2),
        s if find_subsequence(s, NUM_ARRAY[2]).is_some() => Some(3),
        s if find_subsequence(s, NUM_ARRAY[3]).is_some() => Some(4),
        s if find_subsequence(s, NUM_ARRAY[4]).is_some() => Some(5),
        s if find_subsequence(s, NUM_ARRAY[5]).is_some() => Some(6),
        s if find_subsequence(s, NUM_ARRAY[6]).is_some() => Some(7),
        s if find_subsequence(s, NUM_ARRAY[7]).is_some() => Some(8),
        s if find_subsequence(s, NUM_ARRAY[8]).is_some() => Some(9),
        _ => None,
    }
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splited = contents.split("\n");
    let mut sum = 0;

    for line in splited {
        let line = line.as_bytes();
        let mut first_int = 0;
        let mut last_int = 0;
        let mut i = 0;
        let mut last_i = 0;

        while i < line.len() {
            if let Some(num) = find_digit(&line[0..i]) {
                first_int = num;
                break;
            } else if let Some(num) = (line[i] as char).to_digit(10) {
                first_int = num;
                break;
            }
            i += 1;
        }

        while last_i < line.len() {
            if let Some(num) = find_digit(&line[line.len() - 1 - last_i..line.len()]) {
                last_int = num;
                break;
            } else if let Some(num) = (line[line.len() - 1 - last_i] as char).to_digit(10) {
                last_int = num;
                break;
            }
            last_i += 1;
        }

        sum += first_int * 10 + last_int
    }

    println!("{sum}")
}
