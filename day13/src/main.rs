use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
 
#[derive(Hash, Copy, Clone, Eq, PartialEq, PartialOrd)]
struct Coordinate {
    x: i32,
    y: i32
}

enum MineCartDirection {
    None,
    Up,
    Down,
    Left,
    Right,
}

enum MineCartIntersectionState {
    Left,
    Straight,
    Right
}

struct MineCart {
    id: u32,
    location: Coordinate,
    direction: MineCartDirection,
    next_intersection: MineCartIntersectionState
}

struct MineMap {
    carts: Vec<MineCart>,
    cells: HashMap<Coordinate, Cell>
}

struct Cell {
    symbol: char
}

impl Cell {
    fn new(symbol: char) -> Cell {
        return Cell { symbol: match symbol {
            '<' => '-',
            '>' => '-',
            '^' => '|',
            'v' => '|',
            _ => symbol
        }};
    }
}

impl fmt::Display for MineCartDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           MineCartDirection::Up => write!(f, "Up"),
           MineCartDirection::Down => write!(f, "Down"),
           MineCartDirection::Left => write!(f, "Left"),
           MineCartDirection::Right => write!(f, "Right"),
           MineCartDirection::None => write!(f, "None")
       }
    }
}
 
impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        return self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}
 
impl MineCart {
    fn new (id: u32, location: Coordinate, direction: MineCartDirection) -> MineCart {
        return MineCart { id: id, location: location, direction: direction, next_intersection: MineCartIntersectionState::Left };
    }
 
    fn move_forward (&mut self) {
        match self.direction {
            MineCartDirection::Up => { self.location.y -= 1 },
            MineCartDirection::Down => { self.location.y += 1 },
            MineCartDirection::Left => { self.location.x -= 1 },
            MineCartDirection::Right => { self.location.x += 1 },
            MineCartDirection::None => ()
        };
    }

    fn find_next_direction (&mut self, next_cell: &Cell) {
        if (next_cell.symbol == '|') || (next_cell.symbol == '-') {
            return;
        }

        if next_cell.symbol == '+' {
            self.direction = match self.direction {
                MineCartDirection::Up => match self.next_intersection {
                    MineCartIntersectionState::Left => MineCartDirection::Left,
                    MineCartIntersectionState::Straight => MineCartDirection::Up,
                    MineCartIntersectionState::Right => MineCartDirection::Right
                 },
                MineCartDirection::Down => match self.next_intersection {
                    MineCartIntersectionState::Left => MineCartDirection::Right,
                    MineCartIntersectionState::Straight => MineCartDirection::Down,
                    MineCartIntersectionState::Right => MineCartDirection::Left
                },
                MineCartDirection::Left => match self.next_intersection {
                    MineCartIntersectionState::Left => MineCartDirection::Down,
                    MineCartIntersectionState::Straight => MineCartDirection::Left,
                    MineCartIntersectionState::Right => MineCartDirection::Up
                },
                MineCartDirection::Right =>  match self.next_intersection {
                    MineCartIntersectionState::Left => MineCartDirection::Up,
                    MineCartIntersectionState::Straight => MineCartDirection::Right,
                    MineCartIntersectionState::Right => MineCartDirection::Down
                },
                MineCartDirection::None => MineCartDirection::None
            };

            self.next_intersection = match self.next_intersection {
                MineCartIntersectionState::Left => MineCartIntersectionState::Straight,
                MineCartIntersectionState::Straight => MineCartIntersectionState::Right,
                MineCartIntersectionState::Right => MineCartIntersectionState::Left
            };
        }
        else {
            self.direction = match self.direction {
                MineCartDirection::Up => {
                    match next_cell.symbol {
                        '/' => MineCartDirection::Right,
                        '\\' => MineCartDirection::Left,
                        _ => MineCartDirection::None
                    }
                },
                MineCartDirection::Down => {
                    match next_cell.symbol {
                        '/' => MineCartDirection::Left,
                        '\\' => MineCartDirection::Right,
                        _ => MineCartDirection::None
                    }
                },
                MineCartDirection::Left => {
                    match next_cell.symbol {
                        '/' => MineCartDirection::Down,
                        '\\' => MineCartDirection::Up,
                        _ => MineCartDirection::None
                    }
                },
                MineCartDirection::Right => {
                    match next_cell.symbol {
                        '/' => MineCartDirection::Up,
                        '\\' => MineCartDirection::Down,
                        _ => MineCartDirection::None
                    }
                },
                MineCartDirection::None => MineCartDirection::None
            };
        }
    }
 
    fn check_for_collision(&self, other_cart: &MineCart) -> bool {
        if other_cart.id == self.id {
            return false;
        }
 
        return self == other_cart
    }
}
 
impl Eq for MineCart {}
 
impl PartialEq for MineCart {
    fn eq(&self, other: &MineCart) -> bool {
        return self.location == other.location
    }
}

impl PartialOrd for MineCart {
    fn partial_cmp(&self, other: &MineCart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
 
impl Ord for MineCart {
    fn cmp(&self, other: &MineCart) -> Ordering {
        return self.location.cmp(&other.location);
    }
}
 
impl MineMap {
    fn new () -> MineMap {
        let cells: HashMap<Coordinate, Cell> = HashMap::new();
        let carts: Vec<MineCart> = vec![];

        let mut obj = MineMap { cells: cells, carts: carts};

        let f = File::open("input.txt").unwrap();
        let file = BufReader::new(&f);
        
        for (i, line) in file.lines().enumerate() {
            obj.parse_line(&line.unwrap(), i as i32);
        }

        return obj;
    }
    
    fn parse_line(&mut self, line: &String, row_index: i32) {
        let line_bytes = line.as_bytes();

        for col_index in 0..line.len() {
            let coord = Coordinate {x: col_index as i32, y: row_index};
            
            let mut symbol: char = line_bytes[col_index] as char;

            let direction = match symbol {
                '<' => MineCartDirection::Left,
                '>' =>  MineCartDirection::Right,
                '^' => MineCartDirection::Up,
                'v' => MineCartDirection::Down,
                _ => MineCartDirection::None
            };

            match direction {
                MineCartDirection::None => (),
                _ => {
                    let mut cart = MineCart::new(self.carts.len() as u32, coord, direction);
                    self.carts.push(cart);
                }
            }

            self.cells.insert(coord, Cell::new(symbol));
        }
    }
 
    fn check_cart_for_collisions (&self, cart: &MineCart) -> i32 {
        for oc in 0..self.carts.len() {
            let other_cart = &self.carts[oc];
            if cart.check_for_collision(&other_cart) {
                return oc as i32;
            }
        }
 
        return -1;
    }
 
    fn advance_carts (&mut self) -> bool {
        self.carts.sort();
        let mut to_remove: Vec<usize> = vec![];

        for c in 0..self.carts.len() {
            let mut collision_found = false;
            for r in &to_remove {
                if *r == c {
                    collision_found = true;
                    break;
                }
            }

            if collision_found {
                continue;
            }

            self.carts[c].move_forward();
            let next_cell = self.cells.get(&self.carts[c].location).unwrap();
            self.carts[c].find_next_direction(next_cell);

            let collision = self.check_cart_for_collisions(&self.carts[c]);
            if collision != -1 {
                to_remove.push(c);
                to_remove.push(collision as usize);
            }
        }

        to_remove.sort();
        to_remove.reverse();

        for index in to_remove {
            self.carts.remove(index);
        }
 
        return self.carts.len() > 1;
    }
    
    fn simulate (&mut self) -> Coordinate {
        while self.advance_carts() {}
        return self.carts[0].location;
    }
}
 

fn main() {
     let mut map = MineMap::new();
     let last_remaining = map.simulate();
     println!("Last remaining cart at: {},{}", last_remaining.x, last_remaining.y);
}
