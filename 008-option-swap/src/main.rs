// =======================================================
// Rust学習100本ノック 008
// テーマ: Option を「状態の入れ替え装置」として使う
//
// 【目的】
// - Option<T> を match / if に頼らず操作する
// - Option::take / Option::replace の役割を体で覚える
// - 「読む + 書き換える」を1操作で行う感覚を身につける
// - 所有権を clone せずに安全に移動させる
//
// 【作るもの】
// struct Cell {
//     value: Option<i32>,
// }
//
// 【実装するメソッド】
// - new()        : 空の Cell を作る
// - set(v)       : 常に Some(v) にする（元の値があれば返す）
// - clear()      : 中身を None にする（元の値があれば返す）
// - swap(v)      : 今の値と v を入れ替える（元の値を返す）
// - peek()       : 中身を見るだけ（状態を壊さない）
//
// 【状態遷移の意識】
// - replace : 新しい値に入れ替え、元の値を返す
// - take    : 中身を取り出し、None にする
//
// 【制約】
// - clone() / unwrap() / expect() 禁止（テスト除く）
// - if let 禁止
// - match の使用は最小限（0〜1回）
// - Option の API を優先して使う
// - main.rs 1ファイル完結
//
// 【確認ポイント】
// - どのメソッドが &self / &mut self か説明できるか
// - なぜ T ではなく Option<T> を返すのか説明できるか
// - replace と take の違いを言葉で説明できるか
//
// =======================================================

struct Cell {
    value: Option<i32>,
}

impl Cell {
    fn new() -> Self {
        Self {
            value:None,
        }
    }
    
    fn peek(&self) -> Option<i32> {
        self.value
    }

    fn set(&mut self, v:i32) -> Option<i32> {
        self.value.replace(v)
    }
    
    fn clear(&mut self) -> Option<i32> {
        self.value.take()
    }
    
    fn swap(&mut self, v: i32) -> Option<i32> {
        self.value.replace(v)
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cell_is_empty() {
        let c = Cell::new();
        assert_eq!(c.peek(), None);
    }

    #[test]
    fn set_on_empty_cell() {
        let mut c = Cell::new();

        let old = c.set(10);
        assert_eq!(old, None);
        assert_eq!(c.peek(), Some(10));
    }

    #[test]
    fn set_overwrites_existing_value() {
        let mut c = Cell::new();

        c.set(10);
        let old = c.set(20);

        assert_eq!(old, Some(10));
        assert_eq!(c.peek(), Some(20));
    }

    #[test]
    fn clear_on_filled_cell() {
        let mut c = Cell::new();

        c.set(42);
        let old = c.clear();

        assert_eq!(old, Some(42));
        assert_eq!(c.peek(), None);
    }

    #[test]
    fn clear_on_empty_cell() {
        let mut c = Cell::new();

        let old = c.clear();

        assert_eq!(old, None);
        assert_eq!(c.peek(), None);
    }

    #[test]
    fn swap_on_empty_cell() {
        let mut c = Cell::new();

        let old = c.swap(5);

        assert_eq!(old, None);
        assert_eq!(c.peek(), Some(5));
    }

    #[test]
    fn swap_on_filled_cell() {
        let mut c = Cell::new();

        c.set(1);
        let old = c.swap(2);

        assert_eq!(old, Some(1));
        assert_eq!(c.peek(), Some(2));
    }
}