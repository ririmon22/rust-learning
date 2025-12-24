/*
課題022：&T / &mut T API 設計（実戦編）

制約：
- clone禁止（テスト含む）
- &mut T を外部に直接返さない
- borrow checker が拒否する設計を1つ含める
- エラー理由を文章で説明する
*/

struct TextBuffer {
    data: Vec<String>,
}

impl TextBuffer {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn line_count(&self) -> usize {
        self.data.len()
    }
    
    fn get_line(&self, index: usize) -> Option<&str> {
        self.data.get(index).map(|s|  s.as_str())
    }
    
    fn all_lines(&self) -> impl Iterator<Item = &str>{
        self.data.iter().map(|s| s.as_str())
    }
    
    fn push_line(&mut self, line: String) {
        self.data.push(line)
    }
    
    fn replace_line(&mut self, index: usize, line: String) -> Option<String> {
        if index < self.data.len() {
            Some(std::mem::replace(&mut self.data[index], line))
        } else {
            None
        }
    }
    
    fn clear(&mut self) {
        self.data.clear();
    }
    
    fn get_and_append_len(&mut self, index: usize) {
        if let Some(len) = self.data.get(index).map(|s| s.len()) {
            self.data.push(len.to_string());
        }
    }
}

fn main() {
    let mut buf = TextBuffer::new();

    buf.push_line("rust".to_string());
    buf.push_line("language".to_string());

    println!("line count: {}", buf.line_count());

    if let Some(line) = buf.get_line(0) {
        println!("first line: {}", line);
    }
    
    buf.get_and_append_len(1);
    
    for line in buf.all_lines(){
        println!("line: {}", line);
    }
    
    if let Some(line) = buf.replace_line(1, "dijfk".to_string()) {
        println!("{}", line);
    }
    for line in buf.all_lines(){
        println!("line: {}", line);
    }
    buf.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_and_append_len_appends_length_as_string() {
        let mut buf = TextBuffer::new();
        buf.push_line("rust".to_string());
        buf.push_line("lang".to_string());

        buf.get_and_append_len(0);

        assert_eq!(buf.line_count(), 3);
        assert_eq!(buf.get_line(2), Some("4"));
    }

    #[test]
    fn get_and_append_len_invalid_index_does_nothing() {
        let mut buf = TextBuffer::new();
        buf.push_line("hello".to_string());

        buf.get_and_append_len(10);

        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.get_line(0), Some("hello"));
    }
}