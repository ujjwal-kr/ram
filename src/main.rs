use std::fs;
use std::io;
use std::f64;
use std::io::prelude::*;

fn main() -> std::io::Result<()> 
{
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
    run_statement(&blocks, &blocks[0]);
    Ok(())
}

fn run_statement(blocks: &Vec<Vec<&str>>, run_block: &Vec<&str>) {
    let mut stack: Vec<f64> = vec![];
    for statement in run_block {
        let cmd: Vec<&str> = statement.split(" ").collect();

        match cmd[0] {
            "print" => println!("{}", stack[stack.len() - 1]),
            "ram" => stack.push(cmd[1].parse::<f64>().unwrap()),
            "pop" => { stack.pop(); },
            "popall"  => { stack = vec![] }
            "add" => {
                let result = stack[stack.len() - 1] + stack[stack.len() - 2];
                stack.push(result);
            },
            "sub" => {
                let result = stack[stack.len() - 2]- stack[stack.len() - 1];
                stack.push(result);
            },

            "mul" => {
                let result = stack[stack.len() -1] * stack[stack.len() -2];
                stack.push(result);
            }
            
            "div" => {
                let result = stack[stack.len() - 2] / stack[stack.len() -1];
                stack.push(result)
            },

            "sqr" => {
                let result = stack[stack.len() -1] * stack[stack.len() - 1];
                stack.push(result);
            }

            "sqrt" => {
                let result = stack[stack.len() - 1].sqrt();
                stack.push(result);
            }

            "round" => {
                let result = stack[stack.len() - 1].round();
                stack.push(result);
            },
            
            "avg" => {
                let mut total: f64 = 0.0;
                let mut i: f64 = 0.0;
                for num in &stack {
                    i = i + 1.0;
                    total = total + num
                }
                stack.push(total / i)
            },

            "cmp" => {
                if stack[stack.len() - 1] < stack[stack.len() - 2] {
                    stack.push(1.0)
                } else if stack[stack.len() - 1] > stack[stack.len() - 2] {
                    stack.push(-1.0)
                } else {
                    stack.push(0.0)
                }
            }

            "je" => {
                if stack[stack.len() - 1] == 0.0 {
                    let index: usize = cmd[1].parse::<usize>().unwrap();
                    run_statement(blocks, &blocks[index]);
                    stack.pop();
                }
                stack.pop();
            }
            
            "jne" => {
                if stack[stack.len() - 1] != 0.0 {
                    let index: usize = cmd[1].parse::<usize>().unwrap();
                    run_statement(blocks, &blocks[index]);
                    stack.pop();
                }
                stack.pop();
            },

            "jgr" => {
                if stack[stack.len() - 1] == 1.0 {
                    let index: usize = cmd[1].parse::<usize>().unwrap();
                    run_statement(blocks, &blocks[index]);
                    stack.pop();
                }
                stack.pop();
            },

            "jsm" => {
                if stack[stack.len() - 1] == -1.0 {
                    let index: usize = cmd[1].parse::<usize>().unwrap();
                    run_statement(blocks, &blocks[index]);
                    stack.pop();
                }
                stack.pop();
            }

            "jmp" => {
                let index: usize = cmd[1].parse::<usize>().unwrap();
                run_statement(blocks, &blocks[index])
            }
            _ => { println!("Cant recongize command '{}'", cmd[0]); break }
        }
    }
}
