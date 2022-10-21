use super::errors;
use crate::types::{TypeName, Vars};
use crate::{memory::Memory, Registers};

pub fn cmp(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut Registers,
    cmd: Vec<&str>,
    b: &str,
    l: i32,
) {
    if cmd.len() == 1 {
        if memory.stack.len() < 8 {
            super::errors::stack_len_error(b, l);
        }
        let num_1 = memory.get_int_from_stack();
        for _ in 0..4 {
            memory.pop_stack()
        }
        let num_2 = memory.get_int_from_stack();
        for _ in 0..4 {
            memory.pop_stack()
        }
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

pub fn add(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut Registers,
    cmd: Vec<&str>,
    statement: &str,
    b: &str,
    l: i32,
) {
}

pub fn sub(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut Registers,
    cmd: Vec<&str>,
    statement: &str,
    b: &str,
    l: i32,
) {
}

pub fn mul(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut Registers,
    cmd: Vec<&str>,
    statement: &str,
    b: &str,
    l: i32,
) {
}
// pub fn sub(stack: &mut Vec<u8>, b: &str, l: u32) {
//     if stack.len() < 2 {
//         super::errors::stack_len_error(b, l);
//     }
//     let result = stack[stack.len() - 2] - stack[stack.len() - 1];
//     stack.push(result);
// }

// pub fn mul(stack: &mut Vec<u8>, cmd: Vec<&str>, vars: &mut super::super::Vars, b: &str, l: u32) {
//     if cmd.len() > 1 {
//         stack.push(vars.lx * vars.rv);
//     } else {
//         if stack.len() < 2 {
//             super::errors::stack_len_error(b, l);
//         }
//         let result = stack[stack.len() - 1] * stack[stack.len() - 2];
//         stack.push(result);
//     }
// }

// pub fn div(stack: &mut Vec<u8>, b: &str, l: u32) {
//     if stack.len() < 2 {
//         super::errors::stack_len_error(b, l);
//     }
//     let result = stack[stack.len() - 2] / stack[stack.len() - 1];
//     stack.push(result)
// }

// pub fn sqr(stack: &mut Vec<u8>, cmd: Vec<&str>, vars: &mut super::super::Vars, b: &str, l: u32) {
//     if cmd.len() == 1 {
//         if stack.is_empty() {
//             super::errors::stack_len_error(b, l);
//         }
//         let result = stack[stack.len() - 1] * stack[stack.len() - 1];
//         stack.push(result);
//     } else {
//         if cmd[1].trim() == "lx" || cmd[1].trim() == "rv" {
//             if cmd[1].trim() == "lx" {
//                 vars.lx = vars.lx * vars.lx;
//             }
//             if cmd[1].trim() == "rv" {
//                 vars.rv = vars.rv * vars.rv;
//             }
//         } else {
//             super::errors::args_error(b, l);
//         }
//     }
// }

// pub fn sqrt(stack: &mut Vec<u8>, cmd: Vec<&str>, vars: &mut super::super::Vars, b: &str, l: u32) {
//     if cmd.len() == 1 {
//         if stack.is_empty() {
//             super::errors::stack_len_error(b, l);
//         } else {
//             let result = stack[stack.len() - 1].sqrt();
//             stack.push(result);
//         }
//     } else {
//         if cmd[1] == "lx" || cmd[1] == "rv" {
//             if cmd[1].trim() == "lx" {
//                 vars.lx = vars.lx.sqrt();
//             }
//             if cmd[1].trim() == "rv" {
//                 vars.rv = vars.rv.sqrt();
//             }
//         } else {
//             super::errors::args_error(b, l);
//         }
//     }
// }

// pub fn round(stack: &mut Vec<u8>, cmd: Vec<&str>, vars: &mut super::super::Vars, b: &str, l: u32) {
//     if cmd.len() > 1 {
//         if cmd[1].trim() == "lx" || cmd[1].trim() == "rv" {
//             if cmd[1].trim() == "lx" {
//                 vars.lx = vars.lx.round();
//             }
//             if cmd[1].trim() == "rv" {
//                 vars.rv = vars.rv.round();
//             }
//         }
//     } else {
//         if stack.is_empty() {
//             super::errors::stack_len_error(b, l);
//         }
//         let result = stack[stack.len() - 1].round();
//         stack.push(result);
//     }
// }

// pub fn avg(stack: &mut Vec<u8>, b: &str, l: u32) {
//     if stack.is_empty() {
//         super::errors::stack_len_error(b, l);
//     }
//     let mut total: f64 = 0.0;
//     let mut i: f64 = 0.0;
//     for num in stack.clone() {
//         i = i + 1.0;
//         total = total + num
//     }
//     stack.push(total / i)
// }

// // strings

// pub fn split(cmd: Vec<&str>, statement: &str, vars: &mut super::super::Vars, b: &str, l: u32) {
//     if cmd.len() < 3 {
//         super::errors::args_error(b, l);
//     } else {
//         let lits: Vec<&str> = statement.split(">>").collect();
//         let string: &str = vars.string.trim();
//         let str_arg: &str = lits[1].trim();
//         let slice = &str_arg[1..str_arg.len() - 1];
//         if slice == r"\n" || slice == r"\n\n" {
//             if slice == r"\n" {
//                 let str_vec: Vec<&str> = string.split("\n").collect();
//                 vars.str_vec = vec![];
//                 for items in str_vec {
//                     vars.str_vec.push(items.to_string());
//                 }
//             } else if slice == r"\n\n" {
//                 let str_vec: Vec<&str> = string.split("\n\n").collect();
//                 vars.str_vec = vec![];
//                 for items in str_vec {
//                     vars.str_vec.push(items.to_string());
//                 }
//             } else if slice == r"\t" {
//                 let str_vec: Vec<&str> = string.split("\t").collect();
//                 vars.str_vec = vec![];
//                 for items in str_vec {
//                     vars.str_vec.push(items.to_string());
//                 }
//             }
//         } else {
//             let str_vec: Vec<&str> = string.split(slice).collect();
//             vars.str_vec = vec![];
//             for items in str_vec {
//                 vars.str_vec.push(items.to_string());
//             }
//         }
//     }
// }

// // vectors

// pub fn vec_ops(
//     stack: &mut Vec<u8>,
//     cmd: Vec<&str>,
//     statement: &str,
//     vars: &mut super::super::Vars,
//     b: &str,
//     l: u32,
// ) {
//     if cmd.len() < 3 {
//         super::errors::args_error(b, l);
//     }
//     match cmd[1] {
//         "str" => match cmd[2].trim() {
//             "push" => vars.str_vec.push(vars.string.clone().trim().to_string()),
//             "pop" => {
//                 if vars.str_vec.is_empty() {
//                     super::errors::vec_items(b, l);
//                 } else {
//                     vars.str_vec.pop();
//                 }
//             }
//             "len" => {
//                 stack.push(super::errors::parse_float(
//                     vars.str_vec.len().to_string().trim(),
//                     b,
//                     l,
//                 ));
//             }
//             ">>" => {
//                 let lits: Vec<&str> = statement.split(">>").collect();
//                 let data_vec = lits[1].trim();
//                 let slice = &data_vec[1..data_vec.len() - 1];
//                 let index: usize = ret_index(slice.trim(), vars, b, l);
//                 if index >= vars.str_vec.len() {
//                     super::errors::invalid_index(b, l, index);
//                 }
//                 vars.string = vars.str_vec[index].to_string();
//             }
//             _ => super::errors::args_error(b, l),
//         },
//         "int" => match cmd[2].trim() {
//             "push" => match cmd[3].trim() {
//                 "lx" => vars.num_vec.push(vars.lx),
//                 "rv" => vars.num_vec.push(vars.rv),
//                 _ => super::errors::args_error(b, l),
//             },
//             "pop" => {
//                 if vars.num_vec.is_empty() {
//                     super::errors::vec_items(b, l);
//                 }
//                 vars.num_vec.pop();
//             }
//             "len" => {
//                 stack.push(super::errors::parse_float(
//                     vars.num_vec.len().to_string().trim(),
//                     b,
//                     l,
//                 ));
//             }
//             "lx" | "rv" => {
//                 if cmd[3] == ">>" {
//                     let lits: Vec<&str> = statement.split(">>").collect();
//                     let data_vec = lits[1].trim();
//                     let slice = &data_vec[1..data_vec.len() - 1];
//                     let index: usize = ret_index(slice.trim(), vars, b, l);
//                     if index >= vars.num_vec.len() {
//                         super::errors::invalid_index(b, l, index);
//                     }
//                     if cmd[2].trim() == "lx" {
//                         vars.lx = vars.num_vec[index];
//                     } else if cmd[2].trim() == "rv" {
//                         vars.rv = vars.num_vec[index]
//                     } else {
//                         super::errors::args_error(b, l);
//                     }
//                 } else {
//                     super::errors::args_error(b, l);
//                 }
//             }
//             _ => super::errors::args_error(b, l),
//         },
//         _ => super::errors::args_error(b, l),
//     }
// }

// fn ret_index(str_idx: &str, vars: &mut super::super::Vars, b: &str, l: u32) -> usize {
//     if str_idx == "lx" || str_idx == "rv" {
//         if str_idx == "lx" {
//             let index: usize = super::errors::parse_usize(vars.lx.to_string().trim(), b, l);
//             index
//         } else {
//             let index: usize = super::errors::parse_usize(vars.rv.to_string().trim(), b, l);
//             index
//         }
//     } else {
//         let index: usize = super::errors::parse_usize(str_idx, b, l);
//         index
//     }
// }
