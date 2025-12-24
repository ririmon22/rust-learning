/*
課題023：所有権と借用の総まとめ

目的：
- &self / &mut self / T の使い分けを説明できるようにする
- API 設計から所有権の流れを言語化する

制約：
- clone / Rc / RefCell 禁止
- unsafe 禁止
- API の戻り値・引数の型を必ず説明する
- 「なぜこの型なのか」を文章で書く
*/
struct Repository<T> {
    items: Vec<T>,
}

impl<T> Repository<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
    
    fn take(&mut self, index: usize) -> Option<T> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }
    
    fn replace(&mut self, index: usize, item: T) -> Option<T> {
        if index < self.items.len() {
            Some(std::mem::replace(&mut self.items[index], item))
        } else {
            None
        }
    }
    
    fn find<P>(&self, pred: P) -> Option<&T> 
    where
        P:Fn(&T) -> bool,
    {
        self.items.iter().find(|x| pred(x))
    }
    
    fn update<P, F>(&mut self, pred: P, mut f: F)
    where
        P: Fn(&T) -> bool,
        F: FnMut(&mut T),
    {
        if let Some(item) = self.items.iter_mut().find(|x| pred(x)) {
            f(item);
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
    fn new_and_len() {
        let repo = Repository::<i32>::new();
        assert_eq!(repo.len(), 0);
    }

    #[test]
    fn add_and_get() {
        let mut repo = Repository::new();
        repo.add(10);
        repo.add(20);

        assert_eq!(repo.len(), 2);
        assert_eq!(repo.get(0), Some(&10));
        assert_eq!(repo.get(1), Some(&20));
        assert_eq!(repo.get(2), None);
    }

    #[test]
    fn iter_returns_all_items() {
        let mut repo = Repository::new();
        repo.add(1);
        repo.add(2);
        repo.add(3);

        let collected: Vec<i32> = repo.iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn take_removes_and_returns_item() {
        let mut repo = Repository::new();
        repo.add(5);
        repo.add(6);

        let v = repo.take(0);
        assert_eq!(v, Some(5));
        assert_eq!(repo.len(), 1);
        assert_eq!(repo.get(0), Some(&6));
    }

    #[test]
    fn take_invalid_index_returns_none() {
        let mut repo = Repository::new();
        repo.add(1);

        assert_eq!(repo.take(10), None);
        assert_eq!(repo.len(), 1);
    }

    #[test]
    fn replace_swaps_and_returns_old_value() {
        let mut repo = Repository::new();
        repo.add(100);
        repo.add(200);

        let old = repo.replace(1, 300);
        assert_eq!(old, Some(200));
        assert_eq!(repo.get(1), Some(&300));
    }

    #[test]
    fn replace_invalid_index_returns_none() {
        let mut repo = Repository::new();
        repo.add(1);

        assert_eq!(repo.replace(5, 10), None);
        assert_eq!(repo.get(0), Some(&1));
    }

    #[test]
    fn find_returns_reference() {
        let mut repo = Repository::new();
        repo.add(10);
        repo.add(20);
        repo.add(30);

        let found = repo.find(|x| *x == 20);
        assert_eq!(found, Some(&20));
    }

    #[test]
    fn find_returns_none_if_not_found() {
        let mut repo = Repository::new();
        repo.add(1);
        repo.add(2);

        let found = repo.find(|x| *x == 99);
        assert_eq!(found, None);
    }

    #[test]
    fn update_modifies_matching_items() {
        let mut repo = Repository::new();
        repo.add(1);
        repo.add(2);
        repo.add(3);

        repo.update(
            |x| *x % 2 == 1,
            |x| *x *= 10,
        );

        let values: Vec<i32> = repo.iter().copied().collect();
        assert_eq!(values, vec![10, 2, 3]);
    }

    // ---- 設計確認用（コンパイル不可な例） ----

    /*
    #[test]
    fn invalid_design_example() {
        let mut repo = Repository::new();
        repo.add(1);

        let r = repo.get(0); // &T を保持
        repo.add(2);         // &mut self が必要 → 借用衝突

        println!("{:?}", r);
    }
    */
}