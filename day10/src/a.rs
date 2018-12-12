use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

struct Coordinate {
    x: i32,
    y: i32
}

struct Velocity {
    dx: i32,
    dy: i32
}

struct Extrema {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32
}

impl Extrema {
    fn new () -> Extrema {
        return Extrema { xmin: 999999999, xmax: -999999999, ymin: 999999999, ymax: -999999999 };
    }

    fn update(&mut self, c: &Coordinate) {
        if c.x < self.xmin {
            self.xmin = c.x;
        }
        if c.y < self.ymin {
            self.ymin = c.y;
        }
        if c.x > self.xmax {
            self.xmax = c.x;
        }
        if c.y > self.ymax {
            self.ymax = c.y;
        }
    }

    fn clear(&mut self) {
        self.xmin = 999999999;
        self.ymin = 999999999;
        self.xmax = -999999999;
        self.ymax = -999999999;
    }
}

struct LightBeam {
    c: Coordinate,
    v: Velocity
}

impl LightBeam {
    fn new (x: i32, y: i32, dx: i32, dy: i32) -> LightBeam {
        let c = Coordinate { x: x, y:y };
        let v = Velocity { dx: dx, dy: dy };
        return LightBeam { c: c, v: v };
    }

    fn move_one_step (&mut self) {
        self.c.x += self.v.dx;
        self.c.y += self.v.dy;
    }
}

struct LightDisplay {
    beams: Vec<LightBeam>,
    extremes: Extrema
}

impl LightDisplay {
    fn new() -> LightDisplay {
        let beams: Vec<LightBeam> = vec![];
        let extremes = Extrema::new();
        return LightDisplay { beams: beams, extremes: extremes };
    }

    fn add_beam (&mut self, beam: LightBeam) {
        self.extremes.update(&beam.c);
        self.beams.push(beam);
    }

    fn update_extremes (&mut self) {
        self.extremes.clear();
        for beam in &self.beams {
            self.extremes.update(&beam.c);
        }
    }

    fn simulate(&mut self) {
        for beam in self.beams.iter_mut() {
            beam.move_one_step();
        }
        self.update_extremes();
    }

    fn print_status(&self) {
        let dx = self.extremes.xmax - self.extremes.xmin;
        let dy = self.extremes.ymax - self.extremes.ymin;
        let total_e = dx.abs() + dy.abs();
        println!("{},{} / {},{} / {}", self.extremes.xmin, self.extremes.ymin,
                                       self.extremes.xmax, self.extremes.ymax,
                                       total_e);
    }
    
    fn print_display(&self) {
        self.extremes.xmin;
        self.extremes.ymin;

        let mut output_index: HashMap<i32, HashSet<i32>> = HashMap::new();
        for beam in &self.beams {
            let mut col = output_index.entry(beam.c.x).or_insert(HashSet::new());
            col.insert(beam.c.y);
        }
        
        for y in self.extremes.ymin..self.extremes.ymax+1 {
            for x in self.extremes.xmin..self.extremes.xmax+1 {
                if output_index.contains_key(&x) {
                    let col = output_index.get(&x).unwrap();
                    if col.contains(&y) {
                        print!("#");
                    }
                    else {
                        print!(" ");
                    }
                }
                else {
                    print!(" ");
                }
            }
            println!("");
        }

    }
}

fn convert_to_int(s: &str) -> i32 {
    let pieces = s.split(" ").collect::<Vec<&str>>();
    return pieces[pieces.len()-1].to_string().parse::<i32>().unwrap();
}

fn parse_data(line: &String) -> LightBeam {
    let x = convert_to_int(&line[10..16]);
    let y = convert_to_int(&line[18..24]);
    let dx = convert_to_int(&line[36..38]);
    let dy = convert_to_int(&line[40..42]);

    return LightBeam::new(x, y, dx, dy);
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut lights = LightDisplay::new();

    for line in file.lines() {
        let b = parse_data(&line.unwrap());
        lights.add_beam(b);
    }

    // manual binary search, using print_status ;)
    for _i in 0..10594 {
        lights.simulate();
    }
    
    lights.print_status();
    println!("");
    lights.print_display();
}
