/*
課題027：ジェネリクス境界を設計する

目的：
- T: Trait が「能力の制約」であることを理解する
- 複数の能力を要求する設計を体験する
- なぜ境界が必要かを説明できるようにする

制約：
- ジェネリクス（<T>）を使う
- dyn / trait object は使わない
- where 句はまだ使わない（次でやる）
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
fn log_twice<T:Describable>( x: &T) {
    println!("{}", x.describe());
    println!("{}", x.describe());
}
fn main() {
    let user = User {
        name: "Alice".to_string(),
    };

    let product = Product {
        id: 10,
    };

    log_twice(&user);
    log_twice(&product);
}
