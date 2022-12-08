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
