use crate::{
    funcs::errors::{self, ErrorKind},
    memory::Memory,
    CPU,
};
use rand::Rng;

// rand [1,10]
// rand lx [10,10]
// rand rv [10,20]
// rand lx [rv,lx]

pub fn rand(
    memory: &mut Memory,
    registers: &mut CPU,
    cmd: Vec<&str>,
    statement: &str,
) -> Result<(), ErrorKind> {
    let num1: i32;
    let num2: i32;
    let coma_split: Vec<&str> = statement.split(',').collect();
    let num1_str = coma_split[0].split('[').collect::<Vec<&str>>()[1].trim();
    let num2_str = coma_split[1].split(']').collect::<Vec<&str>>()[0].trim();

    match num1_str {
        "lx" => num1 = registers.lx,
        "rv" => num1 = registers.rv,
        _ => num1 = errors::parse_int(num1_str)?,
    }

    match num2_str {
        "lx" => num2 = registers.lx,
        "rv" => num2 = registers.rv,
        _ => num2 = errors::parse_int(num2_str)?,
    }

    if num1 >= num2 {
        return Err(ErrorKind::RangeNegative);
    }

    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(num1..num2);

    match cmd[1] {
        "lx" => registers.lx = random_num,
        "rv" => registers.rv = random_num,
        _ => memory.set_int_to_stack(random_num),
    }
    Ok(())
}
