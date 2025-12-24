// =======================================================
// Rust学習100本ノック 019
// テーマ: 関数境界で所有権を移す
//
// 【目的】
// - 引数で T / &T / &mut T を使い分ける
// - 関数を呼んだ瞬間に「誰が所有者か」を意識する
// - clone に逃げずに設計する
//
// 【制約】
// - clone / to_owned 使用禁止
// - if / if let / while let 使用禁止
// - unwrap / expect / panic! 禁止（テスト除く）
//
// =======================================================
fn consume(s: String) -> usize{
    s.len()
}
fn inspect(s: &String) -> usize{
    s.len()
}
fn modify(s: &mut String) {
    s.push_str("!");
}
fn main() {
    let mut s = String::from("hello");

    let len1 = inspect(&s);
    modify(&mut s);
    let len2 = inspect(&s);

    let len3 = consume(s);
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn inspect_does_not_consume_string() {
        let s = String::from("hello");

        let len1 = inspect(&s);
        let len2 = inspect(&s);

        assert_eq!(len1, 5);
        assert_eq!(len2, 5);

        // inspect は所有権を奪わない
        assert_eq!(s, "hello");
    }

    #[test]
    fn modify_changes_string_in_place() {
        let mut s = String::from("hi");

        modify(&mut s);
        modify(&mut s);

        assert_eq!(s, "hi!!");
    }

    #[test]
    fn consume_takes_ownership() {
        let s = String::from("abc");

        let len = consume(s);
        assert_eq!(len, 3);

        // ↓ これはコンパイルエラーになるのが正解
        // let _ = s;
    }

    #[test]
    fn pipeline_works_as_expected() {
        let mut s = String::from("rust");

        let len1 = inspect(&s);
        assert_eq!(len1, 4);

        modify(&mut s);
        let len2 = inspect(&s);
        assert_eq!(len2, 5);

        let len3 = consume(s);
        assert_eq!(len3, 5);
    }
}