use crate::CPU;
use crate::{memory::Memory};

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
    cpu.jmp = true;
}

pub fn je(
    cpu: &mut CPU,
    cmd: Vec<&str>,
    label_map: HashMap<String, usize>,
    memory: &mut Memory
) {
    if cmd.len() != 2 {
        errors::args_error("", 1);
    }
    if memory.get_int_from_stack("", 1) == 0 {
        let label = cmd[1].trim();
        let dest_counter = get_dest_counter(label_map, label);
        cpu.program_counter = dest_counter as u32;
        cpu.jmp = true;
    }
}

pub fn jne(
    cpu: &mut CPU,
    cmd: Vec<&str>,
    label_map: HashMap<String, usize>,
    memory: &mut Memory
) {
    if cmd.len() != 2 {
        errors::args_error("", 1);
    }
    if memory.get_int_from_stack("", 1) != 0 {
        let label = cmd[1].trim();
        let dest_counter = get_dest_counter(label_map, label);
        cpu.program_counter = dest_counter as u32;
        cpu.jmp = true;
    }
}

pub fn jgr(
    cpu: &mut CPU,
    cmd: Vec<&str>,
    label_map: HashMap<String, usize>,
    memory: &mut Memory
) {
    if cmd.len() != 2 {
        errors::args_error("", 1);
    }
    if memory.get_int_from_stack("", 1) == 1 {
        let label = cmd[1].trim();
        let dest_counter = get_dest_counter(label_map, label);
        cpu.program_counter = dest_counter as u32;
        cpu.jmp = true;
    }
}

pub fn jsm(
    cpu: &mut CPU,
    cmd: Vec<&str>,
    label_map: HashMap<String, usize>,
    memory: &mut Memory
) {
    if cmd.len() != 2 {
        errors::args_error("", 1);
    }
    if memory.get_int_from_stack("", 1) == -1 {
        let label = cmd[1].trim();
        let dest_counter = get_dest_counter(label_map, label);
        cpu.program_counter = dest_counter as u32;
        cpu.jmp = true;
    }
}
