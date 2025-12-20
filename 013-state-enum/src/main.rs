// =======================================================
// Rust学習100本ノック 013
// テーマ: 状態 enum による設計（Option を捨てる）
//
// 【目的】
// - Option による「ある / ない」を状態 enum に昇格させる
// - 状態遷移を match で明示的に表現する
// - 「ありえない状態」を型で潰す
//
// 【制約】
// - Option 使用禁止
// - if / if let / while let 使用禁止
// - unwrap / expect / panic! 禁止
// - 状態遷移は match で exhaustively 書く
//
// =======================================================
enum CounterState {
    Empty,
    Value(i32),
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