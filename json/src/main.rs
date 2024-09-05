use serde_json::json;

fn main() {
    let hanako = json!({
        "name": "hanako",
        "age": 8,
        "favorites": {
            "food": ["apple", "melon"],
        }
    });
    println!("hanako debug: {:?}", hanako);
    println!("hanako[\"name\"]: {}", hanako["name"]);
    println!("hanako[\"name\"].as_str().unwrap(): {}", hanako["name"].as_str().unwrap());

    let mut taro = json!({});
    taro["name"] = json!("taro");
    taro["age"] = json!(10);
    taro["favorites"] = json!({"food": ["cake"]});

    let mut members = json!({});
    members["members"] = json!([hanako, taro]);
    println!("JSON: {}", members.to_string());
}
