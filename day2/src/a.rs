use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn count_chars(chars: &Vec<char>) -> HashMap<char, i32> {
    let mut counts: HashMap<char, i32> = HashMap::new();
    for char in chars {
        *counts.entry(*char).or_insert(0) += 1
    }

    return counts;
}

fn check_count(i: i32, counts: &HashMap<char, i32>) -> bool {
    for (_char, count) in counts {
        if *count == i {
            return true;
        }
    }

    return false;
}

fn main() {   
    let mut total_doubles = 0;
    let mut total_triples = 0;

    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);

    for line in file.lines() {
        let chars: Vec<char> = line.unwrap().chars().collect();
        let counts = count_chars(&chars);

        if check_count(2, &counts) {
            total_doubles += 1;
        }
        if check_count(3, &counts) {
            total_triples += 1;
        }

    }

    println!("checksum: {}", total_doubles * total_triples);
}
