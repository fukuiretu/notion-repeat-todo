use notion_repeat_todo_creator::notion::client::{Base, Default};
use notion_repeat_todo_creator::notion::database::Query;

fn main() {
    let client = Default::new(String::from("hoge"));
    let result = Query::new(client).call(String::from("hoge"), String::from("dummy"));
    // match result {
    // Ok() =>
    // }
}
