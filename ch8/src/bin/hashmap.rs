use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("a", 1);
    scores.insert("b", 2);

    let score = scores.get("a").copied().unwrap_or(0);

    for (k, v) in scores {
        println!("{k}: {v}");
    }

    let key = String::from("key");
    let value = String::from("value");

    let mut temp = HashMap::new();
    temp.insert(key.clone(), value);
    temp.insert(key.clone(), "Not VALUE".to_string());
    temp.entry("Asuna".to_string())
        .or_insert("YUUKI".to_string());

    let confession = "Isn't the moon beautiful today? Isn't it?";
    let mut map = HashMap::new();
    for word in confession.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:#?}");
}
