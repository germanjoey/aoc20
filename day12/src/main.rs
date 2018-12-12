use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp;

fn get_patterns () -> Vec<String> {
    let mut plant_patterns: Vec<String> = vec![];
    
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);

    for line in file.lines() {
        let l = line.unwrap();
        let pieces = l.split(" => ").collect::<Vec<&str>>();
        let pattern = pieces[0].to_string();
        if pieces[1] == "#" {
            plant_patterns.push(pattern);
        }
    }

    return plant_patterns;
}

fn comp_pattern(state: &String, i: usize, pattern: &String) -> bool {
    if i >= 2 {
        return &state[i-2..i+3] == *pattern
    }

    let start = cmp::max(0, (i as i32) -2 ) as usize;
    let substring = format!("{:>5}", &state[start..i+3]);
    let dot_substring = substring.replace(" ", ".");
    return dot_substring == *pattern;
}

fn simulate (state: &String, plant_patterns: &Vec<String>) -> String {
    let mut new_state: Vec<char> = vec![];

    for i in 0..state.len()-2 {
        let mut found: bool = false;
        for pattern in plant_patterns {
            if comp_pattern(state, i, pattern) {
                found = true;
                new_state.push('#');
                break;
            }
        }
        if !found {
            new_state.push('.');
        }
    }

    new_state.push('.');
    new_state.push('.');
    new_state.push('.');

    return new_state.into_iter().collect();
}

fn sum_total (state: &String) -> i32 {
    let mut total: i32 = 0;
    for i in 0..state.len()-3 {
        if &state[i..i+1] == "#" {
            total += (i as i32) - 3;
        }
    }

    return total;
}

fn main() {
    let initial_str = "...##.######...#.##.#...#...##.####..###.#.##.#.##...##..#...##.#..##....##...........#.#.#..###.#...";
    let mut state: String = initial_str.to_string();

    let plant_patterns = get_patterns();

    for _i in 0..2000 {
        state = simulate(&state, &plant_patterns);
        let total: i32 = sum_total(&state);
        println!("{}: {}", _i, total);
    }

    let total: i32 = sum_total(&state);
    println!("");
    println!("total: {}", total);

    // pattern is a difference of 52 per iteration
    // thus, total will be: (50000000000 - 2000) * 52 + 104919
    // (where 104919 is the output from line 84)
}
