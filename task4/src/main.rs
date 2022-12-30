// https://atcoder.jp/contests/abs/tasks/abc081_b

/*
問題文
黒板に N 個の正の整数 A1 ,...,AN
​が書かれています．
すぬけ君は，黒板に書かれている整数がすべて偶数であるとき，次の操作を行うことができます．
黒板に書かれている整数すべてを，2 で割ったものに置き換える．
 */

// in
// N
// A1 A2 A3 ...

use std::io;

fn main() {
    let _line_1 = get_input(); // N
    let line_2 = get_input(); // A1 A2 A3 ...

    let mut items: Vec<u64> = line_2
        .split(" ")
        .map(|item| item.parse::<u64>().unwrap())
        .collect();
    
    let mut count = 0;
    // 全部割り切れるか
    while check_all_even(&items){
    // 全部2で割る
    items = even_all(&items);
    count += 1;
    }

    println!("{}", count);
}

fn check_all_even(items: &Vec<u64>) -> bool {
    // イテレーター内のIntがすべて偶数かを確認する
    (*items).iter().all(|item| item % 2 == 0)
}

fn even_all(vec: &Vec<u64>) -> Vec<u64> {
    (*vec.iter().map(|item| item / 2).collect::<Vec<u64>>()).to_vec()
}

// イテレーター内のIntを割る2する

fn get_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    let answer = word.trim().to_string();
    return answer;
}
