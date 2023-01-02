// https://atcoder.jp/contests/abs/tasks/abc087_b

/*
問題文
あなたは、500 円玉を A 枚、100 円玉を B 枚、50 円玉を C 枚持っています。
これらの硬貨の中から何枚かを選び、合計金額をちょうど X 円にする方法は何通りありますか。

同じ種類の硬貨どうしは区別できません。2 通りの硬貨の選び方は、ある種類の硬貨についてその硬貨を選ぶ枚数が異なるとき区別されます。
 */

use std::{io, result};

#[derive(Debug)]
struct ResultData {
    kind: i32,
    num: i32,
}

struct CoinsInfo {
    kind: i32,
    limit: i32,
}

fn main() {
    let line_1 = get_input(); // A
    let line_2 = get_input(); // B
    let line_3 = get_input(); // C
    let line_4 = get_input(); // X

    let A: i32 = line_1.parse().unwrap();
    let B: i32 = line_2.parse().unwrap();
    let C: i32 = line_3.parse().unwrap();
    let X: i32 = line_4.parse().unwrap();

    let limits = vec![A, B, C];

    let kinds: Vec<CoinsInfo> = vec![
        CoinsInfo {
            kind: 500,
            limit: A,
        },
        CoinsInfo {
            kind: 100,
            limit: B,
        },
        CoinsInfo { kind: 50, limit: C },
    ];

    let a = check_one_process(kinds, X);
    // a.iter().filter(|result|{
    //     result.check_limit(A);
    // }).collect();
    // let b = a.check_limit(A).check_limit(B).check_limit(C);

    println!("{:?}", a);
}

// fn processB(kinds: Vec<CoinsInfo>,  mut total: i32){

//     let a = check_one_process(kinds, X);
//     // let b = a.check_limit(A).check_limit(B).check_limit(C);

// }

// 1次元配列の要素の組み合わせから2次元配列を作成する
fn to_combinate<T>(list: Vec<T>) -> Vec<Vec<T>>{
    let ret: Vec<Vec<T>> = Vec::new();

    

    ret
}

trait MyFunc {
    fn check_limit(self, limit: i32) -> Self;
}

impl MyFunc for Vec<ResultData> {
    fn check_limit(self, limit: i32) -> Self {
        let a = self.into_iter().filter(|result| result.num <= limit);
        let b = a.collect::<Vec<ResultData>>();
        b
    }
}

fn check_one_process(coins: Vec<CoinsInfo>, mut total: i32) -> Vec<ResultData> {
    let mut _total = total;
    let mut result_data_list: Vec<ResultData> = Vec::new();
    let a = coins.iter().fold(_total, |mut acc, coin| {
        let (ret, _total) = check_div(coin.kind, coin.limit, acc);
        acc = _total;
        println!("map {:?}, total {}", ret, acc);
        result_data_list.push(ret);
        acc
    });
    println!("a {:?}, result_data_list {:?}", a, result_data_list);
    result_data_list
}

// 硬貨と金額を指定する
// 割ったあとの金額と枚数を出力する
fn check_div(kind: i32, limit: i32, total: i32) -> (ResultData, i32) {
    let mut _total = total;
    let mut num = 0;
    for i in 0..limit {
        if _total < kind {
            return (ResultData { kind, num }, _total);
        }
        _total -= kind;
        num += 1;
    }

    return (ResultData { kind, num }, _total);
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
