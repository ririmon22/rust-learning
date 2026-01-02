/*
課題030：ゼロコスト抽象化体験

目的：
- ジェネリクス（T）と trait object（dyn）の違いを体験する
- 「同じ処理」を2通りで書き、設計とコストの違いを説明できるようにする
- Rust が言う「ゼロコスト抽象化」の意味を言語化する

制約：
- 同じ Describable trait を使う
- ジェネリクス版と dyn 版の両方を書く
- clone は使わない
*/
trait Describable {
    fn describe(&self) -> String;
}

struct User {
    name: String,
}

struct Product {
    id: u32,
}
    
impl Describable for User {
    fn describe(&self) -> String {
        format!("User(name={})", self.name)
    }
}

impl Describable for Product {
    fn describe(&self) -> String {
        format!("Product(id={})", self.id)
    }
}

fn log_all_generic<T>(items: &[T])
where 
    T: Describable,
{
    for item in items{
        println!("{}", item.describe());
    }
}

fn log_all_dyb(items: &[Box<dyn Describable>]) {
    for item in items {
        println!("{}", item.describe());
    }
}
fn main() {
    
    let mixed: Vec<Box<dyn Describable>> = vec![
        Box::new(User { name: "Alice".into()}),
        Box::new(Product{ id: 1}),
        Box::new(User { name: "Bob".into()}),
    ];
    let users = vec![
        User { name: "Alice".into()},
        User { name: "Bob".into()},
    ];

    let products = vec![
        Product{ id: 1},
        Product{ id: 2},
    ];
    
    log_all_generic(&users);
    log_all_generic(&products);
    
    log_all_dyb(&mixed);
}
