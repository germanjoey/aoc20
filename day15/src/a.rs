use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::HashMap;
 
#[derive(Hash, Copy, Clone, Eq, PartialEq, PartialOrd)]
struct Coordinate {
    x: i32,
    y: i32
}

#[derive(Hash, Eq, PartialOrd)]
struct Warrior {
    id: usize,
    affiliation: char,
    location: Coordinate,
    hit_points: i32
}

struct Battlegrounds {
    cells: HashSet<Coordinate>
}

struct Battle {
    goblins: HashSet<Warrior>,
    elves: HashSet<Warrior>,
    battlegrounds: Battlegrounds
}

impl Coordinate {
    fn is_adjacent (&self, other: &Coordinate) -> bool {
        return self.distance(other) == 1;
    }

    fn distance(&self, other: &Coordinate) -> u32 {
        return ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32;
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        return self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

 
impl Warrior {
    fn new (id: usize, affiliation: char, location: Coordinate) -> Warrior {
        return Warrior { id: id, affiliation: affiliation, location: location, hit_points: 200 };
    }

    fn is_adjacent (&self, other_warrior: &Warrior) -> bool {
        return self.location.is_adjacent(&other_warrior.location);
    }

    fn move_to (&mut self, new_location: Coordinate) {
        self.location.x = new_location.x;
        self.location.y = new_location.y
    }

    fn attack (&self, opponent: &mut Warrior) -> bool {
        println!("  {} attacks {}!", self.id, opponent.id);
        opponent.hit_points -= 3;
        return opponent.hit_points <= 0;
    }
}
 
impl PartialEq for Warrior {
    fn eq(&self, other: &Warrior) -> bool {
        return self.location == other.location
    }
}

impl Ord for Warrior {
    fn cmp(&self, other: &Warrior) -> Ordering {
        return self.location.cmp(&other.location);
    }
}

impl Battlegrounds {
    fn new() -> Battlegrounds {
        return Battlegrounds { cells: HashSet::new() };
    }

    fn insert(&mut self, c: Coordinate) {
        self.cells.insert(c);
    }

    fn find_next_move(&self, source: Coordinate, candidate_targets: &HashSet<Coordinate>, blocked_cells: &mut HashSet<Coordinate>) -> Coordinate {
        let mut min_move: Coordinate = source;
        let mut min_distance: usize = 100000;

        for candidate_target in candidate_targets.iter() {
		    for (dx,dy) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
				let candidate_cell = Coordinate { x: candidate_target.x + dx, y: candidate_target.y + dy };
                if blocked_cells.contains(&candidate_cell) || !self.cells.contains(&candidate_cell) {
                    continue;
                }

                let (distance, next_move) = self.find_next_move_to_target(source, candidate_cell, blocked_cells);
                
                if distance == min_distance {
                    if (next_move.x == min_move.x) && (next_move.y < min_move.y) {
                        min_move = next_move;
                    }
                    else if next_move.x < min_move.x {
                        min_move = next_move;
                    }
                }
                else if distance < min_distance {
                    min_distance = distance;
                    min_move = next_move;
                }
            }
        }

        return min_move;
    }
    
    fn find_next_move_to_target(&self, source: Coordinate, target: Coordinate, blocked_cells: &HashSet<Coordinate>) -> (usize, Coordinate) {
        let mut queue: Vec<Coordinate> = vec![];
        let mut open_set: HashSet<Coordinate> = HashSet::new();
        let mut closed_set: HashSet<Coordinate> = HashSet::new();
        let mut path_info: HashMap<Coordinate, Coordinate> = HashMap::new();

        // initialize
	    path_info.insert(source, source);
		open_set.insert(source);
        queue.push(source);

        while queue.len() > 0 {
            let current_cell = queue.remove(0);
			open_set.remove(&current_cell);

            if current_cell == target {
                return self.find_first_cell_on_path(current_cell, path_info)
		    }

		    for (dx,dy) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
				let child_cell = Coordinate { x: current_cell.x + dx, y: current_cell.y + dy };
                if closed_set.contains(&child_cell) || blocked_cells.contains(&child_cell) || open_set.contains(&child_cell) || !self.cells.contains(&child_cell) {
					continue;
				}

				path_info.insert(child_cell, current_cell);
				open_set.insert(child_cell);
				queue.push(child_cell);
		    }
		
            closed_set.insert(current_cell);
	    }

        return (100000, source);
    }

    fn find_first_cell_on_path(&self, target: Coordinate, path_info: HashMap<Coordinate, Coordinate>) -> (usize, Coordinate) {
	    let mut current_cell: Coordinate = target;
        let mut count: usize = 0;

        loop {
		    let next_cell = path_info.get(&current_cell).unwrap();
		    let next_next_cell = path_info.get(&next_cell).unwrap();
		
		    if next_cell == next_next_cell {
			    return (count+1, current_cell);
		    }

            count += 1;
            current_cell = *next_cell;
	    }
    }
}
 
impl Battle {
    fn new (input_file: &str) -> Battle {
        let battlegrounds: Battlegrounds = Battlegrounds::new();
        let elves: HashSet<Warrior> = HashSet::new();
        let goblins: HashSet<Warrior> = HashSet::new();

        let mut obj = Battle { battlegrounds: battlegrounds, elves: elves, goblins: goblins };

        let f = File::open(input_file).unwrap();
        let file = BufReader::new(&f);
        
        for (i, line) in file.lines().enumerate() {
            obj.parse_line(&line.unwrap(), i as i32);
        }

        return obj;
    }
    
    fn parse_line(&mut self, line: &String, row_index: i32) {
        let line_bytes = line.as_bytes();

        for col_index in 0..line.len() {
            let coord = Coordinate { x: row_index, y: col_index  as i32 };
            let symbol: char = line_bytes[col_index] as char;

            if (symbol == '.') || (symbol == 'G') || (symbol == 'E') {
                self.battlegrounds.insert(coord);

                if symbol != '.' {
                    let id = 1 + self.elves.len() + self.goblins.len();
                    let warrior = Warrior::new(id, symbol, coord);

                    if symbol == 'E' {
                        self.elves.insert(warrior);
                    }
                    else {
                        self.goblins.insert(warrior);
                    }
                }
            }
        }
    }

    fn find_adjacent_opponents(&self, warrior: &mut Warrior, warrior_dict: &HashMap<usize, Warrior>) -> Vec<usize> {
        let mut opponent_ids: Vec<(usize, i32)> = vec![];
        
        for (other_warrior_id, other_warrior) in warrior_dict {
            if other_warrior.affiliation == warrior.affiliation {
                continue;
            }

            if warrior.is_adjacent(other_warrior) {
                opponent_ids.push((*other_warrior_id, other_warrior.hit_points));
            }
        }
        
        opponent_ids.sort_by(|a, b| a.1.cmp(&b.1));
        return opponent_ids.iter().map(|&t| t.0).collect();
    }

    fn move_warrior_to_fight(&self, warrior: &mut Warrior, warrior_dict: &mut HashMap<usize, Warrior>) {
        let mut warrior_locations: HashSet<Coordinate> = HashSet::new();
        let mut opponent_locations: HashSet<Coordinate> = HashSet::new();

        for (_other_warrior_id, other_warrior) in warrior_dict {
            warrior_locations.insert(other_warrior.location);
            if other_warrior.affiliation != warrior.affiliation {
                opponent_locations.insert(other_warrior.location);
            }
        }

        let next_cell = self.battlegrounds.find_next_move(warrior.location, &opponent_locations, &mut warrior_locations);
        println!("  moving {} from {},{} to {},{}", warrior.id, warrior.location.x, warrior.location.y, next_cell.x, next_cell.y);
        warrior.move_to(next_cell);
    }
 
    fn execute_warrior_turn (&self, warrior: &mut Warrior, warrior_dict: &mut HashMap<usize, Warrior>) -> usize {
        let mut opponent_ids: Vec<usize> = self.find_adjacent_opponents(warrior, warrior_dict);
        
        if opponent_ids.len() == 0 {
            self.move_warrior_to_fight(warrior, warrior_dict);
            opponent_ids = self.find_adjacent_opponents(warrior, warrior_dict);
        }

        if opponent_ids.len() > 0  {
            let chosen_opponent_id = opponent_ids[0];
            let mut opponent = warrior_dict.remove(&chosen_opponent_id).unwrap();
            let opponent_died: bool =  warrior.attack(&mut opponent);
            warrior_dict.insert(chosen_opponent_id, opponent);

            if opponent_died {
                return chosen_opponent_id;
            }
        }

        return 0;
    }

    fn gather_warriors(&mut self) -> (usize, usize, Vec<usize>, HashMap<usize, Warrior>) {
        let mut all_warriors: Vec<(usize, Coordinate)> = vec![];
        let mut warrior_dict: HashMap<usize, Warrior> = HashMap::new();

        let num_elves = self.elves.len();
        for warrior in self.elves.drain() {
            all_warriors.push((warrior.id, warrior.location));
            warrior_dict.insert(warrior.id, warrior);
        }
        
        let num_goblins = self.goblins.len();
        for warrior in self.goblins.drain() {
            all_warriors.push((warrior.id, warrior.location));
            warrior_dict.insert(warrior.id, warrior);
        }

        all_warriors.sort_by(|a, b| a.1.cmp(&b.1));

        let all_warriors_flat: Vec<usize> = all_warriors.iter().map(|&t| t.0).collect();
        return (num_elves, num_goblins, all_warriors_flat, warrior_dict);
    }

    fn battle_round (&mut self) -> (bool, bool) {
        let ret = self.gather_warriors();

        let mut num_elves = ret.0;
        let mut num_goblins = ret.1;
        let mut all_warriors: Vec<usize> = ret.2;
        let mut warrior_dict: HashMap<usize, Warrior> = ret.3;

        let mut round_complete: bool = true;
        for warrior_id in &all_warriors {
            if (num_elves == 0) || (num_goblins == 0) {
                round_complete = false;
                break;
            }

            if !warrior_dict.contains_key(&warrior_id) {
                continue;
            }
            
            let mut warrior = warrior_dict.remove(warrior_id).unwrap();
            let defeated_id = self.execute_warrior_turn(&mut warrior, &mut warrior_dict);
            warrior_dict.insert(warrior.id, warrior);

            if defeated_id != 0 {
                println!("  defeated: {}", defeated_id);
                let defeated_warrior = warrior_dict.remove(&defeated_id).unwrap();
                
                if defeated_warrior.affiliation == 'E' {
                    num_elves -= 1;
                }
                else {
                    num_goblins -= 1;
                }
            }
        }

        while all_warriors.len() > 0 {
            let warrior_id = all_warriors.pop().unwrap();
            if !warrior_dict.contains_key(&warrior_id) {
                continue;
            }

            let warrior = warrior_dict.remove(&warrior_id).unwrap();
            if warrior.affiliation == 'G' {
                self.goblins.insert(warrior);
            }
            else {
                self.elves.insert(warrior);
            }
        }
        
        println!("  elves status: {}/{}", self.elves.len(), self.elves.iter().map(|ref w| w.hit_points).sum::<i32>());
        for warrior in &self.elves {
            println!("    {} {}: {} {},{}", warrior.affiliation, warrior.id, warrior.hit_points, warrior.location.x, warrior.location.y);
        }
        
        println!("  goblins status: {}/{}", self.goblins.len(), self.goblins.iter().map(|ref w| w.hit_points).sum::<i32>());
        for warrior in &self.goblins {
            println!("    {} {}: {} {},{}", warrior.affiliation, warrior.id, warrior.hit_points, warrior.location.x, warrior.location.y);
        }
 
        return (round_complete, (self.elves.len() == 0) || (self.goblins.len() == 0));
    }
    
    fn simulate (&mut self) -> (usize, usize) {
        let mut rounds: usize = 0;
        println!("initial state");
        
        for warrior in &self.elves {
            println!("  {} {}: {} {},{}", warrior.affiliation, warrior.id, warrior.hit_points, warrior.location.x, warrior.location.y);
        }
        
        for warrior in &self.goblins {
            println!("  {} {}: {} {},{}", warrior.affiliation, warrior.id, warrior.hit_points, warrior.location.x, warrior.location.y);
        }
        
        loop {
            println!("");
            println!("round {}", rounds);
            let (round_complete, battle_complete) = self.battle_round();

            if round_complete {
                rounds += 1;
            }

            if battle_complete {
                break;
            }
        }
        
        let total_hp = self.elves.iter().map(|ref w| w.hit_points).sum::<i32>()
                     + self.goblins.iter().map(|ref w| w.hit_points).sum::<i32>();
        return (rounds, total_hp as usize);
    }
}

fn main() {
     let mut battle = Battle::new("input.txt");
     let (total_rounds, total_remaining_hit_points) = battle.simulate();

     println!("");
     println!("total_rounds: {}", total_rounds);
     println!("total remaining hit points on winning side: {}", total_remaining_hit_points);
     println!("final score: {}", total_rounds * total_remaining_hit_points);
}
