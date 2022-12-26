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
    let result: String = "";
     let line_1 = get_input(); // a b
 
    //  let line_1_split: Vec<&str> = line_1.split(" ").collect();
    //  let a_num: u32 = line_1_split[0].parse().unwrap();
    //  let b_num: u32 = line_1_split[1].parse().unwrap();
 

     println!("{}", result);
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