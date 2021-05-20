use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    let source = AutoSource::from("0");
    input!{
        // from source,
        k: usize
    };

    let mut all = vec![];

    for i in 1..=9 {
        rec(1, i, &mut all)
    }

    all.sort();

    println!("{}", all[k-1]);
}

fn rec(d:usize, val: i64, mut all:&mut Vec<i64>){
    all.push(val);

    if d == 10 {
        return
    }

    for j in -1..=1{
        let add = val % 10 + j;
        if add < 0 || add > 9{
            continue
        }
        rec(d+1, val * 10 + add, all)
    }
}