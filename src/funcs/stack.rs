use crate::memory::{Memory, self};

use super::errors;

pub fn ram(
    memory: &mut Memory,
    cmd: Vec<&str>,
    statement: &str,
    b: &str,
    l: u32,
) {

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
