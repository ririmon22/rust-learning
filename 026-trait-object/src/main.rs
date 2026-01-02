/*
課題026：trait object（dyn）

目的：
- ジェネリクス（T）と trait object（dyn）の違いを理解する
- 「実行時に型が決まる」設計を体験する

制約：
- dyn Describable を使う
- ジェネリクス（<T>）は使わない
- Box を使う
*/
trait Describable{
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
        format!("Product(id={})",  self.id)
    }
}
fn main() {
    let list: Vec<Box<dyn Describable>> = vec![
        Box::new(User {name: "Alice".into()}),
        Box::new(Product {id: 42}),
    ];

    for item in list {
        println!("{}", item.describe());
    }
}
