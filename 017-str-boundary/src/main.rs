// =======================================================
// Rust学習100本ノック 017
// テーマ: String / &str 境界設計
//
// 【目的】
// - API 境界で所有権をどう扱うか決める
// - 入力は &str、内部は String、出力は &str を徹底
// - clone に逃げずに文字列を扱う
//
// 【制約】
// - clone / to_string / to_owned 使用禁止
// - if / if let / while let 使用禁止
// - unwrap / expect / panic! 禁止（テスト除く）
// - 文字列の所有権移動は明示的に行う
//
// =======================================================
use std::collections::HashMap;

struct Store {
    map: HashMap<String, String>,
}

impl Store {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| s.as_str())
    }
    
    fn remove(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }
}
fn main() {
    let mut s = Store::new();

    s.insert("a".to_string(), "1".to_string());
    s.insert("b".to_string(), "2".to_string());

    assert_eq!(s.get("a"), Some("1"));
    assert_eq!(s.remove("a"), Some("1".to_string()));
    assert_eq!(s.get("a"), None);
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn new_store_is_empty() {
        let s = Store::new();
        assert_eq!(s.get("a"), None);
    }

    #[test]
    fn insert_and_get_work() {
        let mut s = Store::new();

        s.insert("key1".to_string(), "value1".to_string());
        s.insert("key2".to_string(), "value2".to_string());

        assert_eq!(s.get("key1"), Some("value1"));
        assert_eq!(s.get("key2"), Some("value2"));
        assert_eq!(s.get("missing"), None);
    }

    #[test]
    fn get_does_not_move_value() {
        let mut s = Store::new();
        s.insert("a".to_string(), "1".to_string());

        let v1 = s.get("a");
        let v2 = s.get("a");

        assert_eq!(v1, Some("1"));
        assert_eq!(v2, Some("1"));
    }

    #[test]
    fn remove_returns_owned_value() {
        let mut s = Store::new();
        s.insert("a".to_string(), "1".to_string());

        let v = s.remove("a");
        assert_eq!(v, Some("1".to_string()));
    }

    #[test]
    fn remove_removes_entry() {
        let mut s = Store::new();
        s.insert("a".to_string(), "1".to_string());

        assert_eq!(s.remove("a"), Some("1".to_string()));
        assert_eq!(s.get("a"), None);
    }

    #[test]
    fn remove_nonexistent_key_returns_none() {
        let mut s = Store::new();
        assert_eq!(s.remove("missing"), None);
    }

    #[test]
    fn insert_after_remove_works() {
        let mut s = Store::new();
        s.insert("a".to_string(), "1".to_string());
        s.remove("a");

        s.insert("a".to_string(), "2".to_string());
        assert_eq!(s.get("a"), Some("2"));
    }
}