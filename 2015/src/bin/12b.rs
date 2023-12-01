use anyhow::Result;
use serde_json::Value;

fn walk(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(a) => a.iter().map(walk).sum(),
        Value::Object(o) => o
            .values()
            .try_fold(0, |sum, v| match v {
                Value::String(s) if s == "red" => None,
                v => Some(sum + walk(v)),
            })
            .unwrap_or(0),
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/12.txt")?;
    let input: Value = serde_json::from_str(&input)?;
    println!("{}", walk(&input));
    Ok(())
}
