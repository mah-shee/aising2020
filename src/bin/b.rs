#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]

    }
    let mut ans = 0;
    for i in 0..n {
        if a[i] % 2 == 1 && i % 2 == 0 {
            ans += 1;
        }
    }
    println!("{:?}", ans);
}
