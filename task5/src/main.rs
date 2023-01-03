// https://atcoder.jp/contests/abs/tasks/abc087_b

/*
問題文
あなたは、500 円玉を A 枚、100 円玉を B 枚、50 円玉を C 枚持っています。
これらの硬貨の中から何枚かを選び、合計金額をちょうど X 円にする方法は何通りありますか。

同じ種類の硬貨どうしは区別できません。2 通りの硬貨の選び方は、ある種類の硬貨についてその硬貨を選ぶ枚数が異なるとき区別されます。
 */

use std::io;

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

    let coin500 = CoinsInfo {
        kind: 500,
        limit: A,
    };
    let coin100 = CoinsInfo {
        kind: 100,
        limit: B,
    };
    let coin50 = CoinsInfo { kind: 50, limit: C };

    let coinsinfo_combine = three_kaijou_to_list(&coin500, &coin100, &coin50);

    use std::collections::HashSet;
    let mut result_list: Vec<ResultStruct> = Vec::new();

    for item in coinsinfo_combine {
        let a = check_one_process(item, X);
        if a.is_some() {
            // println!("a = {:?}", a);
            result_list.push(a.unwrap());
        }
    }
    let b: HashSet<&ResultStruct> = HashSet::from_iter(result_list.iter());
    println!("b = {:?}", b);
}

// 1次元配列の要素の組み合わせから2次元配列を作成する
fn to_combinate<T>(list: Vec<T>) -> Vec<Vec<T>> {
    let ret: Vec<Vec<T>> = Vec::new();

    // let b = three_kaijou_to_list(A, B, C);

    ret
}

// 3の階乗のリストを作成する
// in
// A, B, C
// out
// [[A, B, C], [A, C, B], [B, A, C], [B, C, A], [C, A, B], [C, B, A]]
fn three_kaijou_to_list<'a, T>(a: &'a T, b: &'a T, c: &'a T) -> Vec<Vec<&'a T>> {
    fn process<'a, T>(
        root: &'a T,
        branch_1: &'a T,
        branch_2: &'a T,
    ) -> ((&'a T, &'a T, &'a T), (&'a T, &'a T, &'a T)) {
        let (two_tuple1, two_tuple2) = two_kaijo_to_list(branch_1, branch_2);
        let result_list = (
            (root, two_tuple1.0, two_tuple1.1),
            (root, two_tuple2.0, two_tuple2.1),
        );
        result_list
    }

    let (result1_list1, result1_list2) = process(a, b, c);
    let (result2_list1, result2_list2) = process(b, a, c);
    let (result3_list1, result3_list2) = process(c, a, b);

    let a = vec![
        result1_list1,
        result1_list2,
        result2_list1,
        result2_list2,
        result3_list1,
        result3_list2,
    ];

    let b = a
        .iter()
        .map(|&item| vec![item.0, item.1, item.2])
        .collect::<Vec<Vec<&T>>>();

    let result: Vec<Vec<&T>> = b;

    result
}

// 2の階乗のリストを作成する
// in
// A, B
// out
// [(A, B), (B, A)]
fn two_kaijo_to_list<'a, T>(a: &'a T, b: &'a T) -> ((&'a T, &'a T), (&'a T, &'a T)) {
    ((a, b), (b, a))
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

#[derive(Debug, Hash, Eq, PartialEq)]
struct ResultStruct {
    coin500_num: i32,
    coin100_num: i32,
    coin50_num: i32,
}

fn check_one_process(coins: Vec<&CoinsInfo>, total: i32) -> Option<ResultStruct> {
    // 硬貨と金額を指定する
    // 割ったあとの金額と枚数を出力する
    fn check_div(kind: i32, limit: i32, total: i32) -> (i32, i32) {
        let mut _total = total;
        let mut num = 0;
        for _ in 0..limit {
            if _total < kind {
                return (num, _total);
            }
            _total -= kind;
            num += 1;
        }

        return (num, _total);
    }

    let mut _total = total;
    let (mut coin500_num, mut coin100_num, mut coin50_num) = (0, 0, 0);

    let remaining_total = coins.iter().fold(_total, |mut acc, coin| {
        let (div_num, _total) = check_div(coin.kind, coin.limit, acc);

        match coin.kind {
            500 => coin500_num += div_num,
            100 => coin100_num += div_num,
            50 => coin50_num += div_num,
            _ => (),
        };

        acc = _total;
        acc
    });

    let result_struct: ResultStruct = ResultStruct {
        coin500_num,
        coin100_num,
        coin50_num,
    };

    if remaining_total != 0 {
        return None;
    }

    Some(result_struct)
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
