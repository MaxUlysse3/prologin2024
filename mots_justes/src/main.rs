use std::{
    collections::HashMap,
};

fn main() {
    let (n, a, b) = parse_input();
    alg2(n, a, b);
    // alg1ex(n, a, b);
    // println!("{}", alg1ex(n, a, b));
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

fn alg2(n: i32, a: Vec<String>, b: Vec<String>) {
    let mut count = HashMap::<String, usize>::new();
    for sa in &a {
        for sb in &b {
            let word = format!("{}{}", sa, sb).to_string();
            match count.get_mut(&word) {
                Some(r) => *r += 1,
                None => { count.insert(word, 1).unwrap(); },
            }
        }
    }

    let mut count_zones = HashMap::<usize, usize>::new();
    for (_, v) in count.iter() {
        match count_zones.get_mut(v) {
            Some(r) => *r += 1,
            None => { count_zones.insert(*v, 1).unwrap(); },
        }
    }

    let mut sum_zones = vec![];
    for i in 0..count_zones.len() {
        
    }
    
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

    println!("target = {} : {}", target, corecs.values().sum::<usize>());

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
        const N: usize = 3;

        let letters = (0..N).map(|x| x.to_string()).collect::<Vec<_>>();
        let l1 = vec![];
        let l2 = vec![];

        for _ in 0..N {
            l1.push(letters[rand::thread_rng().gen_range(0..N)]);
            l2.push(letters[rand::thread_rng().gen_range(0..N)]);
        }

        assert_eq!(alg1ex(N, l1, l2), alg2(N, l1, l2));

    }
}
