use crate::memory::Memory;
use crate::CPU;

use super::errors::ErrorKind;
use std::collections::HashMap;

fn get_dest_counter(lmap: HashMap<String, usize>, label: &str) -> Result<usize, ErrorKind> {
    match lmap.get(label) {
        Some(&c) => Ok(c),
        _ => return Err(ErrorKind::LabelNotFound(label.to_string())),
    }
}

pub fn jmp(
    cpu: &mut CPU,
    cmd: Vec<&str>,
    label_map: HashMap<String, usize>,
) -> Result<(), ErrorKind> {
    if cmd.len() != 2 {
        return Err(ErrorKind::ArgErr);
    }
    let label = cmd[1].trim();
    let dest_counter = get_dest_counter(label_map, label)?;
    cpu.callstack.push(cpu.program_counter + 1);
    cpu.program_counter = dest_counter as u32;
    cpu.jmp = true;
    Ok(())
}

pub fn je(
    cpu: &mut CPU,
    cmd: Vec<&str>,
    label_map: HashMap<String, usize>,
    memory: &mut Memory,
) -> Result<(), ErrorKind> {
    if cmd.len() != 2 {
        return Err(ErrorKind::ArgErr);
    }
    if memory.get_int_from_stack()? == 0 {
        let label = cmd[1].trim();
        let dest_counter = get_dest_counter(label_map, label)?;
        cpu.callstack.push(cpu.program_counter + 1);
        cpu.program_counter = dest_counter as u32;
        cpu.jmp = true;
    }
    let sub = memory.stack.len().saturating_sub(4);
    memory.stack.truncate(sub);
    Ok(())
}

pub fn jne(
    cpu: &mut CPU,
    cmd: Vec<&str>,
    label_map: HashMap<String, usize>,
    memory: &mut Memory,
) -> Result<(), ErrorKind> {
    if cmd.len() != 2 {
        return Err(ErrorKind::ArgErr);
    }
    if memory.get_int_from_stack()? != 0 {
        let label = cmd[1].trim();
        let dest_counter = get_dest_counter(label_map, label)?;
        cpu.callstack.push(cpu.program_counter + 1);
        cpu.program_counter = dest_counter as u32;
        cpu.jmp = true;
    }
    let sub = memory.stack.len().saturating_sub(4);
    memory.stack.truncate(sub);
    Ok(())
}

pub fn jgr(
    cpu: &mut CPU,
    cmd: Vec<&str>,
    label_map: HashMap<String, usize>,
    memory: &mut Memory,
) -> Result<(), ErrorKind> {
    if cmd.len() != 2 {
        return Err(ErrorKind::ArgErr);
    }
    if memory.get_int_from_stack()? == 1 {
        let label = cmd[1].trim();
        let dest_counter = get_dest_counter(label_map, label)?;
        cpu.callstack.push(cpu.program_counter + 1);
        cpu.program_counter = dest_counter as u32;
        cpu.jmp = true;
    }
    let sub = memory.stack.len().saturating_sub(4);
    memory.stack.truncate(sub);
    Ok(())
}

pub fn jsm(
    cpu: &mut CPU,
    cmd: Vec<&str>,
    label_map: HashMap<String, usize>,
    memory: &mut Memory,
) -> Result<(), ErrorKind> {
    if cmd.len() != 2 {
        return Err(ErrorKind::ArgErr);
    }
    if memory.get_int_from_stack()? == -1 {
        let label = cmd[1].trim();
        let dest_counter = get_dest_counter(label_map, label)?;
        cpu.callstack.push(cpu.program_counter + 1);
        cpu.program_counter = dest_counter as u32;
        cpu.jmp = true;
    }
    let sub = memory.stack.len().saturating_sub(4);
    memory.stack.truncate(sub);
    Ok(())
}

pub fn ret(cpu: &mut CPU) -> Result<(), ErrorKind> {
    if cpu.callstack.len() < 1 {
        return Err(ErrorKind::EmptyCallstack);
    }
    cpu.jmp = true;
    cpu.program_counter = cpu.callstack[cpu.callstack.len() - 1];
    cpu.callstack.pop();
    Ok(())
}
