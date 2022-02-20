use super::super::errors;
use super::super::Vars;

pub fn var<'a>(cmd: Vec<&'a str>, statement: &'a str, vars: &mut Vars<'a>, b: usize, l: u32) {
    if cmd.len() < 5 {
        errors::args_error(b, l);
    }
    let lits: Vec<&str> = statement.split(">>").collect();
    let value: &str = lits[1].trim();

    if cmd[2] == "str" {
        vars.hash_str.insert(cmd[1].trim(), value);
    } else if cmd[2] == "int" {
        vars.hash_int
            .insert(cmd[1].trim(), errors::parse_float(value, b, l));
    } else {
        errors::args_error(b, l);
    }
}
