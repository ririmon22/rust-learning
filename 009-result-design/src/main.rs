// =======================================================
// Rust学習100本ノック 009
// テーマ: Result と Error enum を「設計として」使う
//
// 【目的】
// - Option と Result の使い分けを明確にする
// - panic を使わず、失敗を型で表現する
// - エラーの「種類」を enum で設計する
// - 呼び出し側にとって使いやすい API を考える
//
// 【今回の主眼】
// - 「失敗する可能性がある操作」は Result を返す
// - 成功/失敗の分岐を if / match で雑に書かない
// - エラーを文字列ではなく「意味のある型」にする
//
// 【作るもの】
// 数値を保持する小さな構造体と、
// そこに対する「失敗しうる操作」
//
// 例:
// - 0 で割ろうとすると失敗
// - 不正な値を入れようとすると失敗
//
// 【制約】
// - unwrap() / expect() 禁止（テスト除く）
// - panic! 禁止
// - エラーは enum で表現する
// - Result<T, E> を正面から使う
// - main.rs 1ファイル完結
//
// 【確認ポイント】
// - Option では表現しきれない理由を説明できるか
// - なぜこのエラー enum が必要か説明できるか
// - 呼び出し側のコードが読みやすいか
//
// =======================================================
struct Divider {
    value: i32,
}

#[derive(Debug, PartialEq)]
enum DivideError {
    DivideByZero,
    NegativeInput,
}

impl Divider {
    fn new(value: i32) -> Result<Self, DivideError> {
        if value >= 0 {
            Ok(Self{
                value:value,
            })
        } else {
            Err(DivideError::NegativeInput)
        }
    }
    
    fn divide(&self, divisor: i32) -> Result<i32, DivideError> {
        if divisor != 0 {
            let result = self.value / divisor;
            Ok(result)
        } else {
            Err(DivideError::DivideByZero)
        }
    }
}
fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_positive_value_succeeds() {
        let d = Divider::new(10);
        assert!(d.is_ok());

        let d = d.unwrap();
        assert_eq!(d.divide(2), Ok(5));
    }

    #[test]
    fn new_with_negative_value_fails() {
        let d = Divider::new(-1);
        assert!(matches!(d, Err(DivideError::NegativeInput)));
    }

    #[test]
    fn divide_by_non_zero_succeeds() {
        let d = Divider::new(20).unwrap();
        let result = d.divide(4);

        assert_eq!(result, Ok(5));
    }

    #[test]
    fn divide_by_zero_fails() {
        let d = Divider::new(10).unwrap();
        let result = d.divide(0);

        assert_eq!(result, Err(DivideError::DivideByZero));
    }

    #[test]
    fn divide_does_not_change_internal_state() {
        let d = Divider::new(9).unwrap();

        let r1 = d.divide(3);
        let r2 = d.divide(3);

        assert_eq!(r1, Ok(3));
        assert_eq!(r2, Ok(3));
    }
}