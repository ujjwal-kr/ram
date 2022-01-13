fn main() {
    let mut stack: Vec<&str> = vec![];
    
    stack.push("push 10");
    stack.push("push 12");

    println!("{:?}", stack);
    read_statement(stack);
}

fn read_statement(program: Vec<&str>) {
    for command in program {
        let cmd = command.split(" ");
        println!("{:?}", cmd);
    }
}
