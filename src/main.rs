fn main() {
    fizz_buzz15(100)
}

#[allow(dead_code)]
fn fizz_buzz1() {
    // Rustの関数定義（引数なし、戻り値なし）
    let mut x = 1; // xという変数を導入して整数1に束縛

    while x <= 100 {
        // xが100以下の場合に中括弧`{}`で囲まれたスコープを繰り返す
        if x % 15 == 0 {
            // 15で割り切れる（15で割った余りが0）
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            // 3で割り切れる（3で割った余りが0）
            println!("Fizz");
        } else if x % 5 == 0 {
            // 5で割り切れる（5で割った余りが0）
            println!("Buzz");
        } else {
            // それ以外の場合は数字を直接出力
            println!("{}", x);
        }

        x += 1; // xに1を加算
    }
}

#[allow(dead_code)]
fn fizz_buzz2() {
    for x in 1..101 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}

#[allow(dead_code)]
fn fizzbuzz_match() {
    for x in 1..=100 {
        match x % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", x),
        }
    }
}

#[allow(dead_code)]
fn fizzbuzz_guard() {
    for x in 1..=100 {
        match x {
            e if e % 15 == 0 => println!("FizzBuzz"),
            e if e % 3 == 0 => println!("Fizz"),
            e if e % 5 == 0 => println!("Buzz"),
            e => println!("{}", e),
        }
    }
}

#[allow(dead_code)]
fn tuple() {
    for x in 1..=100 {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", x),
        }
    }
}

#[allow(dead_code)]
fn tuple2() {
    for x in 1..=100 {
        let tmp;
        let s = match (x % 3, x % 5) {
            (0, 0) => "FizzBuzz",
            (0, _) => "Fizz",
            (_, 0) => "Buzz",
            _ => {
                tmp = x.to_string();
                &tmp
            }
        };
        println!("{}", s)
    }
}

#[allow(dead_code)]
fn closure() {
    let fz = |x: i32| match (x % 3, x % 5) {
        (0, 0) => format!("FizzBuzz"),
        (0, _) => format!("Fizz"),
        (_, 0) => format!("Buzz"),
        _ => x.to_string(),
    };
    (1..=100).map(fz).for_each(|x| println!("{}", x));
}

#[allow(dead_code)]
fn fold() {
    let res = (1..=100).fold(format!(""), |buf, x| {
        // 文字列の畳み込みを行う
        match (x % 3, x % 5) {
            (0, 0) => format!("{}FizzBuzz\n", buf),
            (0, _) => format!("{}Fizz\n", buf),
            (_, 0) => format!("{}Buzz\n", buf),
            _ => format!("{}{}\n", buf, x),
        }
    });

    println!("{}", res);
}

#[allow(dead_code)]
fn join() {
    fn fz(x: i32) -> String {
        match (x % 3, x % 5) {
            (0, 0) => format!("FizzBuzz"),
            (0, _) => format!("Fizz"),
            (_, 0) => format!("Buzz"),
            _ => x.to_string(),
        }
    }

    let res = (1..=100).map(fz).collect::<Vec<_>>().join("\n");

    println!("{}", res);
}

#[allow(dead_code)]
fn fizz_buzz11() {
    // use std::ops::Rem;

    fn fz<T>(x: T, div_a: T, div_b: T, zero: T) -> String
    where
        T: Rem<T, Output = T> + Eq + Copy + ToString,
    {
        match (x % div_a == zero, x % div_b == zero) {
            (true, true) => format!("FizzBuzz"),
            (true, _) => format!("Fizz"),
            (_, true) => format!("Buzz"),
            _ => x.to_string(),
        }
    }

    (1..=100)
        .map(|x: u32| fz(x, 3, 5, 0))
        .for_each(|x| println!("{}", x));
}

#[allow(dead_code)]
struct FizzBuzz<T> {
    div_a: T,
    div_b: T,
    zero: T,
}

impl<T> FizzBuzz<T> {
    fn new(div_a: T, div_b: T, zero: T) -> Self {
        FizzBuzz { div_a, div_b, zero }
    }
}

trait ToFzStr<T> {
    fn to_str(&self, x: T) -> String;
}

// impl ToFzStr<i32> for FizzBuzz<i32> {
//     fn to_str(&self, x: i32) -> String {
//         match (x % self.div_a == self.zero, x % self.div_b == self.zero) {
//             (true, true) => format!("FizzBuzz"),
//             (true, _) => format!("Fizz"),
//             (_, true) => format!("Buzz"),
//             _ => x.to_string(),
//         }
//     }
// }
use std::ops::Rem;
fn common_fz_str<T>(x: T, div_a: T, div_b: T, zero: T) -> String
where
    T: Rem<T, Output = T> + Eq + Copy + ToString,
{
    match (x % div_a == zero, x % div_b == zero) {
        (true, true) => format!("FizzBuzz"),
        (true, _) => format!("Fizz"),
        (_, true) => format!("Buzz"),
        _ => x.to_string(),
    }
}

impl ToFzStr<i32> for FizzBuzz<i32> {
    fn to_str(&self, x: i32) -> String {
        common_fz_str(x, self.div_a, self.div_b, self.zero)
    }
}

impl ToFzStr<i64> for FizzBuzz<i64> {
    fn to_str(&self, x: i64) -> String {
        common_fz_str(x, self.div_a, self.div_b, self.zero)
    }
}

impl ToFzStr<u32> for FizzBuzz<u32> {
    fn to_str(&self, x: u32) -> String {
        common_fz_str(x, self.div_a, self.div_b, self.zero)
    }
}

#[allow(dead_code)]
fn fizz_buzz14(end: usize) {
    (1..)
        .take(end)
        .map(|x| FizzBuzz::new(3, 5, 0).to_str(x))
        .for_each(|x| println!("{}", x))
}

#[allow(dead_code)]
fn fizz_buzz12() {
    (1..=100)
        .map(|x| FizzBuzz::new(3, 5, 0).to_str(x))
        .for_each(|x| println!("{}", x))
}

#[allow(dead_code)]
fn fizz_buzz15(end: u32) {
    if end > 1 {
        fizz_buzz15(end - 1)
    }

    println!(
        "{}",
        FizzBuzz {
            div_a: 3,
            div_b: 5,
            zero: 0
        }
        .to_str(end)
    )
}
