use std::io::prelude::*;
use std::fs::File;

fn char_at_index(treepieces: &Vec<&str>, index: u32) -> u32 {
	let piece = treepieces[index as usize].to_string();
	return piece.parse().unwrap();
}

fn process_tree(treepieces: &Vec<&str>, starting_index: u32) -> (u32, u32) {
	let mut node_value_total: u32 = 0;
	let mut index: u32 = starting_index;
	let mut child_values: Vec<u32> = vec![];

	let child_node_count: u32 = char_at_index(treepieces, index);
	let metadata_count: u32 = char_at_index(treepieces, index + 1);
	index += 2;
	
	for _c in 0..child_node_count {
		let (new_index, child_value) = process_tree(treepieces, index);
		index = new_index;
		child_values.push(child_value);
	}
	
	let mut metadata_sum: u32 = 0;
	for _m in 0..metadata_count {
		let metadata_value: u32 = char_at_index(treepieces, index);
		metadata_sum += metadata_value;

		if metadata_value <= child_node_count {
			let child_index = (metadata_value - 1) as usize;
			node_value_total += child_values[child_index];
		}

		index += 1;
	}

	if child_node_count == 0 {
		node_value_total = metadata_sum;
	}
	
	return (index, node_value_total);
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut treestring = String::new();
    file.read_to_string(&mut treestring).expect("Unable to read the file");
	let pieces = treestring.split(" ").collect::<Vec<&str>>();
	let (_index, metadata_count) = process_tree(&pieces, 0);
    println!("{}", metadata_count);
}