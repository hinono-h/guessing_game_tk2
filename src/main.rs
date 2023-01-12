use rand::Rng;
use std::cmp::Ordering;
use std::io; //std；standerd（標準）ライブラリ //io；入出力ライブラリ

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // gen_range メソッドは、use rand::Rng 文でスコープに導入した Rng トレイとで定義されている
    // rand::thread_rng() 函数を呼び出して、これから使うある特定の乱数生成器を取得している
    // この乱数生成器は現在のスレッドに固有 / オペレーティングシステムからシート地を得ている
    // ここで使っている範囲式の種類は「開始..終了」という形式で、下限値は含むが上限値は含まない
    // →1から100までをリクエストしたい場合は1..191と入寮くする必要がある
    // → 「1..=100」という範囲を渡すこともできる

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        //let文を使って変数を作っている
        //let mut guess：guessという名前の可変（mutable）変数を導入している
        //Rustの等号記号（=）：いまこの変数を何かに束縛したい
        //String（サイズが拡張可能な文字列型）
        //String::new 関数を呼び出して guess が束縛される値/ この関数はString型の新しいインスタンスを返す
        //ちなみにUTF-8でエンコードされたテキスト片になる

        //newの隣にある「::」構文は newがString型の関連関数※であることを示している
        //関連関数：ある型（ここでは String）に対して実装される関数のこと
        //このnew函数は新しいからの文字列を作成する；new()
        //let mut guess = String::new(); という行は可変変数を作成し
        //その変数は現時点で新しい空のStringののインスタンスに束縛されている

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //プログラムの最初に use std::ioと書いて、標準ライブラリの出力機能を取り込んだアレ
        //ここでioモジュールのstdin函数を呼び出して、ユーザ入力を処理できるようにしている
        //※プログラムの最初にioライブラリをインポートしていなくても、std::io::stdinのように呼び出せる
        //stdin関数はターミナルの標準入力へのハンドルをあらわす型であるstd::io::stdinのインスタンスを返す
        //.read_line(&mut guess)行は標準入力ハンドルのread_lineメソッドを呼び出し、ユーザからの入力を得ている
        //また、read_lineの引数として &mut guessを私、ユーザ入力をどの文字に格納するかを指示している
        //「&」は参照であることを示している。
        //これによりコードの複数の部分が同じデータにアクセスしても、そのデータを何度もメモリにコピらずに済む

        //.method_name()構文でメソッドを呼び出すとき、長い行を改行と空白で分割するのが賢明なことがよくある
        //つまり、.read_line を含む2行は「.read_line(&mut guess).expect("faled to read line");」と書ける

        //read_lineは（前述）渡された文字列にユーザが入力されたものを入れる
        //しかし、同時に値も返す；この場合は for Result※
        //Result型とは列挙型で、enumとも呼ばれ、取りうる値として決まった数の列挙子（veriant）を持つ
        //列挙型はよくmatch（下の方の行に出てくる）と一緒に使われる。
        //これは条件式の一種で、評価時に、列挙型の値がどの列挙子であるかに基づいて異なるコードを実行できる（便利！）
        //Result型の目的：エラー処理に関わる情報を符号化（エンコード）すること

        //resultの列挙子は「Ok」か「Errr」
        //Ok列挙子は処理が成功し、Okの中には正常な値；Errの場合cプログラムをクラッシュさせ引数のメッセージを表示

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
                //ここではguessとsecret_numberを比較している
                //そしてuse分でスコープに導入したOrdering列挙型の列挙子を返す
                //ここではmatch指揮を使用している
                //guessとssecret_numberの値に対してcmpを読んだ結果返されたOrderingの列挙子に基づき次の動作を決定している

                //match式は複数のアーム（腕）で構成されている
                //各アームはマッチさせるパターンと
                //matchに与えられた値がそのアームのpatternにマッチしたときに実行されるコードで構成される

                //Rustは強い静的型システムを持つが、型推論も備えている。
                //let guess = String::new(): と書いたとき、RustはguessがString型であるべきと推論→書かずに済んだ
                //一方 secret_number は数値型；1から100までの値を表現でき、その型には32ビット数値のi32, 同様にi64等がある
                //Rustは文字列と数値型を比較できないとエラー原因になるので注意
                //最終的にはプログラムんお入力として読み込んだStringを実数型に変換し、秘密の数字と数値として比較できるようにしたい
            }
        }
    }
}
