/*
課題024：trait を定義する

目的：
- trait が「能力の表現」であることを理解する
- &self が何を意味するかを説明できるようにする

制約：
- ジェネリクス（<T>）は使わない
- dyn / trait object は使わない
- trait の定義と実装に集中する
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
    let user = User{
        name: "Alice".to_string(),
    };

    let  product = Product{
        id: 10,
    };

    println!("{}", user.describe());
    println!("{}", product.describe());
}
