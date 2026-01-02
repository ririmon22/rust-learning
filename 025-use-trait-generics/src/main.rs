/*
課題025：trait を使う（ジェネリクス）

目的：
- trait を「使う側」として理解する
- T が「型そのもの」であることを体感する
- T: Trait が「能力の制約」であることを説明できるようにする

制約：
- ジェネリクス（<T>）を使う
- dyn / trait object は使わない
- 所有権は動かさない（参照で受け取る）
- trait を実装していない型は受け取れないことを確認する

確認事項：
- 同じ関数に User と Product を渡せる
- 同じ関数に i32 は渡せない（コンパイルエラー）
- なぜ &T で受け取るのか説明できる
*/
trait Describable{
    fn describe(&self) -> String;
}

fn print_description<T: Describable>(x: &T) {
    println!("{}",x.describe());
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
    
    print_description(&user);
    print_description(&product);
}
