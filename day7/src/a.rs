use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;

fn parse_line(line: &String) -> (char, char) {
	let a = &line[5..6];
	let b = &line[36..37];

	let a_chr = a.chars().next().unwrap();
	let b_chr = b.chars().next().unwrap();

	return (a_chr, b_chr);
}

fn parse_input()-> HashMap<char, HashSet<char>> {
	let mut dependencies: HashMap<char, HashSet<char>> = HashMap::new();
	
	let f = File::open("input.txt").unwrap();
	let file = BufReader::new(&f);

	for line in file.lines() {
		let (a, b) = parse_line(&line.unwrap());
		dependencies.entry(a).or_insert(HashSet::new());
		let mut dep = dependencies.entry(b).or_insert(HashSet::new());
		dep.insert(a);
	}

	return dependencies;
}

fn main() {
	let mut dependencies = parse_input();
	
	let mut final_order: Vec<char> = vec![];
	let mut ready: Vec<char> = vec![];
	let mut done: HashSet<char> = HashSet::new();

	loop {
		for (dependent, dependency_set) in dependencies.iter_mut() {
			for letter in &done {
				dependency_set.remove(&letter);
			}

			if dependency_set.len() == 0 {
				ready.push(*dependent);
			}
		}

		for letter in &ready {
			dependencies.remove(&letter);
		}

		ready.sort_by(|a, b| b.cmp(a));
		let next_letter: char = ready.pop().unwrap();
		final_order.push(next_letter);
		done.insert(next_letter);

		if (dependencies.len() == 0) && (ready.len() == 0) {
			break;
		}
	}

	let final_order_string: String = final_order.into_iter().collect();
	println!("{}", final_order_string);
}