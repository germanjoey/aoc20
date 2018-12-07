use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

struct Coordinate {
    x: i32,
    y: i32
}

struct Owner {
    label: i32,
    distance: i32
}

struct MapExtrema {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32
}

struct Map {
	width: i32,
	height: i32,
	grid: HashMap<i32, HashMap<i32, Owner>>,
	extrema: MapExtrema,
}

// USELESS IN THE END!!!! lol. WELL. At least I learned something about Rust!
impl Map {
    fn new(extrema: MapExtrema) -> Map {
		let width: i32 = extrema.xmax - extrema.xmin + 1;
		let height: i32 = extrema.ymax - extrema.ymin + 1;

		let mut grid: HashMap<i32, HashMap<i32, Owner>> = HashMap::new();
		for x in 0..width {
			let mut row: HashMap<i32, Owner> = HashMap::new();
			for y in 0..height {
				let unowned = Owner { label: -1, distance: width * 2 };
				row.insert(y as i32, unowned);
			}

			grid.insert(x as i32, row);
		}

        return Map { width: width, height: height, grid: grid, extrema: extrema };
    }

	fn set_coordinate_owner(&mut self, c: &Coordinate, label: i32, distance: i32) {
		let x: i32 = c.x - self.extrema.xmin;
		let y: i32 = c.y - self.extrema.ymin;
		let cell = self.grid.get_mut(&x).unwrap().get_mut(&y).unwrap();
		*cell = Owner { label: label, distance: distance };
	}
}

fn parse_coord(line: &String) -> Coordinate {
	let pieces = line.split(", ").collect::<Vec<&str>>();
	let x: i32 = pieces[0].to_string().parse().unwrap();
	let y: i32 = pieces[1].to_string().parse().unwrap();
	return Coordinate {x:x, y:y};
}

fn process_coords() -> (MapExtrema, Vec<Coordinate>) {
	let f = File::open("input.txt").unwrap();
	let file = BufReader::new(&f);

	let mut extrema = MapExtrema { xmin: 1000, xmax: 0, ymin: 1000, ymax:0 };
	let mut coords: Vec<Coordinate> = vec![];
	
	for line in file.lines() {
		let c = parse_coord(&line.unwrap());

		if c.x < extrema.xmin {
			extrema.xmin = c.x;
		}
		if c.y < extrema.ymin {
			extrema.ymin = c.y;
		}
		if c.x > extrema.xmax {
			extrema.xmax = c.x;
		}
		if c.y > extrema.ymax {
			extrema.ymax = c.y;
		}

		coords.push(c);
	}

	for i in 0..coords.len() {
		coords[i].x -= extrema.xmin;
		coords[i].y -= extrema.ymin;
	}

	return (extrema, coords);
}


fn main() {
	let (extrema, label_coords) = process_coords();
	let map: Map = Map::new(extrema);
	
	let mut non_infinites: HashSet<i32> = HashSet::new();
	let mut total_area: HashMap<i32, i32> = HashMap::new();
	for owner_label in 0..label_coords.len() {
		let owner_label_i32 = owner_label as i32;
		total_area.entry(owner_label_i32).or_insert(0);
		non_infinites.insert(owner_label_i32);
	}

	for x in 0..map.width {
		for y in 0..map.height {
			let mut owning_label: i32 = -1;
			let mut tie = false;
			let mut min_manhattan: i32 = 2 * (map.width + map.height);

			for owner_label in 0..label_coords.len() {
				let c = &label_coords[owner_label];
				let dx: i32 = x - c.x;
				let dy: i32 = y - c.y;
				
				let dist: i32 = dx.abs() + dy.abs();
				
				if dist == min_manhattan {
					tie = true;
					owning_label = -1;
				}
				else if dist < min_manhattan {
					tie = false;
					min_manhattan = dist;
					owning_label = owner_label as i32;
				}
			}

			if tie == false {
				let count = total_area.get_mut(&owning_label).unwrap();
				*count += 1;

				if (x == 0) || (x == map.width - 1) || (y == 0) || (y == map.height - 1) {
					non_infinites.remove(&owning_label);
				}
			}
		}
	}

	for label in &non_infinites {
		println!("{} {}", label, total_area[label]);
	}
}