pub fn print(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    vars: &mut super::super::Vars,
    b: usize,
    l: u32,
) {
    if cmd.len() == 1 {
        if stack.len() < 1 {
            super::errors::stack_len_error(b, l);
        } else {
            println!("{}", stack[stack.len() - 1]);
        }
    } else {
        if cmd[1] == "lx" {
            println!("{}", vars.lx)
        }
        if cmd[1] == "rv" {
            println!("{}", vars.rv)
        }
        if cmd[1] == "string" {
            println!("{}", vars.string.trim());
        }
        if cmd[1] == "lxstring" {
            println!("{}", vars.lxstring.trim());
        }
        if cmd[1] == "vec" {
            if cmd[2] == "int" {
                println!("{:?}", vars.num_vec);
            } else if cmd[2] == "str" {
                println!("{:?}", vars.str_vec);
            }
        }
    }
}

pub fn printc(cmd: Vec<&str>, statement: &str, b: usize, l: u32) {
    if cmd.len() < 3 {
        super::errors::args_error(b, l);
    } else {
        let prntc_cmd: Vec<&str> = statement.split(">>").collect();
        println!("{}", prntc_cmd[1].trim());
    }
}
