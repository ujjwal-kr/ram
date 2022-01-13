fn main() {
    let mut program: Vec<&str> = vec![];

    program.push("push 1000");
    program.push("push 500");
    program.push("add");
    program.push("print");

    println!("{:?}", program);
    read_statement(program);
}

fn read_statement(program: Vec<&str>) {
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
            _ => println!("Program End")
        }
    }
}

