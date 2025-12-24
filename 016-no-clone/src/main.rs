// =======================================================
// Rust学習100本ノック 016
// テーマ: clone 禁止コレクション操作
//
// 【目的】
// - clone に逃げず、所有権を正しく移動させる
// - &T / &mut T / T の使い分けを体感する
// - Vec<T> 操作で borrow checker と正面衝突する
//
// 【制約】
// - clone / to_owned 使用禁止
// - if / if let / while let 使用禁止
// - unwrap / expect / panic! 禁止（テスト除く）
// - 所有権の移動は明示的に行う
//
// =======================================================
struct Stack {
    data: Vec<String>,
}

impl Stack {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
    
    fn push(&mut self, value: String) {
        self.data.push(value)
    }
    
    fn pop(&mut self) -> Option<String> {
       self.data.pop()
    }

    fn peek(&self) -> Option<&str> {
        self.data.last().map(|s| s.as_str())
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn new_stack_is_empty() {
        let s = Stack::new();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn push_and_peek_works() {
        let mut s = Stack::new();
        s.push("a".to_string());
        s.push("b".to_string());

        assert_eq!(s.peek(), Some("b"));
    }

    #[test]
    fn pop_returns_last_pushed_value() {
        let mut s = Stack::new();
        s.push("a".to_string());
        s.push("b".to_string());

        let v = s.pop();
        assert_eq!(v, Some("b".to_string()));
    }

    #[test]
    fn pop_removes_element() {
        let mut s = Stack::new();
        s.push("a".to_string());
        s.push("b".to_string());

        assert_eq!(s.pop(), Some("b".to_string()));
        assert_eq!(s.pop(), Some("a".to_string()));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn peek_does_not_remove_element() {
        let mut s = Stack::new();
        s.push("x".to_string());

        let p1 = s.peek();
        let p2 = s.peek();

        assert_eq!(p1, Some("x"));
        assert_eq!(p2, Some("x"));

        // peek では所有権が動かないので pop できる
        assert_eq!(s.pop(), Some("x".to_string()));
    }

    #[test]
    fn push_after_pop_works() {
        let mut s = Stack::new();
        s.push("a".to_string());
        assert_eq!(s.pop(), Some("a".to_string()));

        s.push("b".to_string());
        assert_eq!(s.peek(), Some("b"));
    }
}