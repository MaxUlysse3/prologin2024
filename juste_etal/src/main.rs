use std::{
    ops::{SubAssign, AddAssign},
    iter::repeat_n,
};

fn main() {
    // let (n, val, boxes) = parse_input();

    let boxes = repeat_n(0, 10000).collect::<Vec<_>>();
    let n = boxes.len();
    let val = 1_000_000_000;

    let mut res = ModInt::new(0, 1_000_000_007);

    for i in 0usize..(n) {
        let mut module = ModInt::new(0, val);
        for j in i..n {
            module += boxes[j];
            if module.get_val() == 0 {
                res += 1;
            }
        }
        if i % 1000 == 0 {
            println!("{:?}, {:?} / {:?}", i, module, res);
        }
        // println!("{:?} / {}", module, i);
    }
    println!("{:?}", res);

    println!("{}", res.get_val());
    // println!("{}", (0usize..=n).sum::<usize>() % 1_000_000_007);
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
