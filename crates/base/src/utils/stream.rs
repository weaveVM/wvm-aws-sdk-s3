use serde::de::DeserializeOwned;
use std::io::{BufRead, BufReader, Read};
use tokio::io::AsyncReadExt;
use ureq::Body;

pub fn stream_json_lines<T, F>(
    mut body: impl Read,
    mut on_item: F,
) -> Result<Option<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned + Clone,
    F: FnMut(T),
{
    let mut buffer = Vec::new();
    body.read_to_end(&mut buffer)?;

    let raw = std::str::from_utf8(&buffer)?;

    let mut last_item: Option<T> = None;

    for chunk in raw.split("\n\n") {
        for line in chunk.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("data: ") {
                let json_part = &trimmed[6..]; // strip "data: "
                let item: T = serde_json::from_str(json_part)?;
                last_item = Some(item.clone());
                on_item(item);
            }
        }
    }

    Ok(last_item)
}
