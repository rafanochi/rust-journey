use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
// #[serde(tag = "type", content = "c")]
#[serde(untagged)]
enum Message {
    Request { id: String, method: String },
    Response { id: String },
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Data {
    Integer(u64),
    Pair(String, String),
}

#[derive(Deserialize, Debug)]
struct Request {
    #[serde(default = "default_resource")]
    resource: String,

    #[serde(default)]
    timeout: Timeout,

    #[serde(default = "Priority::lowest")]
    priority: Priority,
}

fn default_resource() -> String {
    "/".to_string()
}

/// Timeout in seconds.
#[derive(Deserialize, Debug)]
struct Timeout(u32);
impl Default for Timeout {
    fn default() -> Self {
        Timeout(30)
    }
}

#[derive(Deserialize, Debug)]
enum Priority {
    ExtraHigh,
    High,
    Normal,
    Low,
    ExtraLow,
}
impl Priority {
    fn lowest() -> Self {
        Priority::ExtraLow
    }
}

#[derive(Serialize, Deserialize)]
struct Pagination {
    limit: u64,
    offset: u64,
    total: u64,
}

#[derive(Serialize, Deserialize)]
struct Page {
    // #[serde(flatten)]
    pagination: Pagination,
}

fn main() {
    let msg = Message::Response {
        id: "asdfasd".to_string(),
    };

    let data = Data::Integer(0);

    let serialized = serde_json::to_string(&msg).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Message = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let sd = serde_json::to_string(&data).unwrap();
    println!("serialized = {}", sd);

    let page = Page {
        pagination: Pagination {
            limit: 0,
            offset: 0,
            total: 10,
        },
    };

    let pd = serde_json::to_string(&page).unwrap();
    println!("serialized = {}", pd);
}
