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
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        m: usize,
        a: [usize; n]
    };

    let sum = a.iter().sum();

    let mut ans = 0;
    for i in 0..n{
        if a[i] * 4 * m >= sum {
            ans += 1
        }
    }

    if ans >= m {
        println!("Yes")
    }
    else {
        println!("No");
    }
}
