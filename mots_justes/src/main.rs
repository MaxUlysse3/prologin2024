use std::{
    collections::HashMap,
};

fn main() {
    let (n, a, b) = parse_input();

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

    let correcs = find_corrections(&count);
    // println!("{correcs:?}");

    let correction_maj = test(correcs.clone());
    let mut res = 0;
    for (key, c) in count.iter() {
        // println!("{key}");
        if *c < correction_maj {
            let mut tested = count.clone();
            tested.remove(key);
            if dbg!(test(find_corrections(&tested))) >= correction_maj {
                res += correcs[key];
            } else {
                res += c;
            }
        } else {
            res += correcs[key];
        }
    } 
    println!("{res}");

}

fn test(count: HashMap<String, usize>) -> usize {
    count.values().sum()
}

fn find_corrections(count: &HashMap<String, usize>) -> HashMap<String, usize> {
    // println!("{count:?}");
    let array = &mut (count.values().collect::<Vec<_>>());
    array.sort();
    let target = *array[count.len() / 2] as i32;

    // println!("{target}");

    let mut corecs = HashMap::<String, usize>::new();

    count.iter().for_each(|(k, v)| {
        let dif = (*v as i32 - target).abs() as usize;
        if corecs.contains_key(k) {
            *corecs.get_mut(k).unwrap() += dif;
        } else {
            corecs.insert(k.to_owned(), dif);
        }
    });

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
