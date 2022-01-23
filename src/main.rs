mod notion;

fn main() {
    let notion_client = notion::NotionClient::new(String::from("dummy"));
    let result = notion_client.query_database(
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
