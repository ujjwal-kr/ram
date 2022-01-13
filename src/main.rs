use std::fs;
use std::io;
use std::io::prelude::*;

fn main() -> std::io::Result<()> 
{
    println!("Enter the filename: ");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;

    let mut file = fs::File::open(filename.trim())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut _program: Vec<&str> = contents.split("\n").collect();

    println!("{:?}", _program);
    run_statement(_program);

    Ok(())
}

fn run_statement(program: Vec<&str>) 
{
    let mut stack: Vec<String> = vec![];

    for statement in program {
        let cmd: Vec<&str> = statement.split(" ").collect();

        match cmd[0] {
            "print" => println!("{}", stack[stack.len() - 1]),
            "push" => stack.push(cmd[1].to_string()),
            "pop" => { stack.pop(); },
            "add" => {
                let result = stack[stack.len() - 1].parse::<i32>().unwrap() + stack[stack.len() - 2].parse::<i32>().unwrap();
                stack.push(result.to_string());
            },
            "sub" => {
                let result = stack[stack.len() - 2].parse::<i32>().unwrap() - stack[stack.len() - 1].parse::<i32>().unwrap();
                stack.push(result.to_string());
            },

            "cmp" => {
                if stack[stack.len() -2].parse::<i32>().unwrap() == stack[stack.len() - 1].parse::<i32>().unwrap()
                {
                    stack.push("0".to_string());
                } else if stack[stack.len() -2].parse::<i32>().unwrap() < stack[stack.len() - 1].parse::<i32>().unwrap()
                    {
                        stack.push("-1".to_string())
                    }
                  else if stack[stack.len() -2].parse::<i32>().unwrap() > stack[stack.len() - 1].parse::<i32>().unwrap()
                    {
                        stack.push("1".to_string())                        
                    }  
            }
            _ => { println!("Cant recongize command '{}'", cmd[0]); break }
        }
    }
}

