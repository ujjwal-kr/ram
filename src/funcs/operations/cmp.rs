use crate::{
    funcs::errors,
    memory::Memory,
    types::{TypeName, Vars},
    CPU,
};

pub fn cmp(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
    b: &str,
    l: i32,
) {
    if cmd.len() == 1 {
        let num_1 = memory.get_int_from_stack(b, l);
        let sub = memory.stack.len().saturating_sub(4);
        memory.stack.truncate(sub);

        let num_2 = memory.get_int_from_stack(b, l);
        let sub = memory.stack.len().saturating_sub(4);
        memory.stack.truncate(sub);
        let diff = num_2 - num_1;
        if diff == 0 {
            memory.set_int_to_stack(0)
        } else if diff > 0 {
            memory.set_int_to_stack(1)
        } else if diff < 0 {
            memory.set_int_to_stack(-1)
        }
    } else if cmd.len() == 3 {
        match cmd[1] {
            "lx" => {
                if cmd[2] == "rv" {
                    let diff = registers.lx - registers.rv;
                    if diff == 0 {
                        memory.set_int_to_stack(0)
                    } else if diff > 0 {
                        memory.set_int_to_stack(1)
                    } else if diff < 0 {
                        memory.set_int_to_stack(-1)
                    }
                } else {
                    let var = vars.get_type(cmd[2].to_string(), b, l);
                    if var.name == TypeName::I32 {
                        let value = memory.yeild_i32(var.location);
                        let diff = registers.lx - value;
                        if diff == 0 {
                            memory.set_int_to_stack(0)
                        } else if diff > 0 {
                            memory.set_int_to_stack(1)
                        } else if diff < 0 {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        panic!("Invalid type casting at {}{}", b, l)
                    }
                }
            }
            "rv" => {
                if cmd[2] == "lx" {
                    let diff = registers.rv - registers.lx;
                    if diff == 0 {
                        memory.set_int_to_stack(0)
                    } else if diff > 0 {
                        memory.set_int_to_stack(1)
                    } else if diff < 0 {
                        memory.set_int_to_stack(-1)
                    }
                } else {
                    let var = vars.get_type(cmd[2].to_string(), b, l);
                    if var.name == TypeName::I32 {
                        let value = memory.yeild_i32(var.location);
                        let diff = registers.rv - value;
                        if diff == 0 {
                            memory.set_int_to_stack(0)
                        } else if diff > 0 {
                            memory.set_int_to_stack(1)
                        } else if diff < 0 {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        panic!("Invalid type casting at {}{}", b, l)
                    }
                }
            }
            "string" => {
                if cmd[2] == "lxstring" {
                    if registers.string == registers.lxstring {
                        memory.set_int_to_stack(0)
                    } else {
                        memory.set_int_to_stack(-1)
                    }
                } else {
                    let var = vars.get_type(cmd[2].to_string(), b, l);
                    if var.name == TypeName::String {
                        if registers.string == memory.yeild_string(var.location) {
                            memory.set_int_to_stack(0)
                        } else {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        panic!("Invalid type casting at {}{}", b, l)
                    }
                }
            }
            "lxstring" => {
                if cmd[2] == "string" {
                    if registers.string == registers.lxstring {
                        memory.set_int_to_stack(0)
                    } else {
                        memory.set_int_to_stack(-1)
                    }
                } else {
                    let var = vars.get_type(cmd[2].to_string(), b, l);
                    if var.name == TypeName::String {
                        if registers.lxstring == memory.yeild_string(var.location) {
                            memory.set_int_to_stack(0)
                        } else {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        panic!("Invalid type casting at {}{}", b, l)
                    }
                }
            }
            _ => match cmd[2] {
                "lx" => {
                    let var_1 = vars.get_type(cmd[1].to_string(), b, l);
                    if var_1.name == TypeName::I32 {
                        let value = memory.yeild_i32(var_1.location);
                        let diff = value - registers.lx;
                        if diff == 0 {
                            memory.set_int_to_stack(0)
                        } else if diff > 0 {
                            memory.set_int_to_stack(1)
                        } else if diff < 0 {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        panic!("Invalid type casting at {}{}", b, l)
                    }
                }
                "rv" => {
                    let var_1 = vars.get_type(cmd[1].to_string(), b, l);
                    if var_1.name == TypeName::I32 {
                        let value = memory.yeild_i32(var_1.location);
                        let diff = value - registers.rv;
                        if diff == 0 {
                            memory.set_int_to_stack(0)
                        } else if diff > 0 {
                            memory.set_int_to_stack(1)
                        } else if diff < 0 {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        panic!("Invalid type casting at {}{}", b, l)
                    }
                }
                "string" => {
                    let var_1 = vars.get_type(cmd[1].to_string(), b, l);
                    if var_1.name == TypeName::String {
                        if registers.string == memory.yeild_string(var_1.location) {
                            memory.set_int_to_stack(0)
                        } else {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        panic!("Invalid type casting at {}{}", b, l)
                    }
                }
                "lxstring" => {
                    let var_1 = vars.get_type(cmd[1].to_string(), b, l);
                    if var_1.name == TypeName::String {
                        if registers.lxstring == memory.yeild_string(var_1.location) {
                            memory.set_int_to_stack(0)
                        } else {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        panic!("Invalid type casting at {}{}", b, l)
                    }
                }
                _ => {
                    let var_1 = vars.get_type(cmd[1].to_string(), b, l);
                    let var_2 = vars.get_type(cmd[2].to_string(), b, l);

                    if var_1.name == var_2.name {
                        match var_1.name {
                            TypeName::I32 => {
                                let num_1 = memory.yeild_i32(var_1.location);
                                let num_2 = memory.yeild_i32(var_2.location);
                                let diff = num_1 - num_2;
                                if diff == 0 {
                                    memory.set_int_to_stack(0)
                                } else if diff > 0 {
                                    memory.set_int_to_stack(1)
                                } else if diff < 0 {
                                    memory.set_int_to_stack(-1)
                                }
                            }
                            TypeName::String => {
                                let str_1 = memory.yeild_string(var_1.location);
                                let str_2 = memory.yeild_string(var_2.location);

                                if str_1 == str_2 {
                                    memory.set_int_to_stack(0)
                                } else {
                                    memory.set_int_to_stack(-1)
                                }
                            }
                            _ => todo!("Implement for other types"),
                        }
                    } else {
                        panic!("Invalid type casting at {}{}", b, l)
                    }
                }
            },
        }
    } else {
        errors::args_error(b, l)
    }
}
