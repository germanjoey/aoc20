use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn parse_guard_id(line: &String) -> (String, i32) {
    // [1518-07-16 00:00] Guard #3209 begins shift
    let pieces = line.split(" ").collect::<Vec<&str>>();

    let date = pieces[0].to_string();
    let id_raw = pieces[3].to_string();
    let id = id_raw[1..id_raw.len()].to_string().parse::<i32>().unwrap();
    println!("{} {}", id_raw, id);

    return (date, id);
}

fn parse_time(line: &String) -> i32 {
    // [1518-07-16 00:00] Guard #3209 begins shift
    let pieces = line.split(" ").collect::<Vec<&str>>();

    let time: i32 = pieces[1][3..5].to_string().parse::<i32>().unwrap();
    return time;
}

fn process_log(lines: &mut Vec<String>) -> HashMap<i32, HashMap<i32, i32>> {
    let mut records: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    while lines.len() > 0 {
        let id_line = lines.pop().unwrap();
        let (_date, id) = parse_guard_id(&id_line);
        let record = records.entry(id).or_insert(HashMap::new());

        loop {
            if lines.len() < 1 {
                break;
            }

            if lines[lines.len()-1].to_string().contains("Guard") {
                break;
            }

            let falls_asleep_time: i32 = parse_time(&lines.pop().unwrap());
            let wakes_up_time: i32 = parse_time(&lines.pop().unwrap());

            for t in falls_asleep_time..wakes_up_time {
                let count = record.entry(t).or_insert(0);
                *count += 1;
            }
        }
    }

    return records;
}

fn calc_total_minutes(record: &HashMap<i32, i32>) -> (i32, i32) {
    let mut total = 0;
    let mut max = 0;
    let mut max_minute = 0;
    for (minute, count) in record {
        total += *count;
        if *count > max {
            max_minute = *minute;
            max = *count;
        }
    }

    return (total, max_minute);
}


fn main() {
    let mut lines: Vec<String> = vec![];

    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);

    for line in file.lines() {
        lines.push(line.unwrap());
    }

    lines.sort_unstable_by(|a, b| b.cmp(a));
    let records: HashMap<i32, HashMap<i32, i32>> = process_log(&mut lines);

    let mut max_minutes = 0;
    let mut max_minutes_out = -1;
    for (id, record) in &records {
        let (total_minutes_asleep, max_minute) = calc_total_minutes(&record);
        if total_minutes_asleep > max_minutes {
            max_minutes = total_minutes_asleep;
            max_minutes_out = id * max_minute;
        }
    }

    println!("Output is: {}", max_minutes_out);
}