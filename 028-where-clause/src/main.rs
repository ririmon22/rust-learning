/*
課題028：where 句ノック

目的：
- ジェネリクス境界を where 句で書けるようになる
- 「意味は同じで、書き方が違う」ことを理解する
- 境界が増えたときの読みやすさを体感する

制約：
- where 句を必ず使う
- dyn / trait object は使わない
- 027 と同じ振る舞いを実現する
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

fn log_twice<T>(x: &T) 
where 
    T: Describable,
{
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