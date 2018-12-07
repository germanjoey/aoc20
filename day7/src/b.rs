use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;

fn calc_time_from_step(step: char) -> u8 {
	let time: u8 = step as u8;
	return time - 4;
}

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

struct UnitOfWork {
    step: char,
    time_to_completion: u8
}

fn main() {
	let mut dependencies = parse_input();
	
	let mut final_order: Vec<char> = vec![];
	let mut steps_ready: Vec<char> = vec![];
	let mut done: HashSet<char> = HashSet::new();

	let mut workers_ready: Vec<i32> = vec![0, 1, 2, 3, 4];
	let mut workers_working: HashMap<i32, UnitOfWork> = HashMap::new();
	let mut total_time: i32 = 0;

	loop {
		for (dependent, dependency_set) in dependencies.iter_mut() {
			for letter in &done {
				dependency_set.remove(&letter);
			}

			if dependency_set.len() == 0 {
				steps_ready.push(*dependent);
			}
		}

		for step in &steps_ready {
			dependencies.remove(&step);
		}

		steps_ready.sort_by(|a, b| b.cmp(a));

		// first get the ready workers started
		while (steps_ready.len() > 0) && (workers_ready.len() > 0) {
			let next_step: char = steps_ready.pop().unwrap();
			let next_worker: i32 = workers_ready.pop().unwrap();
			let time: u8 = calc_time_from_step(next_step);
			let work = UnitOfWork { step: next_step, time_to_completion: time };

			workers_working.entry(next_worker).or_insert(work);
		}
		
		let mut finished: Vec<i32> = vec![];
		while finished.len() == 0 {
			total_time += 1;

			for (worker, work) in workers_working.iter_mut() {
				work.time_to_completion -= 1;

				if work.time_to_completion == 0 {
					finished.push(*worker);
				}
			}
		}

		for worker in finished {
			let (_worker, work) = workers_working.remove_entry(&worker).unwrap();
			let finished_step = work.step;
			final_order.push(finished_step);
			done.insert(finished_step);
			workers_ready.push(worker);
		}

		if (dependencies.len() == 0) && (steps_ready.len() == 0) && (workers_ready.len() == 5) {
			break;
		}
	}

	println!("total time: {}", total_time);
}