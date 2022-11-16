use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::{env, io};

use ram::execute_block;
use ram::memory::Memory;
use ram::parser::{self, LabelMap};
use ram::types::Vars;
use ram::CPU;

fn main() {
    let mut filename: String = String::new();
    if env::args().nth(1).is_none() {
        println!("Welcome to the Ram stack-based programming language.");
        println!("Please enter a filename: ");
        io::stdin()
            .read_line(&mut filename)
            .expect("Error reading filename");
        filename = filename.trim().to_string();
    } else {
        filename = env::args().nth(1).unwrap()
    }
    let mut file: File = File::open(filename.trim()).expect("Error opening file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");

    let p_lines: Vec<&str> = contents.split("\n").collect();
    let program: LabelMap = parser::parse_lines(p_lines);
    let instructions = program.instructions;
    let label_map = program.map;

    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    let mut vars = Vars::new();

    cpu.execute(instructions, label_map, &mut memory, &mut vars);
}
