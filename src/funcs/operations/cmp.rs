use crate::{
    funcs::errors::ErrorKind,
    memory::Memory,
    types::{TypeName, Vars},
    CPU,
};

pub fn cmp(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
) -> Result<(), ErrorKind> {
    if cmd.len() == 1 {
        let num_1 = memory.get_int_from_stack()?;
        let sub = memory.stack.len().saturating_sub(4);
        memory.stack.truncate(sub);

        let num_2 = memory.get_int_from_stack()?;
        let sub = memory.stack.len().saturating_sub(4);
        memory.stack.truncate(sub);
        let diff = num_2 - num_1;
        if diff == 0 {
            memory.set_int_to_stack(0);
        } else if diff > 0 {
            memory.set_int_to_stack(1);
        } else if diff < 0 {
            memory.set_int_to_stack(-1);
        }
        Ok(())
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
                    let var = vars.get_type(cmd[2].to_string())?;
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
                        return Err(ErrorKind::Casting {
                            src: cmd[2].to_string(),
                            dest: "lx".to_string(),
                        });
                    }
                }
                Ok(())
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
                    let var = vars.get_type(cmd[2].to_string())?;
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
                        return Err(ErrorKind::Casting {
                            src: cmd[2].to_string(),
                            dest: "lx".to_string(),
                        });
                    }
                }
                Ok(())
            }
            "string" => {
                if cmd[2] == "lxstring" {
                    if registers.string == registers.lxstring {
                        memory.set_int_to_stack(0)
                    } else {
                        memory.set_int_to_stack(-1)
                    }
                } else {
                    let var = vars.get_type(cmd[2].to_string())?;
                    if var.name == TypeName::String {
                        if registers.string == memory.yeild_string(var.location) {
                            memory.set_int_to_stack(0)
                        } else {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        return Err(ErrorKind::Casting {
                            src: cmd[2].to_string(),
                            dest: "string".to_string(),
                        });
                    }
                }
                Ok(())
            }
            "lxstring" => {
                if cmd[2] == "string" {
                    if registers.string == registers.lxstring {
                        memory.set_int_to_stack(0)
                    } else {
                        memory.set_int_to_stack(-1)
                    }
                } else {
                    let var = vars.get_type(cmd[2].to_string())?;
                    if var.name == TypeName::String {
                        if registers.lxstring == memory.yeild_string(var.location) {
                            memory.set_int_to_stack(0)
                        } else {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        return Err(ErrorKind::Casting {
                            src: cmd[2].to_string(),
                            dest: "lxstring".to_string(),
                        });
                    }
                }
                Ok(())
            }
            _ => match cmd[2] {
                "lx" => {
                    let var_1 = vars.get_type(cmd[1].to_string())?;
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
                        return Err(ErrorKind::Casting {
                            src: "lx".to_string(),
                            dest: cmd[1].to_string(),
                        });
                    }
                    Ok(())
                }
                "rv" => {
                    let var_1 = vars.get_type(cmd[1].to_string())?;
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
                        return Err(ErrorKind::Casting {
                            src: "rv".to_string(),
                            dest: cmd[1].to_string(),
                        });
                    }
                    Ok(())
                }
                "string" => {
                    let var_1 = vars.get_type(cmd[1].to_string())?;
                    if var_1.name == TypeName::String {
                        if registers.string == memory.yeild_string(var_1.location) {
                            memory.set_int_to_stack(0)
                        } else {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        return Err(ErrorKind::Casting {
                            src: "string".to_string(),
                            dest: cmd[1].to_string(),
                        });
                    }
                    Ok(())
                }
                "lxstring" => {
                    let var_1 = vars.get_type(cmd[1].to_string())?;
                    if var_1.name == TypeName::String {
                        if registers.lxstring == memory.yeild_string(var_1.location) {
                            memory.set_int_to_stack(0)
                        } else {
                            memory.set_int_to_stack(-1)
                        }
                    } else {
                        return Err(ErrorKind::Casting {
                            src: "string".to_string(),
                            dest: cmd[1].to_string(),
                        });
                    }
                    Ok(())
                }
                _ => {
                    let var_1 = vars.get_type(cmd[1].to_string())?;
                    let var_2 = vars.get_type(cmd[2].to_string())?;

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
                        Ok(())
                    } else {
                        return Err(ErrorKind::Casting {
                            src: cmd[1].to_string(),
                            dest: cmd[2].to_string(),
                        });
                    }
                }
            },
        }
    } else {
        Err(ErrorKind::ArgErr)
    }
}
