use notion_repeat_todo_creator::notion::client::{Client, Default};
use notion_repeat_todo_creator::notion::database::Database;

fn main() {
    let database = Database::new(String::from("dummy"));
    let result = database.query(String::from("hoge"), vec![], String::from("dummy"));
    // match result {
    // Ok() =>
    // }
    let client = Default::new(String::from("hoge"));
}
