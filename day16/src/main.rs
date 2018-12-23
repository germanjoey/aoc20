use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

custom_derive! {
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumDisplay, EnumFromStr, IterVariants(Instructions))]
    #[allow(non_camel_case_types)]
    enum Instruction {
        addr, addi,
        mulr, muli,
        banr, bani,
        borr, bori,
        setr, seti,
        gtrr, gtri, gtir, 
        eqrr, eqri, eqir
    }
}

impl Instruction {
    fn exec(&self, registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
        use Instruction::*;
        registers[c] = match self {
            addr => registers[a] + registers[b],
            addi => registers[a] + b,
            mulr => registers[a] * registers[b],
            muli => registers[a] * b,
            banr => registers[a] & registers[b],
            bani => registers[a] & b,
            borr => registers[a] | registers[b],
            bori => registers[a] | b,
            setr => registers[a],
            seti => a,
            gtrr => (registers[a] > registers[b]) as usize,
            gtir => (a > registers[b]) as usize,
            gtri => (registers[a] > b) as usize,
            eqrr => (registers[a] == registers[b]) as usize,
            eqri => (a == registers[b]) as usize,
            eqir => (registers[a] == b) as usize
        };
    }

    fn simulate(&self, registers: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
        let mut result: Vec<usize> = registers.clone();
        self.exec(&mut result, a, b, c);
        return result;
    }
}

fn check_all_opcodes(instr_filter: &mut Vec<HashSet<Instruction>>, registers: &Vec<usize>, expected_output: &Vec<usize>, opcode: usize, a: usize, b: usize, c: usize) {
    for inst in Instruction::iter_variants() {
        let after_registers: Vec<usize> = inst.simulate(registers, a, b, c);

        for i in 0..registers.len() {
            if expected_output[i] != after_registers[i] {
                instr_filter[opcode].remove(&inst);
                break;
            }
        }
    }
}

fn parse_line (delimiter: &str, line: &str) -> Vec<usize> {
    let str_pieces: Vec<&str> = line.split(delimiter).collect::<Vec<&str>>();
    let pieces: Vec<usize> = str_pieces.iter().map( |&x| x.to_string().parse::<usize>().unwrap() ).collect::<Vec<usize>>();
    return pieces;
}

fn check_sample(instr_filter: &mut Vec<HashSet<Instruction>>, input_registers_line: &String, instr_params_line: &String, expected_output_line: &String) {
    let input_registers = parse_line(", ", &input_registers_line[9..input_registers_line.len()-1]);
    let instr_params = parse_line(" ", instr_params_line.as_str());
    let expected_output = parse_line(", ", &expected_output_line[9..expected_output_line.len()-1]);

    check_all_opcodes(instr_filter, &input_registers, &expected_output, instr_params[0], instr_params[1], instr_params[2], instr_params[3]);
}

fn run_command(instr_map: &HashMap<usize, Instruction>, registers: &mut Vec<usize>, instr_params_line: &String) {
    let instr_params = parse_line(" ", instr_params_line.as_str());
    instr_map[&instr_params[0]].exec(registers, instr_params[1], instr_params[2], instr_params[3]);
}

fn setup_instr_filter() -> Vec<HashSet<Instruction>> {
    let mut instr_filter: Vec<HashSet<Instruction>> = vec![];

    for _ in 0..16 {
        let mut per_opcode: HashSet<Instruction> = HashSet::new();
        for inst in Instruction::iter_variants() {
            per_opcode.insert(inst);
        }
        instr_filter.push(per_opcode);
    }

    return instr_filter;
}

fn main() {
    let mut instr_filter = setup_instr_filter();

    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    let mut lines_iter = file.lines();

    loop {
        let before_line = lines_iter.next().unwrap().unwrap();

        if before_line.len() < 3 {
            break;
        }

        let instr_params_line = lines_iter.next().unwrap().unwrap();
        let after_line = lines_iter.next().unwrap().unwrap();
        
        // blank line
        lines_iter.next();

        check_sample(&mut instr_filter, &before_line, &instr_params_line, &after_line);
    }

    let mut instr_map: HashMap<usize, Instruction> = HashMap::new();
    let mut assigned: HashSet<Instruction> = HashSet::new();

    while assigned.len() < 16 {
        for i in 0..16 {
            if instr_map.contains_key(&i) {
                continue;
            }

            if instr_filter[i].len() == 1 {
                for inst in instr_filter[i].drain() {
                    instr_map.insert(i, inst);
                    assigned.insert(inst);
                }
                break;
            }
        }
        
        for inst in &assigned {
            for i in 0..16 {
                instr_filter[i].remove(inst);
            }
        }
    }
    
    for i in 0..16 {
        println!("for opcode {}: {}", i, instr_map[&i]);
    }

    // blank!
    lines_iter.next();
    
    let mut registers: Vec<usize> = vec![0, 0, 0, 0];
    for line in lines_iter {
        run_command(&instr_map, &mut registers, &line.unwrap());
    }

    println!("\nvalue of register 0: {}", registers[0]);
}
