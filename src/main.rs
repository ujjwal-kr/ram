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

    let mut _program: Vec<&str> = contents.split("\n").collect();
    run_statement(&_program);
    println!("{:?}", _program);
    Ok(())
}

fn run_statement(program: &Vec<&str>) {
    let mut stack: Vec<f64> = vec![];

    for statement in program {
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
            }
            _ => { println!("Cant recongize command '{}'", cmd[0]); break }
        }
    }
}
