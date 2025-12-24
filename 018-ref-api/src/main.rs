// =======================================================
// Rust学習100本ノック 018
// テーマ: &T / &mut T API 設計
//
// 【目的】
// - 読み取り専用 API と更新 API を型で分離する
// - &mut を必要最小限に閉じ込める
// - borrow checker による安全性を設計として使う
//
// 【制約】
// - clone / to_owned 使用禁止
// - if / if let / while let 使用禁止
// - unwrap / expect / panic! 禁止（テスト除く）
// - &mut self は「本当に更新する API」だけに使う
//
// =======================================================
struct Config {
    timeout: u64,
    retries: u32,
}

impl Config {
    fn new(timeout: u64, retries: u32) -> Self {
        Self { timeout, retries }
    }
    
    fn timeout(&self) -> u64 {
        self.timeout
    }
    
    fn retries(&self) -> u32 {
        self.retries
    }
    
    fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }

    fn inc_retries(&mut self) {
        self.retries += 1;
    }
}
fn main() {
    let mut c = Config::new(100, 3);

    assert_eq!(c.timeout(), 100);
    assert_eq!(c.retries(), 3);
    
    c.set_timeout(200);
    c.inc_retries();

    assert_eq!(c.timeout(), 200);
    assert_eq!(c.retries(), 4);
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn new_config_sets_values() {
        let c = Config::new(100, 3);
        assert_eq!(c.timeout(), 100);
        assert_eq!(c.retries(), 3);
    }

    #[test]
    fn getters_do_not_modify_state() {
        let c = Config::new(50, 1);

        let t1 = c.timeout();
        let t2 = c.timeout();
        let r1 = c.retries();
        let r2 = c.retries();

        assert_eq!(t1, 50);
        assert_eq!(t2, 50);
        assert_eq!(r1, 1);
        assert_eq!(r2, 1);
    }

    #[test]
    fn set_timeout_updates_value() {
        let mut c = Config::new(100, 3);

        c.set_timeout(200);

        assert_eq!(c.timeout(), 200);
        assert_eq!(c.retries(), 3);
    }

    #[test]
    fn inc_retries_increments_only_retries() {
        let mut c = Config::new(100, 3);

        c.inc_retries();
        c.inc_retries();

        assert_eq!(c.retries(), 5);
        assert_eq!(c.timeout(), 100);
    }

    #[test]
    fn mutable_and_immutable_access_are_separated() {
        let mut c = Config::new(10, 0);

        let t = c.timeout(); // &self
        assert_eq!(t, 10);

        c.inc_retries();     // &mut self
        assert_eq!(c.retries(), 1);
    }
}