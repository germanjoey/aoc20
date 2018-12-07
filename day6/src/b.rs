use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

struct Coordinate {
    x: i32,
    y: i32
}

struct MapExtrema {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32
}

struct Map {
	width: i32,
	height: i32
}

impl Map {
    fn new(extrema: MapExtrema) -> Map {
		let width: i32 = extrema.xmax - extrema.xmin + 1;
		let height: i32 = extrema.ymax - extrema.ymin + 1;

        return Map { width: width, height };
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
	let mut total_region_size = 0;

	for x in 0..map.width {
		for y in 0..map.height {
			let mut total_manhattan: i32 = 0;

			for owner_label in 0..label_coords.len() {
				let c = &label_coords[owner_label];
				let dx: i32 = x - c.x;
				let dy: i32 = y - c.y;
				
				let dist: i32 = dx.abs() + dy.abs();
				total_manhattan += dist;
			}

			if total_manhattan < 10000 {
				total_region_size += 1;
			}
		}
	}

	println!("total region size: {}", total_region_size);
}