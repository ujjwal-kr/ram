use super::errors::ErrorKind;
use crate::types::Vector;
use crate::types::{TypeName, Vars};
use crate::{memory::Memory, CPU};

pub fn vec(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
    statement: &str,
) -> Result<(), ErrorKind> {
    if cmd.len() < 3 {
        return Err(ErrorKind::ArgErr);
    }
    match cmd[2] {
        "push" => {
            let var = vars.get_type(cmd[1].to_string()).unwrap();
            let value = cmd[3];
            if var.name == TypeName::Vector(Vector::Int) {
                match value {
                    "lx" => {
                        let vec_mod = vars.get_vec_int_mod(var, registers.lx, memory);
                        memory.vec_int_push(&vec_mod.heap_addr, vec_mod.value_bytes);
                    }
                    "rv" => {
                        let vec_mod = vars.get_vec_int_mod(var, registers.rv, memory);
                        memory.vec_int_push(&vec_mod.heap_addr, vec_mod.value_bytes);
                    }
                    _ => {
                        let var2 = vars.get_type(value.to_string())?;
                        if var2.name == TypeName::I32 {
                            let var2_int = memory.yeild_i32(var2.location);
                            let vec_mod = vars.get_vec_int_mod(var, var2_int, memory);
                            memory.vec_int_push(&vec_mod.heap_addr, vec_mod.value_bytes);
                        } else {
                            return Err(ErrorKind::ExpectedInt(value.to_string()));
                        }
                    }
                }
            } else if var.name == TypeName::Vector(Vector::String) {
                match value {
                    "string" => {
                        let vec_mod = vars.get_vec_str_mod(var, &registers.string, memory);
                        memory.vec_str_push(&vec_mod.heap_addr, &vec_mod.value_bytes);
                    }
                    "lxstring" => {
                        let vec_mod = vars.get_vec_str_mod(var, &registers.lxstring, memory);
                        memory.vec_str_push(&vec_mod.heap_addr, &vec_mod.value_bytes);
                    }
                    _ => {
                        let var2 = vars.get_type(value.to_string())?;
                        if var2.name == TypeName::String {
                            let str_value = memory.yeild_string(var2.location);
                            let vec_mod = vars.get_vec_str_mod(var, &str_value, memory);
                            memory.vec_str_push(&vec_mod.heap_addr, &vec_mod.value_bytes);
                        } else {
                            return Err(ErrorKind::ExpectedStr(value.to_string()));
                        }
                    }
                }
            } else {
                return Err(ErrorKind::ExpectedVec(cmd[1].to_string()));
            }
        }
        "len" => {
            let var = vars.get_type(cmd[1].to_string())?;
            if var.name == TypeName::Vector(Vector::Int) {
                let len = vars.vec_int_len(var.location, memory);
                memory.set_int_to_stack(len);
            } else if var.name == TypeName::Vector(Vector::String) {
                let len = vars.vec_str_len(var.location, memory);
                memory.set_int_to_stack(len);
            } else {
                return Err(ErrorKind::ExpectedVec(cmd[1].to_string()));
            }
        }
        "shift" => {
            let var = vars.get_type(cmd[1].to_string())?;
            let heap_addr = memory.load(var.location).to_owned();
            match var.name {
                TypeName::Vector(Vector::String) => memory.shift_vec_str(&heap_addr),
                TypeName::Vector(Vector::Int) => memory.shift_vec_int(&heap_addr),
                _ => return Err(ErrorKind::ExpectedVec(cmd[1].to_string())),
            }
        }
        "pop" => {
            let var = vars.get_type(cmd[1].to_string())?;
            let heap_addr = memory.load(var.location).to_owned();
            match var.name {
                TypeName::Vector(Vector::String) => memory.pop_vec_str(&heap_addr),
                TypeName::Vector(Vector::Int) => memory.pop_vec_int(&heap_addr),
                _ => return Err(ErrorKind::ExpectedVec(cmd[1].to_string())),
            }
        }
        _ => {
            let s = cmd.join("");
            let chars = s.split("").collect::<Vec<&str>>();
            if !chars.contains(&"[") || !chars.contains(&"]") || !chars.contains(&"=") {
                return Err(ErrorKind::ArgErr);
            } // performance overhead

            let bracket_i = chars.iter().position(|&r| r == "[").unwrap();
            let equal_i = chars.iter().position(|&r| r == "=").unwrap();

            if bracket_i > equal_i {
                let to_assign = cmd[1];
                let var_exp = statement.split('=').collect::<Vec<&str>>()[1].trim();
                let var_str = var_exp.split('[').collect::<Vec<&str>>()[0].trim();
                let idx_str = var_exp.split('[').collect::<Vec<&str>>()[1]
                    .trim()
                    .split(']')
                    .collect::<Vec<&str>>()[0]
                    .trim();
                let index: usize;
                match idx_str {
                    "lx" => index = registers.lx as usize,
                    "rv" => index = registers.rv as usize,
                    _ => {
                        let num = idx_str.parse::<i32>();
                        match num {
                            Ok(n) => index = n as usize,
                            _parse_int_error => {
                                let idx_var = vars.get_type(idx_str.to_string())?;
                                if idx_var.name == TypeName::I32 {
                                    index = memory.yeild_i32(idx_var.location) as usize;
                                } else {
                                    return Err(ErrorKind::ExpectedInt(idx_str.to_string()));
                                }
                            }
                        }
                    }
                }
                let var = vars.get_type(var_str.to_string())?;
                if var.name == TypeName::Vector(Vector::Int) {
                    match to_assign {
                        "lx" => {
                            registers.lx =
                                vars.vec_int_item(var.location, var_str.to_string(), index, memory)?
                        }
                        "rv" => {
                            registers.rv =
                                vars.vec_int_item(var.location, var_str.to_string(), index, memory)?
                        }
                        _ => {
                            let assign_var = vars.get_type(to_assign.to_string())?;
                            if assign_var.name == TypeName::I32 {
                                let data: &[u8] = &vars
                                    .vec_int_item(var.location, var_str.to_string(), index, memory)?
                                    .to_be_bytes();
                                memory.stack_mod(assign_var.location, data);
                            } else {
                                return Err(ErrorKind::ExpectedInt(to_assign.to_string()));
                            }
                        }
                    }
                } else if var.name == TypeName::Vector(Vector::String) {
                    match to_assign {
                        "string" => {
                            registers.string =
                                vars.vec_str_item(var.location, var_str.to_string(), index, memory)?
                        }
                        "lxstring" => {
                            registers.lxstring =
                                vars.vec_str_item(var.location, var_str.to_string(), index, memory)?
                        }
                        _ => {
                            let assign_var = vars.get_type(to_assign.to_string())?;
                            if assign_var.name == TypeName::String {
                                let str = vars.vec_str_item(
                                    var.location,
                                    var_str.to_string(),
                                    index,
                                    memory,
                                )?;
                                let new_str_addr = memory.malloc(str.as_bytes());
                                let heap_addr: [u8; 4] = memory
                                    .load(assign_var.clone().location)
                                    .try_into()
                                    .expect("illegal heap addr");
                                memory.stack_mod(assign_var.location, &new_str_addr.to_be_bytes());
                                memory.free(u32::from_be_bytes(heap_addr));
                            } else {
                                return Err(ErrorKind::ExpectedStr(to_assign.to_string()));
                            }
                        }
                    }
                } else {
                    return Err(ErrorKind::ExpectedVec(var_str.to_string()));
                }
            } else {
                let var_exp = statement.split('=').collect::<Vec<&str>>()[0].trim();
                let var_str_vec = var_exp.split('[').collect::<Vec<&str>>();
                let idx_str = var_str_vec[1].split(']').collect::<Vec<&str>>()[0].trim();
                let var_str = var_str_vec[0]
                    .split_ascii_whitespace()
                    .collect::<Vec<&str>>()[1]
                    .trim();
                let idx: usize;
                match idx_str {
                    "lx" => idx = registers.lx as usize,
                    "rv" => idx = registers.rv as usize,
                    _ => {
                        let num = idx_str.parse::<i32>();
                        match num {
                            Ok(n) => idx = n as usize,
                            _parse_int_error => {
                                let idx_var = vars.get_type(idx_str.to_string())?;
                                if idx_var.name == TypeName::I32 {
                                    idx = memory.yeild_i32(idx_var.location) as usize;
                                } else {
                                    return Err(ErrorKind::ExpectedInt(idx_str.to_string()));
                                }
                            }
                        }
                    }
                }
                let var = vars.get_type(var_str.to_string())?;
                let assign_to = statement.split('=').collect::<Vec<&str>>()[1].trim();
                if var.name == TypeName::Vector(Vector::Int) {
                    match assign_to {
                        "lx" => {
                            let vec_mod = vars.get_vec_int_mod(var, registers.lx, memory);
                            match memory.mod_vec_int(&vec_mod.heap_addr, idx, &vec_mod.value_bytes)
                            {
                                Ok(()) => (),
                                Err(()) => return Err(ErrorKind::VecLen(var_str.to_string())),
                            }
                        }
                        "rv" => {
                            let vec_mod = vars.get_vec_int_mod(var, registers.rv, memory);
                            match memory.mod_vec_int(&vec_mod.heap_addr, idx, &vec_mod.value_bytes)
                            {
                                Ok(()) => (),
                                Err(()) => return Err(ErrorKind::VecLen(var_str.to_string())),
                            }
                        }
                        _ => {
                            let assigned_var = vars.get_type(assign_to.to_string())?;
                            if assigned_var.name == TypeName::I32 {
                                let vec_mod = vars.get_vec_int_mod(
                                    var,
                                    memory.yeild_i32(assigned_var.location),
                                    memory,
                                );
                                match memory.mod_vec_int(
                                    &vec_mod.heap_addr,
                                    idx,
                                    &vec_mod.value_bytes,
                                ) {
                                    Ok(()) => (),
                                    Err(()) => return Err(ErrorKind::VecLen(var_str.to_string())),
                                }
                            } else {
                                return Err(ErrorKind::ExpectedInt(assign_to.to_string()));
                            }
                        }
                    }
                } else if var.name == TypeName::Vector(Vector::String) {
                    match assign_to {
                        "string" => {
                            let vec_mod = vars.get_vec_str_mod(var, &registers.string, memory);
                            match memory.mod_vec_str(&vec_mod.heap_addr, idx, &vec_mod.value_bytes)
                            {
                                Ok(()) => (),
                                Err(()) => return Err(ErrorKind::VecLen(var_str.to_string())),
                            }
                        }
                        "lxstring" => {
                            let vec_mod = vars.get_vec_str_mod(var, &registers.lxstring, memory);
                            match memory.mod_vec_str(&vec_mod.heap_addr, idx, &vec_mod.value_bytes)
                            {
                                Ok(()) => (),
                                Err(()) => return Err(ErrorKind::VecLen(var_str.to_string())),
                            }
                        }
                        _ => {
                            let assigned_var = vars.get_type(assign_to.to_string())?;
                            if assigned_var.name == TypeName::String {
                                let vec_mod = vars.get_vec_str_mod(
                                    var,
                                    memory.yeild_string(assigned_var.location).trim(),
                                    memory,
                                );
                                match memory.mod_vec_str(
                                    &vec_mod.heap_addr,
                                    idx,
                                    &vec_mod.value_bytes,
                                ) {
                                    Ok(()) => (),
                                    Err(()) => return Err(ErrorKind::VecLen(var_str.to_string())),
                                }
                            } else {
                                return Err(ErrorKind::ExpectedStr(assign_to.to_string()));
                            }
                        }
                    }
                } else {
                    return Err(ErrorKind::ExpectedVec(var_str.to_string()));
                }
            }
        }
    }

    Ok(())
}
