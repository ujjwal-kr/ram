use crate::types::{Vars, TypeName};
use crate::{memory::Memory, Registers};

use super::errors;

pub fn ram(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut Registers,
    cmd: Vec<&str>,
    statement: &str,
    b: &str,
    l: i32,
) {
    // ram 10
    // ram lx/rv 10
    // ram lx/rv prev
    // ram lx
    // ram rv
    // ram string/lxstring = hello world
    // ram <var> :str = hello world
    // ram <var> :int = 10
    // ram <var> :vec :int = [1,2,3]
    // ram <var> :vec :str = [meow, dog]

    if cmd.len() < 2 {
        errors::args_error(b, l);
    }

    match cmd[1] {
        "lx" => {
            if cmd.len() == 2 {
                // ram lx
                vars.set_int_to_stack(memory, registers.lx.to_string().trim(), b, l)
            } else if cmd[2] == "prev" {
                // ram lx prev
                registers.lx = memory.get_int_from_stack(b, l);
                for _ in 0..4 {
                    memory.pop_stack()
                }
            } else {
                // parse cmd[2] as int
                registers.lx = errors::parse_int(cmd[2], b, l)
            }
        }
        "rv" => {
            if cmd.len() == 2 {
                // ram rv
                vars.set_int_to_stack(memory, registers.rv.to_string().trim(), b, l)
            } else if cmd[2] == "prev" {
                // ram lx prev
                registers.rv = memory.get_int_from_stack(b, l);
                for _ in 0..4 {
                    memory.pop_stack()
                }
            } else {
                // parse cmd[2] as int
                registers.rv = errors::parse_int(cmd[2], b, l)
            }
        }
        "string" => {
            // ram string = hello
            let exp = statement.split('=').collect::<Vec<&str>>()[1].trim();
            registers.string = exp.to_string();
        }
        "lxstring" => {
            let exp = statement.split('=').collect::<Vec<&str>>()[1].trim();
            registers.lxstring = exp.to_string();
        }
        _ => {
            if cmd.len() > 3 {
                if &cmd[2][0..1] == ":" {
                    let name = cmd[1];
                    let exp = statement.split('=').collect::<Vec<&str>>()[1].trim();
                    match &cmd[2][1..cmd[2].len()] {
                        "str" => vars.set_string(name.to_string(), exp, memory),
                        "int" => vars.set_int(name.to_string(), exp, memory, b, l),
                        "vec" => {
                            if &cmd[3][0..1] == ":" {
                                match &cmd[3][1..cmd[3].len()] {
                                    "str" => vars.set_str_vec(name.to_string(), exp, memory),
                                    "int" => vars.set_int_vec(name.to_string(), exp, memory, b, l),
                                    _ => errors::args_error(b, l),
                                }
                            } else {
                                errors::args_error(b, l)
                            }
                        }
                        _ => errors::args_error(b, l),
                    }
                } else {
                    errors::args_error(b, l)
                }
            } else {
                // try to parse cmd[1] as int
                vars.set_int_to_stack(memory, cmd[1], b, l)
            }
        }
    }
}

pub fn copy(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut Registers,
    cmd: Vec<&str>,
    statement: &str,
    b: &str,
    l: i32,
) {
    if cmd.len() < 4 || cmd[2] != "=" {
        errors::args_error(b, l)
    }

    let src: &str = statement.split('=').collect::<Vec<&str>>()[1].trim();

    match cmd[1] {
        "lx" => match src {
            "rv" => registers.rv = registers.lx,
            _ => {
                let t = vars.get_type(src.to_string(), b, l);
                let var: i32;
                if t.name == TypeName::I32 {
                    var = memory.yeild_i32(t.location);
                    registers.lx = var
                } else {
                    panic!("Expected {} to be int at {}{}", src, b, l)
                }
            },
        },

        "rv" => match src {
            "lx" => registers.lx = registers.rv,
            _ => {
                let t = vars.get_type(src.to_string(), b, l);
                let var: i32;
                if t.name == TypeName::I32 {
                    var = memory.yeild_i32(t.location);
                    registers.rv = var
                } else {
                    panic!("Expected {} to be int at {}{}", src, b, l)
                }
            }
        },

        "string" => match src {
            "lxstring" => registers.string = registers.lxstring.clone(),
            _ => {
                let t = vars.get_type(src.to_string(), b, l);
                let var: String;
                if t.name == TypeName::String {
                    var = memory.yeild_string(t.location);
                    registers.string = var
                } else {
                    panic!("Expected {} to be string at {}{}", src, b, l)
                }
            }
        },

        "lxstring" => match src {
            "string" => registers.lxstring = registers.string.clone(),
            _ => {
                let t = vars.get_type(src.to_string(), b, l);
                let var: String;
                if t.name == TypeName::String {
                    var = memory.yeild_string(t.location);
                    registers.lxstring = var
                } else {
                    panic!("Expected {} to be string at {}{}", src, b, l)
                }
            }
        },

        _ => (),
    }
}
