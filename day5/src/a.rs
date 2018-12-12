use std::io::prelude::*;
use std::fs::File;

fn check_reaction(polymer: &mut String, index: usize) -> bool {
    let a = &polymer[index..(index+1)];
    let b = &polymer[(index+1)..(index+2)];

    let a_chr = a.chars().next().unwrap();
    let b_chr = b.chars().next().unwrap();

    let case_order1 = a_chr.is_uppercase() && b_chr.is_lowercase();
    let case_order2 = b_chr.is_uppercase() && a_chr.is_lowercase();

    return (case_order1 || case_order2) && (a.to_uppercase() == b.to_uppercase());
}

fn react_polymers(polymer: &mut String) {
    loop {
        let mut changed = false;
        for i in 1..polymer.len() {
            let index = polymer.len() - i - 1;

            if index >= polymer.len() {
                break;
            }

            if check_reaction(polymer, index) {
                changed = true;
                polymer.remove(index);
                polymer.remove(index);
                break;
            }

        }

        if changed == false {
            break;
        }
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut polymer = String::new();
    file.read_to_string(&mut polymer).expect("Unable to read the file");
    react_polymers(&mut polymer);
    println!("{}", polymer.len());
}
