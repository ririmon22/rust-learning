/*
課題029：Into / From 実装

目的：
- 「変換できる」という能力を trait で表現する
- 所有権が move される変換を体感する
- From を実装すると Into が自動で使えることを理解する

制約：
- From を実装する（Into は直接実装しない）
- clone は使わない
- dyn / trait object は使わない
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

impl From<User> for String {
    fn from(user:User) -> Self {
        user.describe()
    }
}

impl From<Product> for String {
    fn from(product: Product) -> Self {
        product.describe()
    }
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
    };

    // From<User> for String を使う
    let user_desc = String::from(user);
    println!("{}", user_desc);

    // ここで user はもう使えない（所有権が move された）
    // println!("{:?}", user); // ← コンパイルエラーになる

    let product = Product {
        id: 10,
    };

    // Into<String> for Product を使う（From を実装していれば自動で使える）
    let product_desc: String = product.into();
    println!("{}", product_desc);

    // ここでも product は使えない
    // println!("{:?}", product); // ← コンパイルエラー
}