// =======================================================
// Rust学習100本ノック 020
// テーマ: 所有権パズル（通すだけ）
//
// 【目的】
// - コンパイルエラーを見て「何が問題か」を説明できるようになる
// - &T / &mut T / T の関係を順序で理解する
// - 借用が有効なスコープを意識する
//
// 【制約】
// - 関数定義は変更禁止
// - clone / to_owned 使用禁止
// - if / if let / while let 使用禁止
// - unsafe 禁止
// - main の中だけを並べ替えて通す
//
// =======================================================

fn consume(s: String) -> usize {
    s.len()
}

fn inspect(s: &String) -> usize {
    s.len()
}

fn modify(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("rust");

    // let a = consume(s);
    let b = inspect(&s);
    modify(&mut s);
    let c = inspect(&s);

    let a = consume(s);

    println!("{a} {b} {c}");
}