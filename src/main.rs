// 定义 JSON 数据结构
#[derive(Debug)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),           // 数组：使用动态数组 Vec，自动扩容
    Object(Vec<(String, JsonValue)>), // 对象：键值对列表
}

fn main() {
    // 模拟构建一个复杂的 JSON 对象
    // 对应 C 语言：cJSON_CreateObject, cJSON_AddString, cJSON_AddItemToArray
    let data = JsonValue::Object(vec![
        ("name".to_string(), JsonValue::String("Tom".to_string())),
        ("age".to_string(), JsonValue::Number(18.0)),
        ("is_hacker".to_string(), JsonValue::Bool(true)),
        ("skills".to_string(), JsonValue::Array(vec![
            JsonValue::String("C".to_string()),
            JsonValue::String("Rust".to_string()),
            JsonValue::String("Linux".to_string()),
        ])),
    ]);

    // 打印结果
    println!("Successfully built JSON in Rust:\n{:#?}", data);
}