// https://atcoder.jp/contests/abs/tasks/practice_1
use std::io;

fn main() {
    let line_1 = get_input(); // a
    let line_2 = get_input(); // b c
    let line_3 = get_input(); // string

    let a_num: u16 = line_1.parse().unwrap();
    let line_2_split: Vec<&str> = line_2.split(" ").collect();
    let b_num: u16 = line_2_split[0].parse().unwrap();
    let c_num: u16 = line_2_split[1].parse().unwrap();

    println!("{} {}", a_num + b_num + c_num, line_3);
  }

fn get_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    let answer = word.trim().to_string();
    return answer;
}
