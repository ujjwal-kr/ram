use crate::types::Vector;
use crate::types::{Type, TypeName, Vars};
use crate::{memory::Memory, CPU};

use super::errors::{self, ErrorKind};

pub fn ram(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
    statement: &str,
) -> Result<(), ErrorKind> {
    // ram 10
    // ram lx/rv 10
    // ram lx/rv prev
    // ram lx
    // ram rv
    // ram string/lxstring = 'hello world'
    // ram <var> :str = 'hello world'
    // ram <var> :int = 10
    // ram <var> :vec :int = [1,2,3]
    // ram <var> :vec :str = ['meow', 'dog']

    if cmd.len() < 2 {
        return Err(ErrorKind::ArgErr);
    }

    match cmd[1] {
        "lx" => {
            if cmd.len() == 2 {
                // ram lx
                vars.set_int_to_stack(memory, registers.lx.to_string().trim())?
            } else if cmd[2] == "prev" {
                // ram lx prev
                registers.lx = memory.get_int_from_stack()?;
                let sub = memory.stack.len().saturating_sub(4);
                memory.stack.truncate(sub);
            } else {
                // parse cmd[2] as int
                registers.lx = errors::parse_int(cmd[2])?
            }
        }
        "rv" => {
            if cmd.len() == 2 {
                // ram rv
                vars.set_int_to_stack(memory, registers.rv.to_string().trim())?;
            } else if cmd[2] == "prev" {
                // ram lx prev
                registers.rv = memory.get_int_from_stack()?;
                let sub = memory.stack.len().saturating_sub(4);
                memory.stack.truncate(sub);
            } else {
                // parse cmd[2] as int
                registers.rv = errors::parse_int(cmd[2])?;
            }
        }
        "string" => {
            // ram string = 'hello'
            let exp = statement.split('=').collect::<Vec<&str>>()[1].trim();
            let _ = &exp[1..exp.len() - 1].clone_into(&mut registers.string);
        }
        "lxstring" => {
            let exp = statement.split('=').collect::<Vec<&str>>()[1].trim();
            let _ = &exp[1..exp.len() - 1].clone_into(&mut registers.lxstring);
        }
        _ => {
            if cmd.len() > 3 {
                if &cmd[2][0..1] == ":" {
                    let name = cmd[1];
                    let exp = statement.split('=').collect::<Vec<&str>>()[1].trim();
                    match &cmd[2][1..cmd[2].len()] {
                        "str" => vars.set_string(name.to_string(), &exp[1..exp.len() - 1], memory),
                        "int" => vars.set_int(name.to_string(), exp, memory)?,
                        "vec" => {
                            if &cmd[3][0..1] == ":" {
                                match &cmd[3][1..cmd[3].len()] {
                                    "str" => vars.set_str_vec(name.to_string(), exp, memory),
                                    "int" => vars.set_int_vec(name.to_string(), exp, memory)?,
                                    _ => return Err(ErrorKind::ArgErr),
                                }
                            } else {
                                return Err(ErrorKind::ArgErr);
                            }
                        }
                        _ => return Err(ErrorKind::ArgErr),
                    }
                } else {
                    return Err(ErrorKind::ArgErr);
                }
            } else {
                // try to parse cmd[1] as int
                vars.set_int_to_stack(memory, cmd[1])?
            }
        }
    }
    Ok(())
}

pub fn copy(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
    statement: &str,
) -> Result<(), ErrorKind> {
    if cmd.len() < 4 || cmd[2] != "=" {
        return Err(ErrorKind::ArgErr);
    }

    let src: &str = statement.split('=').collect::<Vec<&str>>()[1].trim();
    let dest: &str = cmd[1];

    match dest {
        "lx" => match src {
            "rv" => registers.rv = registers.lx,
            _ => {
                let t = vars.get_type(src.to_string())?;
                let var: i32;
                if t.name == TypeName::I32 {
                    var = memory.yeild_i32(t.location);
                    registers.lx = var
                } else {
                    return Err(ErrorKind::ExpectedInt(src.to_string()));
                }
            }
        },

        "rv" => match src {
            "lx" => registers.lx = registers.rv,
            _ => {
                let t = vars.get_type(src.to_string())?;
                let var: i32;
                if t.name == TypeName::I32 {
                    var = memory.yeild_i32(t.location);
                    registers.rv = var
                } else {
                    return Err(ErrorKind::ExpectedInt(src.to_string()));
                }
            }
        },

        "string" => match src {
            "lxstring" => registers.string = registers.lxstring.clone(),
            _ => {
                let t = vars.get_type(src.to_string())?;
                let var: String;
                if t.name == TypeName::String {
                    var = memory.yeild_string(t.location);
                    registers.string = var
                } else {
                    return Err(ErrorKind::ExpectedStr(src.to_string()));
                }
            }
        },

        "lxstring" => match src {
            "string" => registers.lxstring = registers.string.clone(),
            _ => {
                let t = vars.get_type(src.to_string())?;
                let var: String;
                if t.name == TypeName::String {
                    var = memory.yeild_string(t.location);
                    registers.lxstring = var
                } else {
                    return Err(ErrorKind::ExpectedStr(src.to_string()));
                }
            }
        },

        _ => {
            match src {
                "lx" => {
                    let t: Type;
                    t = vars.get_type(dest.to_string())?;
                    if t.name == TypeName::I32 {
                        memory.stack_mod(t.location, &registers.lx.to_be_bytes())
                    } else {
                        return Err(ErrorKind::ExpectedInt(dest.to_string()));
                    }
                }
                "rv" => {
                    let t: Type;
                    t = vars.get_type(dest.to_string())?;
                    if t.name == TypeName::I32 {
                        memory.stack_mod(t.location, &registers.rv.to_be_bytes())
                    } else {
                        return Err(ErrorKind::ExpectedInt(dest.to_string()));
                    }
                }
                "string" => {
                    let t: Type;
                    t = vars.get_type(dest.to_string())?;
                    if t.name == TypeName::String {
                        // gc
                        let addr: [u8; 4] = memory
                            .load(t.location)
                            .try_into()
                            .expect("Error converting location to addr");
                        memory.free(u32::from_be_bytes(addr));
                        // end gc
                        vars.set_string(dest.to_string(), &registers.string, memory)
                    } else {
                        return Err(ErrorKind::ExpectedStr(dest.to_string()));
                    }
                }
                "lxstring" => {
                    let t: Type;
                    t = vars.get_type(dest.to_string())?;
                    if t.name == TypeName::String {
                        // gc
                        let addr: [u8; 4] = memory
                            .load(t.location)
                            .try_into()
                            .expect("Error converting location to addr");
                        memory.free(u32::from_be_bytes(addr));
                        // end gc
                        vars.set_string(dest.to_string(), &registers.lxstring, memory)
                    } else {
                        return Err(ErrorKind::ExpectedStr(dest.to_string()));
                    }
                }
                _ => match vars.cast(src, dest, memory)? {
                    Some(cast_stack) => {
                        memory.stack_mod(cast_stack.dest_location, &cast_stack.src_data)
                    }
                    None => (),
                },
            }
        }
    }
    Ok(())
}

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
                            let var2_int = memory.yeild_i32(var2.clone().location);
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
            let var = vars.get_type(cmd[1].to_string()).unwrap();
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
                let var_exp = statement.split("=").collect::<Vec<&str>>()[1].trim();
                let var_str = var_exp.split("[").collect::<Vec<&str>>()[0].trim();
                let idx_str = var_exp.split("[").collect::<Vec<&str>>()[1]
                    .trim()
                    .split("]")
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
                let var_exp = statement.split("=").collect::<Vec<&str>>()[0].trim();
                let var_str_vec = var_exp.split("[").collect::<Vec<&str>>();
                let idx_str = var_str_vec[1].split("]").collect::<Vec<&str>>()[0].trim();
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
                let assign_to = statement.split("=").collect::<Vec<&str>>()[1].trim();
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
