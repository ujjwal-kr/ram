use std::fs;
use std::io;
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

fn run_statement(program: &Vec<&str>) 
{
    let mut stack: Vec<i32> = vec![];

    for statement in program {
        let cmd: Vec<&str> = statement.split(" ").collect();

        match cmd[0] {
            "print" => println!("{}", stack[stack.len() - 1]),
            "push" => stack.push(cmd[1].parse::<i32>().unwrap()),
            "pop" => { stack.pop(); },
            "add" => {
                let result = stack[stack.len() - 1] + stack[stack.len() - 2];
                stack.push(result);
            },
            "sub" => {
                let result = stack[stack.len() - 2]- stack[stack.len() - 1];
                stack.push(result);
            },

            "cmp" => {
                if stack[stack.len() -2] == stack[stack.len() - 1]
                {
                    stack.push(0);
                } else if stack[stack.len() -2] < stack[stack.len() - 1]
                    {
                        stack.push(-1)
                    }
                  else if stack[stack.len() -2] > stack[stack.len() - 1]
                    {
                        stack.push(1)                        
                    }  
            },
            _ => { println!("Cant recongize command '{}'", cmd[0]); break }
        }
    }
}

