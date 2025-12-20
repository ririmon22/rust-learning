use std::f32::consts::E;

struct Slot {
    value: Option<String>,
}

impl Slot {
    fn new() -> Self {
        Self{
            value: None,
        }
    }
    
    fn is_empty(&self) -> bool {
        match &self.value {
            Some(_) => false,
            _ => true,
        }
    }
    fn insert(&mut self, v: String) -> Result<(), &'static str> {
        if self.value.is_some() {
            Err("Already intialized")
        } else {
            self.value.get_or_insert(v);
            Ok(())
        }
    }
    
    fn peek(&self) -> Option<&str> {
        self.value.as_deref()
    }
    
    fn take(&mut self) -> Option<String>{
        std::mem::replace(&mut self.value, None)
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_slot_is_empty() {
        let slot = Slot::new();
        assert!(slot.is_empty());
        assert_eq!(slot.peek(), None);
    }

    #[test]
    fn insert_into_empty_slot() {
        let mut slot = Slot::new();

        let result = slot.insert("hello".to_string());
        assert!(result.is_ok());

        assert!(!slot.is_empty());
        assert_eq!(slot.peek(), Some("hello"));
    }

    #[test]
    fn insert_into_filled_slot_fails() {
        let mut slot = Slot::new();

        slot.insert("first".to_string()).unwrap();
        let result = slot.insert("second".to_string());

        assert!(result.is_err());
        assert_eq!(slot.peek(), Some("first"));
    }

    #[test]
    fn peek_does_not_move_value() {
        let mut slot = Slot::new();
        slot.insert("hello".to_string()).unwrap();

        let p1 = slot.peek();
        let p2 = slot.peek();

        assert_eq!(p1, Some("hello"));
        assert_eq!(p2, Some("hello"));
        assert!(!slot.is_empty());
    }

    #[test]
    fn take_from_filled_slot() {
        let mut slot = Slot::new();
        slot.insert("hello".to_string()).unwrap();

        let v = slot.take();

        assert_eq!(v, Some("hello".to_string()));
        assert!(slot.is_empty());
        assert_eq!(slot.peek(), None);
    }

    #[test]
    fn take_from_empty_slot() {
        let mut slot = Slot::new();

        let v = slot.take();

        assert_eq!(v, None);
        assert!(slot.is_empty());
    }
}