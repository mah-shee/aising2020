#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = vec![0; n];
    for i in 1..n {
        for j in 1..i + 1 {
            'inner: for k in 1..j + 1 {
                let tmp = (i + j + k) * (i + j + k) - (i * j + j * k + k * i);
                if tmp > n {
                    break 'inner;
                } else {
                    if i == j && i == k {
                        ans[tmp - 1] += 1;
                    } else if i != j && j != k && k != i {
                        ans[tmp - 1] += 6;
                    } else {
                        ans[tmp - 1] += 3;
                    }
                }
            }
        }
    }
    for i in 0..n {
        println!("{:?}", ans[i]);
    }
}
