use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

fn main() {   
	let mut seen = HashSet::new();
	let mut total = 0;
	let mut first_seen: i32 = 0;
	let mut first_seen_was_seen = false;

	loop {
		let f = File::open("input.txt").unwrap();
		let file = BufReader::new(&f);

		for line in file.lines() {
			// convert to line to an integer; te result is 0 if there's an error
			let i: i32 = line.unwrap().parse::<i32>().unwrap();
			
			total += i;
		
			if seen.contains(&total) && (first_seen_was_seen == false) {
				first_seen_was_seen = true;
				first_seen = total;
				break;
			}

			seen.insert(total);
		}

		if first_seen_was_seen {
			break;
		}
	}

	println!("total: {}", total.to_string());
	println!("the first repeated frequency seen was: {}.", first_seen);
}
