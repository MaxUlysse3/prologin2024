use std::{
    ops::{AddAssign, SubAssign, MulAssign},
    collections::{HashMap},
};


/// une route bidirectionnelle reliant deux maisons
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Route {
    /// la première maison
    a: i32,
    /// la seconde maison
    b: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct ModInt {
    val: i64,
    modulo: i64,
}

impl ModInt {
    fn new(val: i64, modulo: i64) -> Self {
        Self {
            val,
            modulo,
        }
    }

    fn get_val(&self) -> i32 {
        self.val as i32
    }
}

impl AddAssign<i64> for ModInt {
    fn add_assign(&mut self, mut rhs: i64) {
        while self.modulo - rhs <= self.val {
            rhs -= self.modulo;
        }
        self.val += rhs;
    }
}

impl SubAssign<i64> for ModInt {
    fn sub_assign(&mut self, mut rhs: i64) {
        while self.val < rhs {
            rhs -= self.modulo;
        }
        self.val -= rhs;
    }
}

impl MulAssign<i64> for ModInt {
    fn mul_assign(&mut self, rhs: i64) {
        self.val *= rhs;
        self.val = self.val % self.modulo;
    }
}

/// * `n` - le nombre de maisons
/// * `m` - le nombre de routes
/// * `k` - le temps restant à Joseph, soit la longueur maximale d'un chemin juste
/// * `scores` - le score associé à chaque maison
/// * `routes` - la liste des routes
fn score_total(n: i32, m: i32, k: i32, scores: Vec<i32>, mut routes: Vec<Route>) -> i32 {
    routes.iter_mut().for_each(|Route {a, b}| {
        *a -= 1;
        *b -= 1;
    });
    let mut connections = std::iter::from_fn(|| Some(Vec::new())).take(n as usize).collect::<Vec<Vec<i32>>>();
    for Route {a, b} in &routes {
        connections[*b as usize].push(*a);
        connections[*a as usize].push(*b);
    }
    let mut lens = std::iter::repeat(None).take(n as usize).collect::<Vec<Option<i32>>>();
    // println!("{connections:?}");
    explore(0, &routes, &mut lens, &connections, 0, k, 0);
    // For each element e of lens, if the house at index i is reachable, lens[i] will be Some(l)
    // where l is the length of the path. If the house is unreachable, lens[i] = None
    // println!("{lens:?}");

    // Contains (i, l) where i is a house index and l then length of the path from house 1 for all
    // reachable houses
    let houses = lens.into_iter().enumerate().filter_map(|(i, l)| Some((i, l?))).collect::<Vec<_>>();
    // println!("{houses:?}");

    let mut sums = std::iter::repeat(ModInt::new(0, 1_000_000_007)).take(k as usize + 1).collect::<Vec<_>>();
    houses.iter().for_each(|(i, l)| sums[*l as usize] += scores[*i as usize] as i64);
    // println!("{sums:?}");
    for i in 1..=k as usize {
        let s = sums[i - 1].get_val() as i64;
        sums[i] += s;
    }
    // println!("{sums:?}");

    let mut res = ModInt::new(0, 1_000_000_007);

    houses.iter().for_each(|(i, l)| res += scores[*i as usize] as i64 * sums[k as usize - *l as usize].get_val() as i64);
    res.get_val()
}

fn explore(len: i32, routes: &Vec<Route>, lens: &mut Vec<Option<i32>>, connections: &Vec<Vec<i32>>,
    h: i32, k_max: i32, last_h: i32) {
    // println!("Exploring {h}");
    let connected = &connections[h as usize];
    let r = &mut lens[h as usize];
    if r == &None {
        *r = Some(len);
    }
    match r.as_mut() {
        Some(l) => *l = if *l < len {*l} else {len},
        _ => (),
    }
    if len == k_max {
        return;
    }
    for id in connected {
        // if *id != last_h {
        //     explore(len + 1, routes, lens, connections, *id, k_max, h);
        // }
        match lens[*id as usize].as_mut() {
            Some(other_len) => if *other_len <= len + 1 { continue; },
            None => (),
        }
        explore(len + 1, routes, lens, connections, *id, k_max, h);
    }
}

fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let m = read_line(&mut buffer)
        .parse()
        .expect("invalid `M` parameter");

    let k = read_line(&mut buffer)
        .parse()
        .expect("invalid `K` parameter");

    let scores = read_line(&mut buffer)
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid `scores` parameter");

    let routes = (0..m)
        .map(|_| read_line(&mut buffer).parse())
        .collect::<Result<_, _>>()
        .expect("invalid `routes` parameter");

    println!("{}", score_total(n, m, k, scores, routes));
    
    // let n = 5i32;
    // let m = 5i32;
    // let k = 6i32;
    // let scores = (1..=n).collect::<Vec<i32>>();
    // let mut routes = Vec::<Route>::with_capacity(m as usize);
    // for i in 1..=4 {
    //     routes.push(Route { a: i, b: i + 1} );
    // }
    // routes.push(Route {a: 1, b: 5});
    // let out = score_total(n, m, k, scores, routes);
    // println!("{}", out);
    // let res = 750591879;
    // println!("{res}");
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}

impl std::str::FromStr for Route {
    type Err = Box<dyn std::error::Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line = line.split_whitespace();
        Ok(Self {
            a: line.next().ok_or("missing `a`")?.parse()?,
            b: line.next().ok_or("missing `b`")?.parse()?,
        })
    }
}

