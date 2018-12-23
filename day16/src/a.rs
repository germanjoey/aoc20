use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

custom_derive! {
    #[derive(Debug, PartialEq, EnumDisplay, EnumFromStr, IterVariants(Instructions))]
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
    fn exec(&self, registers: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
        let mut result: Vec<usize> = registers.clone();

        use Instruction::*;
        result[c as usize] = match self {
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
            eqir => (registers[a] == b) as usize,
        };

        return result;
    }
}

fn check_all_opcodes(registers: &Vec<usize>, expected_output: &Vec<usize>, a: usize, b: usize, c: usize) -> usize {
    let mut count: usize = 0;

    for inst in Instruction::iter_variants() {
        let after_registers: Vec<usize> = inst.exec(registers, a, b, c);

        let mut regs_match = true;
        for i in 0..registers.len() {
            if expected_output[i] != after_registers[i] {
                regs_match = false;
            }
        }

        if regs_match {
            count += 1;
        }
    }

    return count;
}

fn parse_line (delimiter: &str, line: &str) -> Vec<usize> {
    let str_pieces: Vec<&str> = line.split(delimiter).collect::<Vec<&str>>();
    let pieces: Vec<usize> = str_pieces.iter().map( |&x| x.to_string().parse::<usize>().unwrap() ).collect::<Vec<usize>>();
    return pieces;
}

fn check_sample(input_registers_line: &String, instr_params_line: &String, expected_output_line: &String) -> bool {
    let input_registers = parse_line(", ", &input_registers_line[9..input_registers_line.len()-1]);
    let instr_params = parse_line(" ", instr_params_line.as_str());
    let expected_output = parse_line(", ", &expected_output_line[9..expected_output_line.len()-1]);

    let similiarity_count = check_all_opcodes(&input_registers, &expected_output, instr_params[1], instr_params[2], instr_params[3]);
    return similiarity_count >= 3;
}

fn main() {
    let mut vague_sample_count = 0;

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

        if check_sample(&before_line, &instr_params_line, &after_line) {
            vague_sample_count += 1;
        }
    }

    println!("Number of vague samples in test input: {}.", vague_sample_count);
}
