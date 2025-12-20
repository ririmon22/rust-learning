// =======================================================
// Rust学習100本ノック 010
// テーマ: Result の合成（map / and_then / map_err）
//
// 【目的】
// - Result<T, E> を「値の流れ」として扱う
// - if / match による分岐を減らす
// - `?` が内部で何をしているかを理解する
// - エラー変換（map_err）の役割を掴む
//
// 【今回の主眼】
// - 成功時の変換: map
// - 失敗時の変換: map_err
// - Result を返す関数の連結: and_then
//
// 【制約】
// - unwrap() / expect() 禁止（テスト除く）
// - panic! 禁止
// - if let / while let 禁止
// - match は最小限（0〜1回）
// - Result の combinator を優先して使う
//
// =======================================================

#[derive(Debug, PartialEq)]
enum ParseCalcError {
    ParseError,
    DivideByZero,
}

fn parse_number(s: &str) -> Result<i32, ParseCalcError> {
    s.parse::<i32>().map_err(|_| ParseCalcError::ParseError)
}

fn derive_100(n: i32) -> Result<i32, ParseCalcError> {
    if n == 0 {
        Err(ParseCalcError::DivideByZero)
    } else {
        Ok(100 / n)
    }
}

fn processs(s: &str) -> Result<i32, ParseCalcError> {
    parse_number(s).and_then(derive_100)
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_success() {
        let r = parse_number("10");
        assert_eq!(r, Ok(10));
    }

    #[test]
    fn parse_number_failure() {
        let r = parse_number("abc");
        assert!(matches!(r, Err(ParseCalcError::ParseError)));
    }

    #[test]
    fn divide_100_success() {
        let r = derive_100(10);
        assert_eq!(r, Ok(10));
    }

    #[test]
    fn divide_100_by_zero_fails() {
        let r = derive_100(0);
        assert!(matches!(r, Err(ParseCalcError::DivideByZero)));
    }

    #[test]
    fn process_success() {
        let r = processs("10");
        assert_eq!(r, Ok(10));
    }

    #[test]
    fn process_parse_error() {
        let r = processs("xyz");
        assert!(matches!(r, Err(ParseCalcError::ParseError)));
    }

    #[test]
    fn process_divide_by_zero_error() {
        let r = processs("0");
        assert!(matches!(r, Err(ParseCalcError::DivideByZero)));
    }
}