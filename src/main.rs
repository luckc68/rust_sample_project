use std::io;

fn main() {
    println!("数字を入力してください: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

    let num: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("有効な数字を入力してください。");
            return;
        }
    };

    if num % 2 == 0 {
        println!("入力された数字 {} は偶数です。", num);
    } else {
        println!("入力された数字 {} は奇数です。", num);
    }
}
