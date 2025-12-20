// =======================================================
// Rust学習100本ノック 012
// テーマ: if let / while let 禁止（match 強制）
//
// 【目的】
// - Option / Result を必ず match で処理する
// - 分岐の抜けを exhaustiveness で潰す
// - 「楽な書き方」に逃げない設計筋トレ
//
// 【制約】
// - if / if let / while let 使用禁止
// - unwrap / expect / panic! 禁止
// - Option / Result は match でのみ分解
// - _ の乱用禁止（意味のある分岐を書く）
//
// =======================================================

struct Counter {
    value: Option<i32>,
}

impl Counter {
    fn new() -> Self{
        Self { value: None }
    }
    
    fn increment(&mut self) -> Result<(), &'static str> {
        self.value = match self.value {
            Some(n) => Some(n+1),
            None => return Err("Error")
        };
        Ok(())
    }
    
    fn take(&mut self) -> Option<i32> {
        self.value.take()
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
        assert_eq!(c.value, None);
    }

    #[test]
    fn increment_on_empty_counter_fails() {
        let mut c = Counter::new();
        let r = c.increment();
        assert!(r.is_err());
        assert_eq!(c.value, None);
    }

    #[test]
    fn increment_on_some_value_updates_state() {
        let mut c = Counter::new();
        c.value = Some(0);

        let r = c.increment();
        assert!(r.is_ok());
        assert_eq!(c.value, Some(1));
    }

    #[test]
    fn increment_multiple_times() {
        let mut c = Counter::new();
        c.value = Some(5);

        assert!(c.increment().is_ok());
        assert!(c.increment().is_ok());
        assert!(c.increment().is_ok());

        assert_eq!(c.value, Some(8));
    }

    #[test]
    fn take_returns_value_and_clears_state() {
        let mut c = Counter::new();
        c.value = Some(10);

        let v = c.take();
        assert_eq!(v, Some(10));
        assert_eq!(c.value, None);
    }

    #[test]
    fn take_on_empty_counter_returns_none() {
        let mut c = Counter::new();

        let v = c.take();
        assert_eq!(v, None);
        assert_eq!(c.value, None);
    }
}