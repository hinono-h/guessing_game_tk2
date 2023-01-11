use rand::Rng;
use std::cmp::Ordering;
use std::io;

//Rng：乱数生成期が生成すべきメソッドを実装している
//Rustはデフォで標準ライブラリで定義されているアイテムの中のいくつかを
//すべてのスコープに取り込む
//このセットはprelude(プレリュード)と呼ばれ、標準ライブラリのドキュメントで見られる
//RustがすべてのRustプログラムに自動的にインポートするもののリスト
//→手動だと短調になるが、自動だと大量にインポートされるが、それを回避する
//ほぼ全てのRustプログラムで使用されるもの、特に特性に焦点をあてたものとなっている

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    //gen_range メソッドは、use rand::Rng 文でスコープに導入した Rng トレイとで定義されている
    //rand::thread_rng() 函数を呼び出して、これから使うある特定の乱数生成器を取得している
    //この乱数生成器は現在のスレッドに固有 / オペレーティングシステムからシート地を得ている
    //ここで使っている範囲式の種類は「開始..終了」という形式で、下限値は含むが上限値は含まない
    //→1から100までをリクエストしたい場合は1..191と入寮くする必要がある
    //→ 「1..=100」という範囲を渡すこともできる

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
