use std::{collections::HashMap, process};

#[derive(Debug)]
pub enum ErrorKind {
    ParseInt,
    ArgErr,
    StackLen,
    VarNotFound(String),
    Casting { src: String, dest: String },
    ExpectedInt(String),
    ExpectedStr(String),
    ExpectedVec(String),
    RangeNegative,
    EmptyCallstack,
    LabelNotFound(String),
    VecLen(String),
}

pub fn parse_int(value: &str) -> Result<i32, ErrorKind> {
    let num: i32;

    match value.parse::<i32>() {
        Ok(n) => num = n,

        _parse_int_errorr => return Err(ErrorKind::ParseInt),
    }

    Ok(num)
}

pub fn get_label(pc: u32, map: HashMap<String, usize>) -> String {
    let mut value_point_vec: Vec<usize> = vec![];
    let mut key_vec: Vec<String> = vec![];
    for (k, v) in map.clone().into_iter() {
        value_point_vec.push(v);
        key_vec.push(k);
    }
    value_point_vec.sort();
    let final_point: usize;
    let mut final_label = String::from("");
    let mut point_stack: Vec<usize> = vec![];
    for point in value_point_vec {
        if pc >= point as u32 {
            point_stack.push(point)
        }
    }
    final_point = point_stack[point_stack.len() - 1];
    for k in key_vec {
        match map.get(&k) {
            Some(&n) => {
                if n == final_point {
                    final_label = k;
                    break;
                }
            }
            None => panic!("nani?"),
        }
    }
    final_label
}
