// #![feature(test)]

// extern crate test;

use std::{
    ops::{SubAssign, AddAssign},
    // iter::repeat_n,
};

// use test::Bencher;

fn main() {
    first_try();
}

fn first_try() {
    let (n, val, boxes) = parse_input();

    // let boxes = repeat_n(1, 1000000).collect::<Vec<_>>();
    // let n = boxes.len();
    // let val = 100;

    let mut res = ModInt::new(0, 1_000_000_007);

    let mut test_0 = true;
    let mut test_1 = true;
    for i in &boxes {
        if *i != 0 {
            test_0 = false;
        } 
        if *i != 1 {
            test_1 = false;
        }
    }
    if test_0 {
        for i in 0..=n {
            res += i as i32;
        }
    } else if test_1 {
        // println!("{}", n as i32 / val);
        for fact in 1..=(n as i32 / val) {
            res += n as i32 - (fact * val) + 1;
            // for _ in 0..=(n as i32 - (fact * val)) {
            //     res += 1;
            // }
        }
    } else {
        for i in 0usize..(n) {
            let mut module = ModInt::new(0, val);
            for j in i..n {
                module += boxes[j];
                if module.get_val() == 0 {
                    res += 1;
                }
            }
        }
    }

    // println!("{:?}", res);

    println!("{}", res.get_val());
}

fn alternative() {
    let (n, val, boxes) = parse_input();

    // let boxes = repeat_n(0, 10000).collect::<Vec<_>>();
    // let n = boxes.len();
    // let val = 1_000_000_000;

    let mut res = ModInt::new(0, 1_000_000_007);

    let mut first = ModInt::new(0, val);
    for size in 1usize..=n {
        first += boxes[size-1];

        let mut module = first;
        if module.get_val() == 0 {
            res += 1;
        }
        for i in size..n {
            module += boxes[i];
            if module.get_val() == 0 {
                res += 1;
            }
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

// #[bench]
// fn bench_algo(b: &mut Bencher) {
//     b.iter(|| {
//         test::black_box(main());
//     });
// }
