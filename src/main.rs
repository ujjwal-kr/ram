use std::f64;
use std::fs;
use std::io;
use std::io::prelude::*;

mod funcs;
use funcs::{errors, print, stack};

#[derive(Clone)]
pub struct Vars {
    pub lx: f64,
    pub rv: f64,
    pub string: String,
}

fn main() -> std::io::Result<()> {
    println!("Welcome to the Ram stack-based programming language.");
    println!("Enter the filename: ");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;

    let mut file = fs::File::open(filename.trim())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut _program: Vec<&str> = contents.split("\n\n").collect();
    let mut blocks: Vec<Vec<&str>> = vec![];
    for block in &_program {
        let block_vec: Vec<&str> = block.split("\n").collect();
        blocks.push(block_vec);
    }
    let vars = Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
    };
    match run_statement(&blocks, &blocks[0], 0, vars) {
        Ok(()) => (),
        _ => println!("Something went wrong"),
    }
    Ok(())
}

fn run_statement(
    blocks: &Vec<Vec<&str>>,
    run_block: &Vec<&str>,
    block_number: usize,
    vars: Vars,
) -> std::io::Result<()> {
    let mut line = 0u32;
    let mut local_vars = Vars {
        lx: vars.lx,
        rv: vars.rv,
        string: vars.string,
    };
    let mut stack: Vec<f64> = vec![];
    for statement in run_block {
        line = line + 1;
        let mut cmd: Vec<&str> = statement.split(" ").collect();
        // comments
        let lits: Vec<&str> = cmd[0].trim().split("").collect();
        if lits[1] == "/" && lits[2] == "/" {
            cmd[0] = "//";
        }
        match cmd[0] {
            "//" => (),
            "print" => print::print(&mut stack, cmd, &mut local_vars, block_number, line),
            "printc" => print::printc(cmd, statement, block_number, line),
            "ram" => stack::ram(
                &mut stack,
                cmd,
                statement,
                &mut local_vars,
                block_number,
                line,
            ),
            "stdin" => stack::stdin(&mut local_vars, cmd, block_number, line),
            "pop" => stack::pop(&mut stack, block_number, line),
            "popall" => stack = vec![],
            "add" => add(&mut stack, cmd, &mut local_vars, block_number, line),
            "sub" => sub(&mut stack, block_number, line),
            "mul" => mul(&mut stack, cmd, &mut local_vars, block_number, line),
            "div" => div(&mut stack, block_number, line),
            "sqr" => sqr(&mut stack, cmd, &mut local_vars, block_number, line),
            "sqrt" => sqrt(&mut stack, cmd, &mut local_vars, block_number, line),
            "round" => round(&mut stack, cmd, &mut local_vars, block_number, line),
            "avg" => avg(&mut stack, block_number, line),
            "rand" => stack::random(&mut stack, statement, block_number, line),
            "cmp" => cmp(&mut stack, block_number, line),

            "je" => {
                if cmd.len() != 2 {
                    errors::args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] == 0.0 {
                    let index: usize = errors::parse_usize(cmd[1], block_number, line);
                    if blocks.len() <= index {
                        errors::invalid_jmp(block_number, line, index);
                        break;
                    }
                    match run_statement(blocks, &blocks[index], index, local_vars.clone()) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jne" => {
                if cmd.len() != 2 {
                    errors::args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] != 0.0 {
                    let index: usize = errors::parse_usize(cmd[1], block_number, line);
                    if blocks.len() <= index {
                        errors::invalid_jmp(block_number, line, index);
                        break;
                    }
                    match run_statement(blocks, &blocks[index], index, local_vars.clone()) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jgr" => {
                if cmd.len() != 2 {
                    errors::args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] == 1.0 {
                    let index: usize = errors::parse_usize(cmd[1], block_number, line);
                    if blocks.len() <= index {
                        errors::invalid_jmp(block_number, line, index);
                        break;
                    }
                    match run_statement(blocks, &blocks[index], index, local_vars.clone()) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jsm" => {
                if cmd.len() != 2 {
                    errors::args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] == -1.0 {
                    let index: usize = errors::parse_usize(cmd[1], block_number, line);
                    if blocks.len() <= index {
                        errors::invalid_jmp(block_number, line, index)
                    }
                    match run_statement(blocks, &blocks[index], index, local_vars.clone()) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jmp" => {
                if cmd.len() != 2 {
                    errors::args_error(block_number, line);
                    break;
                }
                let index: usize = errors::parse_usize(cmd[1], block_number, line);
                if blocks.len() <= index {
                    errors::invalid_jmp(block_number, line, index);
                    break;
                }
                match run_statement(blocks, &blocks[index], index, local_vars.clone()) {
                    Ok(()) => (),
                    _ => println!("Something went wrong"),
                }
            }
            "halt" => {
                panic!("Program End");
            }
            _ => {
                println!(
                    "Cant recognize command '{}' at b{}:l{}",
                    cmd[0],
                    block_number.to_string(),
                    line.to_string()
                );
                break;
            }
        }
    }
    Ok(())
}

fn add(stack: &mut Vec<f64>, cmd: Vec<&str>, vars: &mut Vars, b: usize, l: u32) {
    if cmd.len() > 1 {
        stack.push(vars.lx + vars.rv);
    } else {
        if stack.len() < 2 {
            errors::stack_len_error(b, l);
        } else {
            let result = stack[stack.len() - 1] + stack[stack.len() - 2];
            stack.push(result);
        }
    }
}

fn sub(stack: &mut Vec<f64>, b: usize, l: u32) {
    if stack.len() < 2 {
        errors::stack_len_error(b, l);
    }
    let result = stack[stack.len() - 2] - stack[stack.len() - 1];
    stack.push(result);
}

fn mul(stack: &mut Vec<f64>, cmd: Vec<&str>, vars: &mut Vars, b: usize, l: u32) {
    if cmd.len() > 1 {
        stack.push(vars.lx * vars.rv);
    } else {
        if stack.len() < 2 {
            errors::stack_len_error(b, l);
        }
        let result = stack[stack.len() - 1] * stack[stack.len() - 2];
        stack.push(result);
    }
}

fn div(stack: &mut Vec<f64>, b: usize, l: u32) {
    if stack.len() < 2 {
        errors::stack_len_error(b, l);
    }
    let result = stack[stack.len() - 2] / stack[stack.len() - 1];
    stack.push(result)
}

fn sqr(stack: &mut Vec<f64>, cmd: Vec<&str>, vars: &mut Vars, b: usize, l: u32) {
    if cmd[1] == "lx" || cmd[1] == "rv" {
        if cmd[1] == "lx" {
            vars.lx = vars.lx * vars.lx;
        }
        if cmd[1] == "rv" {
            vars.rv = vars.rv * vars.rv;
        }
    } else {
        if stack.len() < 1 {
            errors::stack_len_error(b, l);
        }
        let result = stack[stack.len() - 1] * stack[stack.len() - 1];
        stack.push(result);
    }
}

fn sqrt(stack: &mut Vec<f64>, cmd: Vec<&str>, vars: &mut Vars, b: usize, l: u32) {
    if cmd[1] == "lx" || cmd[1] == "rv" {
        if cmd[1] == "lx" {
            vars.lx = vars.lx.sqrt();
        }
        if cmd[1] == "rv" {
            vars.rv = vars.rv.sqrt();
        }
    } else {
        let result = stack[stack.len() - 1].sqrt();
        stack.push(result);
    }
    if stack.len() < 1 {
        errors::stack_len_error(b, l);
    }
    let result = stack[stack.len() - 1].sqrt();
    stack.push(result);
}

fn round(stack: &mut Vec<f64>, cmd: Vec<&str>, vars: &mut Vars, b: usize, l: u32) {
    if cmd.len() > 1 {
        if cmd[1] == "lx" || cmd[1] == "rv" {
            if cmd[1] == "lx" {
                vars.lx = vars.lx.round();
            }
            if cmd[1] == "rv" {
                vars.rv = vars.rv.round();
            }
        }
    } else {
        if stack.len() < 1 {
            errors::stack_len_error(b, l);
        }
        let result = stack[stack.len() - 1].round();
        stack.push(result);
    }
}

fn avg(stack: &mut Vec<f64>, b: usize, l: u32) {
    if stack.len() < 1 {
        errors::stack_len_error(b, l);
    }
    let mut total: f64 = 0.0;
    let mut i: f64 = 0.0;
    for num in stack.clone() {
        i = i + 1.0;
        total = total + num
    }
    stack.push(total / i)
}

fn cmp(stack: &mut Vec<f64>, b: usize, l: u32) {
    if stack.len() < 2 {
        errors::stack_len_error(b, l);
    }
    if stack[stack.len() - 1] < stack[stack.len() - 2] {
        stack.push(1.0)
    } else if stack[stack.len() - 1] > stack[stack.len() - 2] {
        stack.push(-1.0)
    } else {
        stack.push(0.0)
    }
}
