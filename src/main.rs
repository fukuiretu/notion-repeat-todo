mod notion;

fn main() {
    let database = notion::Database::new(String::from("dummy"));
    let result = database.query(
        vec![notion::Header {
            name: String::from("content-type"),
            value: String::from("application/json"),
        }],
        "hoge".to_string(),
    );
    // match result {
    // Ok() =>
    // }
}
