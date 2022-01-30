mod notion;

fn main() {
    let database = notion::Database::new(String::from("dummy"));
    let result = database.query(String::from("hoge"), vec![], String::from("dummy"));
    // match result {
    // Ok() =>
    // }
    let client = notion::client::Default::new(String::from("hoge"));
}
