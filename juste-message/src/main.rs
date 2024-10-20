use std::{
    io,
};

fn main() {
    let (len, msg) = parse_input();

    let mut res = String::new();
    let mut last_num = 0;
    for c in msg.chars() {
        match &c.to_string().parse::<i32>() {
            Err(_) => res.push(c),
            Ok(n) => {
                if *n >= last_num {
                    res.extend(n.to_owned().to_string().chars());
                }
                last_num = *n;
            },
        }
    }

    println!("{}", res);
}

fn parse_input() -> (i32, String) {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    stdin.read_line(&mut input).unwrap();
    
    let mut iter = input.trim().split('\n');
    let len: i32 = iter.next().unwrap().parse().unwrap();
    let msg = iter.next().unwrap();
    (len, msg.to_string())
}
