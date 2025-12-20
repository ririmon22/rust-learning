// =======================================================
// Rust学習100本ノック 014
// テーマ: match exhaustiveness 破壊テスト
//
// 【目的】
// - enum に variant を追加したとき
//   何が壊れるかを体感する
// - Rust が「網羅性」をどう保証しているか理解する
// - `_ =>` に逃げる危険性を実感する
//
// 【制約】
// - if / if let / while let 使用禁止
// - unwrap / expect / panic! 禁止
// - match は exhaustively 書く
// - `_ =>` は原則使用禁止（比較用に一度だけ可）
//
//
// =======================================================

// =======================================================
// Rust学習100本ノック 014 振り返り
//
// ■ 何をしたか
// CounterState に新しい variant（Broken）を追加し、
// 既存の match（increment / take）を意図的に壊した。
//
// ■ 起きたこと
// - increment と take の match がコンパイルエラーになった
// - 「Broken が網羅されていない」と Rust が指摘してきた
//
// ■ 学んだこと
// - enum + match の exhaustiveness は
//   「将来の変更点をコンパイルエラーで教えてくれる仕組み」
// - `_ =>` を書けば通るが、
//   それは「新しい状態が追加されても気づけなくなる」ことを意味する
// - `_ =>` は安全装置を外す行為であり、
//   設計変更に対するアラートを失う
//
// ■ 今回の判断
// - Broken は「現状あり得ない状態」
// - にもかかわらず enum に追加したことで
//   match が壊れることを確認できた
//
// ■ 結論
// enum + match は分岐のためではなく、
// 「変更に強い設計」を実現するための仕組み。
// =======================================================

enum CounterState {
    Empty,
    Value(i32),
    Broken,
}

struct Counter {
    state: CounterState,
}

impl Counter {
    fn new() -> Self{
        Self {
            state: CounterState::Empty,
        }
    }
    
    fn set(&mut self, v:i32) {
       self.state = CounterState::Value(v);
    }
    
    fn increment(&mut self) -> Result<(), &'static str> {
        self.state =  match self.state {
            CounterState::Value(n) => CounterState::Value(n+1),
            CounterState::Empty => return Err("Error"),
        };
        Ok(())
    }
    
    fn take(&mut self) -> Option<i32> {
        match self.state {
            CounterState::Value(n) => {
                self.state = CounterState::Empty;
                Some(n)
            },
            CounterState::Empty => None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_counter_is_empty() {
        let c = Counter::new();
        match c.state {
            CounterState::Empty => {}
            _ => panic!("new counter must be Empty"),
        }
    }

    #[test]
    fn set_moves_state_to_value() {
        let mut c = Counter::new();
        c.set(10);

        match c.state {
            CounterState::Value(v) => assert_eq!(v, 10),
            _ => panic!("counter must be Value after set"),
        }
    }

    #[test]
    fn increment_on_empty_fails() {
        let mut c = Counter::new();

        let r = c.increment();
        assert!(r.is_err());

        match c.state {
            CounterState::Empty => {}
            _ => panic!("state must remain Empty on error"),
        }
    }

    #[test]
    fn increment_on_value_updates_state() {
        let mut c = Counter::new();
        c.set(1);

        let r = c.increment();
        assert!(r.is_ok());

        match c.state {
            CounterState::Value(v) => assert_eq!(v, 2),
            _ => panic!("state must be Value after increment"),
        }
    }

    #[test]
    fn increment_multiple_times() {
        let mut c = Counter::new();
        c.set(5);

        assert!(c.increment().is_ok());
        assert!(c.increment().is_ok());
        assert!(c.increment().is_ok());

        match c.state {
            CounterState::Value(v) => assert_eq!(v, 8),
            _ => panic!("state must be Value"),
        }
    }

    #[test]
    fn take_from_value_returns_some_and_moves_to_empty() {
        let mut c = Counter::new();
        c.set(42);

        let v = c.take();
        assert_eq!(v, Some(42));

        match c.state {
            CounterState::Empty => {}
            _ => panic!("state must be Empty after take"),
        }
    }

    #[test]
    fn take_from_empty_returns_none_and_stays_empty() {
        let mut c = Counter::new();

        let v = c.take();
        assert_eq!(v, None);

        match c.state {
            CounterState::Empty => {}
            _ => panic!("state must remain Empty"),
        }
    }
}
