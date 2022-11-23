use crate::{
    memory::Memory,
    types::{Type, TypeName, Vars},
    CPU, funcs::errors::ErrorKind,
};

pub fn sub(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
) -> Result<(), ErrorKind> {
    if cmd.len() == 1 {
        let n1: i32 = memory.get_int_from_stack()?;
        let sub = memory.stack.len().saturating_sub(4);
        memory.stack.truncate(sub);

        let n2: i32 = memory.get_int_from_stack()?;
        let sub = memory.stack.len().saturating_sub(4);
        memory.stack.truncate(sub);
        memory.set_int_to_stack(n2 - n1);
        Ok(())
    } else {
        match cmd[1] {
            "lx" => {
                if cmd.len() == 2 {
                    let n: i32 = memory.get_int_from_stack()?;
                    let sub = memory.stack.len().saturating_sub(4);
                    memory.stack.truncate(sub);
                    memory.set_int_to_stack(n - registers.lx);
                    Ok(())
                } else {
                    match cmd[2] {
                        "rv" => memory.set_int_to_stack(registers.lx - registers.rv),
                        "lx" => memory.set_int_to_stack(registers.lx - registers.lx),
                        _ => {
                            let n: i32;
                            let t: Type = vars.get_type(cmd[2].to_string())?;
                            match t.name {
                                TypeName::I32 => n = memory.yeild_i32(t.location),
                                _ => return Err(ErrorKind::ExpectedInt(cmd[2].to_string())),
                            }
                            memory.set_int_to_stack(registers.lx - n)
                        }
                    }
                    Ok(())
                }
            }
            "rv" => {
                if cmd.len() == 2 {
                    let n: i32 = memory.get_int_from_stack()?;
                    let sub = memory.stack.len().saturating_sub(4);
                    memory.stack.truncate(sub);
                    memory.set_int_to_stack(n - registers.rv);
                    Ok(())
                } else {
                    match cmd[2] {
                        "rv" => memory.set_int_to_stack(registers.rv - registers.rv),
                        "lx" => memory.set_int_to_stack(registers.rv - registers.lx),
                        _ => {
                            let n: i32;
                            let t: Type = vars.get_type(cmd[2].to_string())?;
                            match t.name {
                                TypeName::I32 => n = memory.yeild_i32(t.location),
                                _ => return Err(ErrorKind::ExpectedInt(cmd[2].to_string())),
                            }
                            memory.set_int_to_stack(registers.rv - n)
                        }
                    }
                    Ok(())
                }
            }
            _ => {
                if cmd.len() == 2 {
                    let var: i32;
                    let t: Type = vars.get_type(cmd[1].to_string())?;
                    match t.name {
                        TypeName::I32 => var = memory.yeild_i32(t.location),
                        _ => return Err(ErrorKind::ExpectedInt(cmd[2].to_string())),
                    }
                    let n: i32 = memory.get_int_from_stack()?;
                    let sub = memory.stack.len().saturating_sub(4);
                    memory.stack.truncate(sub);
                    memory.set_int_to_stack(var - n);
                    Ok(())
                } else if cmd.len() == 3 {
                    let var: i32;
                    let t: Type = vars.get_type(cmd[1].to_string())?;
                    match t.name {
                        TypeName::I32 => var = memory.yeild_i32(t.location),
                        _ => return Err(ErrorKind::ExpectedInt(cmd[2].to_string())),
                    }
                    match cmd[2] {
                        "lx" => memory.set_int_to_stack(var - registers.lx),
                        "rv" => memory.set_int_to_stack(var - registers.rv),
                        _ => {
                            let var2: i32;
                            let t2: Type = vars.get_type(cmd[2].to_string())?;
                            match t2.name {
                                TypeName::I32 => var2 = memory.yeild_i32(t2.location),
                                _ => return Err(ErrorKind::ExpectedInt(cmd[2].to_string())),
                            }
                            memory.set_int_to_stack(var - var2);
                        }
                    }
                    Ok(())
                } else {
                    Ok(())
                }
            }
        }
    }
}
