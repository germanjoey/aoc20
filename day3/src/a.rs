use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn mark_contested(s: &String, claims: &mut HashMap<String, i32>) {
	// #1327 @ 127,459: 22x19
	let pieces = s.split(" ").collect::<Vec<&str>>();

	let colonless_loc = pieces[2][0..pieces[2].len()-1].to_string();
	let loc = colonless_loc.split(",").collect::<Vec<&str>>();
	let start_x: i32 = loc[0].to_string().parse::<i32>().unwrap();
	let start_y: i32 = loc[1].to_string().parse::<i32>().unwrap();

	let dim_str = pieces[3].to_string();
	let dim = dim_str.split("x").collect::<Vec<&str>>();
	let width: i32 = dim[0].to_string().parse::<i32>().unwrap();
	let height: i32 = dim[1].to_string().parse::<i32>().unwrap();

	for x in start_x..(start_x+width) {
		for y in start_y..(start_y+height) {
			let loc = format!("{}x{}", x.to_string(), y.to_string());
			let count = claims.entry(loc).or_insert(0);
			*count += 1;
		}
	}
}

fn main() {
	let mut claims: HashMap<String, i32> = HashMap::new();

	let f = File::open("input.txt").unwrap();
	let file = BufReader::new(&f);

	for line in file.lines() {
		mark_contested(&line.unwrap(), &mut claims);
	}

	let mut total_contested = 0;

	for (_loc, claim_count) in &claims {
		if claim_count > &1 {
			total_contested += 1;
		}
	}

	println!("total contested inches: {}", total_contested);
}
