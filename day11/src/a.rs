use std::cmp;
 
struct Coordinate {
    x: u32,
    y: u32
}
 
fn get_square(start_x: u32, start_y: u32, size: u32) -> Vec<Coordinate> {
    let mut v: Vec<Coordinate> = vec![];
 
    let max_x: u32 = cmp::min(301, start_x+size);
    let max_y: u32 = cmp::min(301, start_y+size);
 
    for x in start_x..max_x {
        for y in start_y..max_y {
            v.push(Coordinate { x:x, y:y });
        }
    }
 
    return v;
}
 
fn calculate_power(c: &Coordinate, grid_serial_number: u32) -> i32 {
    let rack_id = c.x + 10;
    let mut power_level = rack_id * c.y;
    power_level += grid_serial_number;
    power_level *= rack_id;
 
    let power_level_str: String = power_level.to_string();
   
    let l = power_level_str.len();
    let mut hundreds_place: u8 = 0;
    if l >= 3 {
        hundreds_place = power_level_str.as_bytes()[l - 3] - 48;
    }
   
    let final_power: i32 = (hundreds_place as i32) - 5;
    return final_power;
}
 
fn find_max_power (max_x: u32, max_y: u32, square_size: u32, grid_serial_number: u32) -> (i32, Coordinate) {
    let mut max_power: i32 = -1000;
    let mut max_power_location = Coordinate { x:0, y:0 };
    for x in 1..max_x {
        for y in 1..max_y {
 
            let mut power: i32 = 0;
            let points = get_square(x, y);
            for point in &points {
                power += calculate_power(&point, grid_serial_number)
            }
 
            if power > max_power {
                max_power = power;
                max_power_location = Coordinate { x:x, y:y };
            }
        }
    }
 
    return (max_power, max_power_location);
}
 
fn main() {
    let mut max_power_overall: i32 = -1000;
    let mut max_power_size = 0;
    let mut max_power_location = Coordinate { x:0, y:0 };
 
    for size in 3..300 {
        let mut max_power: i32;
        let mut max_power_coordinate: Coordinate;
 
        (max_power, max_power_location) = find_max_power(300 as u32, 300 as u32, 9110 as u32);
        if max_power > max_power_overall {
            max_power = max_power_overall;
            max_power_size = max_power_size;
            max_power_location = max_power_location;
        }
    }
 
    println!("max power is: {},{},{}", max_power_location.x, max_power_location.y, max_power_size);
}