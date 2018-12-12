use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {   
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut total = 0;
    for line in file.lines() {
        let l = line.unwrap();
        //let i =  l.to_string().parse::<i32>().unwrap_or(0);
        //total += i;

        let r = l.to_string().parse::<i32>();
        let i: i32 = match r {
            Ok(n) => n,
            Err(_err) => 0
        };
        total += i;
    }

    println!("{}", total.to_string())
}
