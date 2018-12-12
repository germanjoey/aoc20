use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

fn gen_id_permutation(s: &String, offset: usize) -> String {
    let a = &s[0..offset];
    let b = &s[offset+1..s.len()];
    return format!("{}{}", a.to_string(), b.to_string());
}

fn permute_check_and_add(s: &String, variations: &mut HashSet<String>) -> Option<String> {
    let mut found: Option<String> = None;

    // need to do two loops here: the first is for checking, the second for inserting
    // that's just in case there are double-letters in the string.
    for i in 0..s.len() {
        let permutation = gen_id_permutation(s, i);
        if variations.contains(&permutation) {
            found = Some(permutation.to_string())
        }
    }
    
    for i in 0..s.len() {
        let permutation = gen_id_permutation(s, i);
        variations.insert(permutation);
    }
    
    return found;
}

fn main() {
    let mut variations: HashSet<String> = HashSet::new();

    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);

    for (i, line) in file.lines().enumerate() {
        match permute_check_and_add(&line.unwrap(), &mut variations) {
            Some(s) => println!("found result: {} {}", i, s),
            None => (),
        };
    }
}
