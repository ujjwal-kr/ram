use rand::Rng;
use std::f64;
use std::fs;
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct Vars {
    lx: f64,
    rv: f64,
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
    let vars = Vars { lx: 0.0, rv: 0.0 };
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
            "print" => {
                if cmd.len() == 1 {
                    if stack.len() < 1 {
                        stack_len_error(block_number, line);
                        break;
                    }
                    println!("{}", stack[stack.len() - 1]);
                } else {
                    if cmd[1] == "lx" {
                        println!("{}", local_vars.lx)
                    }
                    if cmd[1] == "rv" {
                        println!("{}", local_vars.rv)
                    }
                }
            }
            "printc" => {
                if cmd.len() < 3 {
                    args_error(block_number, line);
                    break;
                }
                let prntc_cmd: Vec<&str> = statement.split(">>").collect();
                println!("{}", prntc_cmd[1].trim());
            }
            "ram" => {
                if cmd[1] == "lx" || cmd[1] == "rv" {
                    if cmd.len() == 2 {
                        if cmd[1] == "lx" {
                            stack.push(local_vars.lx)
                        }
                        if cmd[1] == "rv" {
                            stack.push(local_vars.rv)
                        }
                    } else {
                        if cmd[1] == "lx" {
                            if cmd[2] == "prev" {
                                local_vars.lx = stack[stack.len() - 1];
                            } else {
                                local_vars.lx = parse_float(cmd[2], block_number, line)
                            }
                        }
                        if cmd[1] == "rv" {
                            if cmd[2] == "prev" {
                                local_vars.rv = stack[stack.len() - 1];
                            } else {
                                local_vars.rv = parse_float(cmd[2], block_number, line)
                            }
                        }
                    }
                } else {
                    stack.push(parse_float(cmd[1], block_number, line))
                }
            }
            "stdin" => {
                if cmd.len() != 2 {
                    args_error(block_number, line);
                    break;
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let number: f64 = parse_float(input.trim(), block_number, line);
                if cmd[1] == "lx" {
                    local_vars.lx = number
                }
                if cmd[1] == "rv" {
                    local_vars.rv = number
                }
            }
            "pop" => {
                if stack.len() < 1 {
                    stack_len_error(block_number, line);
                    break;
                }
                stack.pop();
            }
            "popall" => stack = vec![],
            "add" => {
                if cmd.len() > 1 {
                    stack.push(local_vars.lx + local_vars.rv);
                } else {
                    if stack.len() < 2 {
                        stack_len_error(block_number, line);
                        break;
                    }
                    let result = stack[stack.len() - 1] + stack[stack.len() - 2];
                    stack.push(result);
                }
            }
            "sub" => {
                if stack.len() < 2 {
                    stack_len_error(block_number, line);
                    break;
                }
                let result = stack[stack.len() - 2] - stack[stack.len() - 1];
                stack.push(result);
            }

            "mul" => {
                if cmd.len() > 1 {
                    stack.push(local_vars.lx * local_vars.rv);
                } else {
                    if stack.len() < 2 {
                        stack_len_error(block_number, line);
                        break;
                    }
                    let result = stack[stack.len() - 1] * stack[stack.len() - 2];
                    stack.push(result);
                }
            }

            "div" => {
                if stack.len() < 2 {
                    stack_len_error(block_number, line);
                    break;
                }
                let result = stack[stack.len() - 2] / stack[stack.len() - 1];
                stack.push(result)
            }

            "sqr" => {
                if cmd[1] == "lx" || cmd[1] == "rv" {
                    if cmd[1] == "lx" {
                        local_vars.lx = local_vars.lx * local_vars.lx;
                    }
                    if cmd[1] == "rv" {
                        local_vars.rv = local_vars.rv * local_vars.rv;
                    }
                } else {
                    if stack.len() < 1 {
                        stack_len_error(block_number, line);
                        break;
                    }
                    let result = stack[stack.len() - 1] * stack[stack.len() - 1];
                    stack.push(result);
                }
            }

            "sqrt" => {
                if cmd[1] == "lx" || cmd[1] == "rv" {
                    if cmd[1] == "lx" {
                        local_vars.lx = local_vars.lx.sqrt();
                    }
                    if cmd[1] == "rv" {
                        local_vars.rv = local_vars.rv.sqrt();
                    }
                } else {
                    let result = stack[stack.len() - 1].sqrt();
                    stack.push(result);
                }
                if stack.len() < 1 {
                    stack_len_error(block_number, line);
                    break;
                }
                let result = stack[stack.len() - 1].sqrt();
                stack.push(result);
            }

            "round" => {
                if cmd.len() > 1 {
                    if cmd[1] == "lx" || cmd[1] == "rv" {
                        if cmd[1] == "lx" {
                            local_vars.lx = local_vars.lx.round();
                        }
                        if cmd[1] == "rv" {
                            local_vars.rv = local_vars.rv.round();
                        }
                    }
                } else {
                    if stack.len() < 1 {
                        stack_len_error(block_number, line);
                        break;
                    }
                    let result = stack[stack.len() - 1].round();
                    stack.push(result);
                }
            }

            "avg" => {
                if stack.len() < 1 {
                    stack_len_error(block_number, line);
                    break;
                }
                let mut total: f64 = 0.0;
                let mut i: f64 = 0.0;
                for num in &stack {
                    i = i + 1.0;
                    total = total + num
                }
                stack.push(total / i)
            }

            "rand" => {
                let rand_cmd: Vec<&str> = statement.split(">>").collect();
                let numbers: Vec<&str> = rand_cmd[1].split(",").collect();

                let mut rng = rand::thread_rng();
                let random = rng.gen_range(
                    parse_float(numbers[0].trim(), block_number, line)
                        ..parse_float(numbers[1].trim(), block_number, line),
                );
                stack.push(random);
            }

            "cmp" => {
                if stack.len() < 2 {
                    stack_len_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] < stack[stack.len() - 2] {
                    stack.push(1.0)
                } else if stack[stack.len() - 1] > stack[stack.len() - 2] {
                    stack.push(-1.0)
                } else {
                    stack.push(0.0)
                }
            }

            "je" => {
                if cmd.len() != 2 {
                    args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] == 0.0 {
                    let index: usize = parse_usize(cmd[1], block_number, line);
                    if blocks.len() <= index {
                        invalid_jmp(block_number, line, index);
                        break;
                    }
                    match run_statement(blocks, &blocks[index], index, local_vars) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jne" => {
                if cmd.len() != 2 {
                    args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] != 0.0 {
                    let index: usize = parse_usize(cmd[1], block_number, line);
                    if blocks.len() <= index {
                        invalid_jmp(block_number, line, index);
                        break;
                    }
                    match run_statement(blocks, &blocks[index], index, local_vars) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jgr" => {
                if cmd.len() != 2 {
                    args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] == 1.0 {
                    let index: usize = parse_usize(cmd[1], block_number, line);
                    if blocks.len() <= index {
                        invalid_jmp(block_number, line, index);
                        break;
                    }
                    match run_statement(blocks, &blocks[index], index, local_vars) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jsm" => {
                if cmd.len() != 2 {
                    args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] == -1.0 {
                    let index: usize = parse_usize(cmd[1], block_number, line);
                    if blocks.len() <= index {
                        invalid_jmp(block_number, line, index)
                    }
                    match run_statement(blocks, &blocks[index], index, local_vars) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jmp" => {
                if cmd.len() != 2 {
                    args_error(block_number, line);
                    break;
                }
                let index: usize = parse_usize(cmd[1], block_number, line);
                if blocks.len() <= index {
                    invalid_jmp(block_number, line, index);
                    break;
                }
                match run_statement(blocks, &blocks[index], index, local_vars) {
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

fn parse_float(arg: &str, block: usize, line: u32) -> f64 {
    let num: f64;
    num = arg.parse().expect(
        format!(
            "Input a number at block:{}:line{}",
            block.to_string(),
            line.to_string()
        )
        .trim(),
    );
    num
}

fn parse_usize(arg: &str, block: usize, line: u32) -> usize {
    let num: usize;
    num = arg.parse().expect(
        format!(
            "Invalid jmp statement, block {} is invalid block:{}::line:{}",
            arg,
            block.to_string(),
            line.to_string()
        )
        .trim(),
    );
    num
}

fn stack_len_error(block: usize, line: u32) {
    println!(
        "Not enough items in the stack, block:{}::line:{}",
        block.to_string(),
        line.to_string()
    );
}

fn args_error(block: usize, line: u32) {
    println!(
        "Invalid arguments at block:{}::line:{}",
        block.to_string(),
        line.to_string()
    );
}

fn invalid_jmp(block: usize, line: u32, inv_block: usize) {
    println!(
        "Invalid jmp arguments at block:{}::line:{}, block {} does not exist",
        block.to_string(),
        line.to_string(),
        inv_block.to_string()
    );
}
