use clipboard::{ClipboardContext, ClipboardProvider};
use serde_json::{Value, json};

fn main() {
    // 读取剪贴板内容
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let input_json = ctx.get_contents().unwrap();

    // 解析 JSON
    let json_value: Value = serde_json::from_str(&input_json).unwrap();

    // 排序和格式化 JSON
    let sorted_json = sort_json(&json_value);
    let formatted_json = serde_json::to_string_pretty(&sorted_json).unwrap();

    // 将结果写回剪贴板
    ctx.set_contents(formatted_json).unwrap();
}

fn sort_json(json_value: &Value) -> Value {
    match json_value {
        Value::Object(map) => {
            let mut sorted_map = map.clone();
            for (_, value) in sorted_map.iter_mut() {
                *value = sort_json(value);
            }
            Value::Object(sorted_map)
        }
        Value::Array(array) => {
            let mut sorted_array = array.clone();
            for value in sorted_array.iter_mut() {
                *value = sort_json(value);
            }
            sorted_array.sort_unstable_by(|a, b| a.to_string().cmp(&b.to_string()));
            Value::Array(sorted_array)
        }
        _ => json_value.clone(),
    }
}
