use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use ram::execute_block;
use ram::memory::Memory;
use ram::parser;
use ram::types::Vars;
use ram::Registers;

fn main() {
    let mut filename: String = String::new();
    println!("Welcome to the Ram stack-based programming language.");
    println!("Please enter a filename: ");
    io::stdin()
        .read_line(&mut filename)
        .expect("Error reading filename");
    filename = filename.trim().to_string();

    let mut file: File = File::open(filename.trim()).expect("Error opening file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");

    let p_lines: Vec<&str> = contents.split("\n").collect();
    let instructions: HashMap<String, Vec<String>> = parser::parse_lines(p_lines);

    let registers: Registers = Registers::new();
    let mut memory: Memory = Memory::new();
    let mut vars: Vars = Vars::new();

    execute_block(instructions, "main:", registers, &mut memory, &mut vars);
}
