// https://atcoder.jp/contests/abs/tasks/abc086_a
/* 問題文
シカのAtCoDeerくんは二つの正整数 a,b を見つけました。 a と b の積が偶数か奇数か判定してください。
制約
1 ≤ a,b ≤ 10000
a,b は整数
入力
入力は以下の形式で標準入力から与えられる。

 a b
 出力
 積が奇数なら Odd と、 偶数なら Even と出力せよ。

 入力例 1
 Copy
 3 4
 */

use std::io;

fn main() {
    let line_1 = get_input(); // a b

    let line_1_split: Vec<&str> = line_1.split(" ").collect();
    let a_num: u32 = line_1_split[0].parse().unwrap();
    let b_num: u32 = line_1_split[1].parse().unwrap();

    println!("{}", if check_even(a_num * b_num) {"Even"}else{"Odd"});
  }

fn check_even(value: u32) -> bool{
  (value % 2) == 0
}

fn get_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    let answer = word.trim().to_string();
    return answer;
}
