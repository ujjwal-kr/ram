use crate::types::Vars;
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
    // ram <var> = <another-var> [casting]

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
                registers.lx = vars.get_int_from_stack(memory)
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
                registers.rv = vars.get_int_from_stack(memory)
            } else {
                // parse cmd[2] as int
                registers.lx = errors::parse_int(cmd[2], b, l)
            }
        }
        "string" => {
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

// pub fn ram(
//     memory: &mut Memory,
//     cmd: Vec<&str>,
//     statement: &str,
//     b: &str,
//     l: u32,
// ) {
//     match cmd[1].trim() {
//         "lx" => {
//             if cmd.len() == 2 {
//                 stack.push(vars.lx)
//             } else if cmd.len() >= 3 {
//                 match cmd[2].trim() {
//                     "prev" => vars.lx = stack[stack.len() - 1],
//                     _ => vars.lx = errors::parse_float(cmd[2], b, l),
//                 }
//             } else {
//                 errors::args_error(b, l);
//             }
//         }
//         "rv" => {
//             if cmd.len() == 2 {
//                 stack.push(vars.rv)
//             } else if cmd.len() >= 3 {
//                 match cmd[2].trim() {
//                     "prev" => vars.rv = stack[stack.len() - 1],
//                     _ => vars.rv = errors::parse_float(cmd[2], b, l),
//                 }
//             } else {
//                 errors::args_error(b, l);
//             }
//         }
//         "string" => {
//             if cmd.len() < 4 {
//                 errors::args_error(b, l);
//             }
//             if cmd[2] != ">>" {
//                 // TODO: error statement
//                 errors::args_error(b, l);
//             }
//             let lits: Vec<&str> = statement.split(">>").collect();
//             vars.string = lits[1].trim().to_string();
//         }

//         "lxstring" => {
//             if cmd.len() < 4 {
//                 errors::args_error(b, l);
//             }
//             if cmd[2] != ">>" {
//                 // TODO: error statement
//                 errors::args_error(b, l);
//             }
//             let lits: Vec<&str> = statement.split(">>").collect();
//             vars.lxstring = lits[1].trim().to_string();
//         }
//         "vec" => {
//             if cmd.len() < 5 {
//                 errors::args_error(b, l)
//             }
//             if cmd[3] != ">>" {
//                 // TODO: error statement
//                 errors::args_error(b, l);
//             }

//             match cmd[2] {
//                 "str" => {
//                     let lits: Vec<&str> = statement.split(">>").collect();
//                     let data_vec: String = lits[1].trim().to_string();
//                     let slice = &data_vec[1..data_vec.len() - 1];
//                     let str_vec: Vec<&str> = slice.split(",").collect();
//                     for item in str_vec {
//                         vars.str_vec.push(item.trim().to_string());
//                     }
//                 }
//                 "int" => {
//                     let lits: Vec<&str> = statement.split(">>").collect();
//                     let str_vec: String = lits[1].trim().to_string();
//                     let slice = &str_vec[1..str_vec.len() - 1];
//                     let data_vec: Vec<&str> = slice.split(",").collect();
//                     for item in data_vec {
//                         vars.num_vec.push(errors::parse_float(item, b, l));
//                     }
//                 }
//                 _ => errors::args_error(b, l),
//             }
//         }

//         "global_var" => {
//             if cmd.len() < 3 {
//                 errors::args_error(b, l);
//             }
//             if cmd.len() == 3 {
//                 match hash_vars.hash_int.get(cmd[2].trim()) {
//                     Some(&value) => stack.push(value),
//                     _ => super::errors::var_error(cmd[2].trim(), b, l),
//                 }
//             } else {
//                 if cmd[3].trim() == "prev" {
//                     hash_vars
//                         .hash_int
//                         .insert(cmd[2].trim().to_string(), stack[stack.len() - 1]);
//                 }
//             }
//         }

//         "var" => {
//             if cmd.len() < 3 {
//                 errors::args_error(b, l);
//             }
//             if cmd.len() == 3 {
//                 match vars.var_int.get(cmd[2].trim()) {
//                     Some(&value) => stack.push(value),
//                     _ => super::errors::var_error(cmd[2].trim(), b, l),
//                 }
//             } else {
//                 if cmd[3].trim() == "prev" {
//                     vars.var_int
//                         .insert(cmd[2].trim().to_string(), stack[stack.len() - 1]);
//                 }
//             }
//         }
//         _ => stack.push(errors::parse_float(cmd[1], b, l)),
//     }
// }

// pub fn pop(stack: &mut Vec<u8>, cmd: Vec<&str>, b: &str, l: u32) {
//     if stack.is_empty() {
//         super::errors::stack_len_error(b, l);
//     }
//     if cmd.len() == 1 {
//         stack.pop();
//     } else if cmd.len() == 2 {
//         let pop_amount: usize = super::errors::parse_usize(cmd[1], b, l);
//         if pop_amount <= stack.len() {
//             for _n in 0..pop_amount {
//                 stack.pop();
//             }
//         } else {
//             super::errors::stack_len_error(b, l);
//         }
//     }
// }

// pub fn strfn(stack: &mut Vec<u8>, vars: &mut super::super::Vars, cmd: Vec<&str>, b: &str, l: u32) {
//     if cmd.len() < 2 {
//         super::errors::args_error(b, l);
//     }
//     match cmd[1].trim() {
//         "cmp" => {
//             if vars.lxstring.trim() == vars.string.trim() {
//                 stack.push(0.0);
//             } else {
//                 stack.push(-1.0);
//             }
//         }
//         "string" => vars.string = vars.lxstring.clone(),
//         "lxstring" => vars.string = vars.lxstring.clone(),
//         _ => errors::args_error(b, l),
//     }
// }
