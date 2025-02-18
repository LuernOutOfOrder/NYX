pub fn parse_todo(todo_string: String) -> Vec<String> {
    let todo_decoded: Vec<String> =
        serde_json::from_str(&todo_string).expect("Failed to deserialize todo");
    todo_decoded
}

pub fn stringify_todo(todo: Vec<String>) -> String {
    let todo_string = serde_json::to_string(&todo).expect("Failed to serialize todo");
    todo_string
}
