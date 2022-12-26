// https://atcoder.jp/contests/abs/tasks/abc081_a
/*
問題文
すぬけ君は 1,2,3 の番号がついた 3 つのマスからなるマス目を持っています。 各マスには 0 か 1 が書かれており、マス i には si
​
  が書かれています。

すぬけ君は 1 が書かれたマスにビー玉を置きます。 ビー玉が置かれるマスがいくつあるか求めてください。

制約
s1, s2 ,s3 は 1 あるいは 0
 */

use std::io;

fn main() {
    let line_1 = get_input(); // abc

    let count = line_1.chars().fold(0, |mut a, b| {
        if b == '1' {
            a += 1
        };
        a
    });

    let result: String = count.to_string();

    println!("{}", result);
}

fn get_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    let answer = word.trim().to_string();
    return answer;
}
