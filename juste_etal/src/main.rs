use std::{
    ops::{SubAssign, AddAssign},
    collections::HashMap,
};

fn main() {
    let (n, val, mut boxes) = parse_input();

    // let mut boxes = [0].into_iter().cycle().take(100000000).collect::<Vec<_>>();
    // let n = boxes.len();
    // let val = 18;

    let mut sum = ModInt::new(0, val);
    for b in boxes.iter_mut() {
        sum += *b;
        *b = sum.get_val()
    }
    // println!("{boxes:?}");

    let mut counter = HashMap::<i32, i32>::new();
    counter.insert(0, 1);
    for s in boxes.into_iter() {
        match counter.get_mut(&s) {
            Some(r) => *r += 1,
            None => {
                counter.insert(s, 1);
                ()
            },
        }
    }

    let mut res = ModInt::new(0, 1_000_000_007);
    for n in counter.values() {
        for c in 0..*n {
            res += c;
        }
    }

    println!("{}", res.get_val());

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModInt {
    val: i32,
    module: i32,
}

impl ModInt {
    pub fn new(val: i32, module: i32) -> Self {
        Self {
            val,
            module,
        }
    }

    #[inline]
    pub fn get_val(&self) -> i32 {
        self.val
    }
}

impl AddAssign<i32> for ModInt {
    fn add_assign(&mut self, rhs: i32) {
        while self.module - self.val <= rhs {
            self.val -= self.module;
        }
        self.val += rhs;
    }
}

impl SubAssign<i32> for ModInt {
    fn sub_assign(&mut self, rhs: i32) {
        while self.val <= rhs {
            self.val += self.module;
        }
        self.val -= rhs;
    }
}

fn parse_input() -> (usize, i32, Vec<i32>) {
    let stdin = std::io::stdin();

    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    stdin.read_line(&mut input).unwrap();
    stdin.read_line(&mut input).unwrap();

    let iter = &mut input.trim().split('\n');

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let val = iter.next().unwrap().parse::<i32>().unwrap();

    let boxes = iter.next().unwrap().split(' ').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    (n, val, boxes)
}
