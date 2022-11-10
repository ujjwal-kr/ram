use crate::funcs::errors;
use crate::types::{TypeName, Vars};
use crate::{memory::Memory, Registers};
use rand::Rng;

// rand [1,10]
// rand lx [10,10]
// rand rv [10,20]
// rand lx [rv,lx]

pub fn rand(
    memory: &mut Memory,
    registers: &mut Registers,
    cmd: Vec<&str>,
    statement: &str,
    b: &str,
    l: i32,
) {
    let num1: u32;
    let num2: u32;
    let coma_split: Vec<&str> = statement.split(',').collect();
    num1 = coma_split[0].split('[').collect::<Vec<&str>>()[1].trim().parse().expect("ok");
    num2 = coma_split[1].split(']').collect::<Vec<&str>>()[0].trim().parse().expect("ok");
    // todo: implement for lx and rv as ranges

    if num1 >= num2 {
        panic!("Range cannot be negative {}{}", b, l)
    }

    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(num1..num2);

    match cmd[1] {
        "lx" => registers.lx = random_num as i32,
        "rv" => registers.rv = random_num as i32,
        _ => memory.set_int_to_stack(random_num as i32)
    }
}