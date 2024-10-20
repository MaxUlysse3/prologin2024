fn main() {
    let (len, tab) = parse_input();

    let mut max_unsafe = 0;
    let mut unsafe_count = 0;

    let mut min_safe = len;
    let mut safe_count = 0;

    for s in tab {
        if s == 0 {
            if max_unsafe < unsafe_count {
                max_unsafe = unsafe_count;
            }
            unsafe_count = 0;
            safe_count += 1;
        } else {
            if min_safe > safe_count && safe_count != 0 {
                min_safe = safe_count;
            }
            safe_count = 0;
            unsafe_count += 1;
        }
        // println!("unsafe: {:?} / {:?}, safe: {:?} / {:?}", max_unsafe, unsafe_count, min_safe, safe_count);
    }

    if max_unsafe < unsafe_count {
        max_unsafe = unsafe_count;
    }

    if min_safe > safe_count && safe_count != 0 {
        min_safe = safe_count;
    }

    // println!("unsafe: {:?} / {:?}, safe: {:?} / {:?}", max_unsafe, unsafe_count, min_safe, safe_count);

    println!("{}", min_safe - max_unsafe);
}

fn parse_input() -> (i32, Vec<i32>) {
    let stdin = std::io::stdin();

    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    stdin.read_line(&mut input).unwrap();

    let iter = &mut input.trim().split('\n');

    let len = iter.next().unwrap().parse::<i32>().unwrap();

    let tab = iter.next().unwrap().split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

    (len, tab)
}
