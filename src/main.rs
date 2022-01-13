fn main() {
    let mut program: Vec<&str> = vec![];

    program.push("push 1000");
    program.push("print");

    println!("{:?}", program);
    read_statement(program);
}

fn read_statement(program: Vec<&str>) {
    let mut stack: Vec<&str> = vec![];

    for statement in program {
        let cmd: Vec<&str> = statement.split(" ").collect();

        match cmd[0] {
            "print" => println!("{}", stack[stack.len() - 1]),
            "push" => stack.push(cmd[1]),
            "pop" => { stack.pop(); },
            "add" => {
                let result = stack[stack.len()].parse::<i32>().unwrap() + stack[stack.len() - 1].parse::<i32>().unwrap();
                let result_str = result.to_string();
                stack.push(result_str);
            },
            _ => println!("Program End")
        }
    }
}

