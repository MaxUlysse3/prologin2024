use std::collections::HashMap;

fn main() {
    let (n, m, crits) = parse_input();
    let mut map = HashMap::<String, bool>::new();
    for cr in crits {
        if !map.contains_key(&cr) {
            map.insert(cr, true);
        } else {
            let v = map.get_mut(&cr).unwrap();
            *v = !*v;
        }
    }
    let iter = map.into_iter().map(|(_, v)| if v {1} else {0});
    let s: i32 = iter.sum();

    println!("{}", s);
}

fn parse_input() -> (i32, i32, Vec<String>) {
    let stdin = std::io::stdin();

    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    stdin.read_line(&mut input).unwrap();
    
    let iter = &mut input.trim().split('\n');
    let n = iter.next().unwrap().parse::<i32>().unwrap();
    let m = iter.next().unwrap().parse::<i32>().unwrap();

    let mut crits = vec![];

    for _ in 0..n {
        input = String::new();

        stdin.read_line(&mut input).unwrap();

        crits.push(input.trim().to_owned());
    }

    (n, m, crits)

}
