use crate::CPU;
use crate::parser::LabelMap;
use crate::{memory::Memory, types::Vars};

use super::errors;
use std::collections::HashMap;

fn get_dest_counter(lmap:  HashMap<String, usize>, label: &str) -> usize {
    match lmap.get(label) {
        Some(&c) => c,
        _ => panic!("Label {} not found", label),
    }
}

pub fn jmp(
    cpu: &mut CPU,
    cmd: Vec<&str>,
    label_map: HashMap<String, usize>
) {
    if cmd.len() != 2 {
        errors::args_error("", 1);
    }
    let label = cmd[1].trim();
    let dest_counter = get_dest_counter(label_map, label);
    cpu.program_counter = dest_counter as u32;
}

// pub fn je(
//     memory: &mut Memory,
//     vars: &mut Vars,
//     registers: CPU,
//     program: HashMap<String, Vec<String>>,
//     cmd: Vec<&str>,
//     b: &str,
//     l: i32,
// ) {
//     if cmd.len() < 2 {
//         errors::args_error(b, l)
//     }
//     let num = memory.get_int_from_stack(b, l);
//     if num == 0 {
//         let label = cmd[1].trim();
//         for _ in 0..4 {
//             memory.pop_stack()
//         }
//         execute_block(program, label, registers, memory, vars)
//     }
// }

// pub fn jne(
//     memory: &mut Memory,
//     vars: &mut Vars,
//     registers: CPU,
//     program: HashMap<String, Vec<String>>,
//     cmd: Vec<&str>,
//     b: &str,
//     l: i32,
// ) {
//     if cmd.len() < 2 {
//         errors::args_error(b, l)
//     }
//     let num = memory.get_int_from_stack(b, l);
//     if num != 0 {
//         let label = cmd[1].trim();
//         for _ in 0..4 {
//             memory.pop_stack()
//         }
//         execute_block(program, label, registers, memory, vars)
//     }
// }

// pub fn jgr(
//     memory: &mut Memory,
//     vars: &mut Vars,
//     registers: CPU,
//     program: HashMap<String, Vec<String>>,
//     cmd: Vec<&str>,
//     b: &str,
//     l: i32,
// ) {
//     if cmd.len() < 2 {
//         errors::args_error(b, l)
//     }
//     let num = memory.get_int_from_stack(b, l);
//     if num == 1 {
//         let label = cmd[1].trim();
//         for _ in 0..4 {
//             memory.pop_stack()
//         }
//         execute_block(program, label, registers, memory, vars)
//     }
// }

// pub fn jsm(
//     memory: &mut Memory,
//     vars: &mut Vars,
//     registers: CPU,
//     program: HashMap<String, Vec<String>>,
//     cmd: Vec<&str>,
//     b: &str,
//     l: i32,
// ) {
//     if cmd.len() < 2 {
//         errors::args_error(b, l)
//     }
//     let num = memory.get_int_from_stack(b, l);
//     if num == -1 {
//         let label = cmd[1].trim();
//         for _ in 0..4 {
//             memory.pop_stack()
//         }
//         execute_block(program, label, registers, memory, vars)
//     }
// }
