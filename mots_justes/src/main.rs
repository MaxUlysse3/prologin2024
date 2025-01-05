use std::{
    collections::{HashMap, HashSet},
};

fn main() {
    let (n, a, b) = parse_input();
    // alg2(n, a, b);
    // alg1ex(n, a, b);
    println!("{}", alg2(n, a, b));
}

fn alg1ex(n: i32, a: Vec<String>, b: Vec<String>) -> usize {
    let mut count = HashMap::<String, usize>::new();

    for syla in &a {
        for sylb in &b {
            let word = format!("{}{}", syla, sylb).to_string();
            if count.contains_key(&word) {
                *count.get_mut(&word).unwrap() += 1;
            } else {
                count.insert(word, 1);
            }
            // println!("{syla}{sylb}");
        }
    }

    // println!("{count:?}");
    let mut nums = count.values();

    let res = nums.map(|c| {
        find_corrections(&count, *c as i32).values().sum::<usize>()
    }).min().unwrap();

    println!("{:?}", res);

    res

}

fn alg2(n: i32, a: Vec<String>, b: Vec<String>) -> usize {
    // println!("\n\n\n ALG2");
    let mut count = HashMap::<String, usize>::new();
    for sa in &a {
        for sb in &b {
            let word = format!("{}{}", sa, sb).to_string();
            match count.get_mut(&word) {
                Some(r) => *r += 1,
                None => { count.insert(word, 1); },
            }
        }
    }

    let mut num_of_zones = HashMap::<usize, usize>::new();
    for c in count.values() {
        *num_of_zones.entry(*c).or_insert(0) += 1;
    }
    // let mut counted = count.iter().map(|(_, v)| *v).collect::<Vec<_>>();
    let mut counted = count.values().map(|v| *v).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
    counted.sort();
    // println!("{counted:?}");

    let mut targs = vec![];
    let mid = counted.len() / 2;

    targs.push(counted[mid]);
    let size_target = 20;
    for i in 1..=size_target {
        if mid + i < counted.len() {
            targs.push(counted[mid + i]);
        }
        if mid >= i {
            targs.push(counted[mid - i]);
        }
    }

    let mut res = vec![];
    // println!("targs: {targs:?}");

    for target in targs {
        let r = find_corrections(&count, target as i32).values().sum::<usize>();
        res.push(r);
    }
    // println!("res : {res:?}");
    let res = *res.iter().min().unwrap();
    // println!("res algbest : {res}");
    res
}

fn test(count: HashMap<String, usize>) -> usize {
    count.values().sum()
}

fn find_corrections(count: &HashMap<String, usize>, target: i32) -> HashMap<String, usize> {
    let mut corecs = HashMap::<String, usize>::new();

    count.iter().for_each(|(k, v)| {
        let mut dif = (*v as i32 - target).abs() as usize;
        dif = std::cmp::min(*v, dif);
        if corecs.contains_key(k) {
            *corecs.get_mut(k).unwrap() += dif;
        } else {
            corecs.insert(k.to_owned(), dif);
        }
    });

    // println!("target = {} : {}", target, corecs.values().sum::<usize>());

    corecs
}

fn parse_input() -> (i32, Vec<String>, Vec<String>) {
    let stdin = std::io::stdin();

    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();

    let n = input.trim().parse::<i32>().unwrap();
    
    input = String::new();
    for _ in 0..n {
        stdin.read_line(&mut input).unwrap();
    }
    let a = input.trim().split('\n').map(|x| x.to_string()).collect::<Vec<_>>();

    input = String::new();
    for _ in 0..n {
        stdin.read_line(&mut input).unwrap();
    }
    let b = input.trim().split('\n').map(|x| x.to_string()).collect::<Vec<_>>();
    (n, a, b)
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_alg() {
        const N: usize = 12;
        const K: usize = 100;

        let letters = (0..N).map(|x| x.to_string()).collect::<Vec<_>>();
        for c in 0..K {
            println!("---------   num test : {c}, N = {N}");
            let mut l1 = vec![];
            let mut l2 = vec![];

            for _ in 0..N {
                l1.push(letters[rand::thread_rng().gen_range(0..N)].clone());
                l2.push(letters[rand::thread_rng().gen_range(0..N)].clone());
            }

            assert_eq!(alg1ex(N as i32, l1.clone(), l2.clone()), alg2(N as i32, l1, l2));
        }
    }
}
