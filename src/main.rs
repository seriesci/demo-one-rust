extern crate reqwest;

use reqwest::header::AUTHORIZATION;
use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("failed to execute process");

    let sha = str::from_utf8(&output.stdout).unwrap();

    let mut map = HashMap::new();
    map.insert("sha", sha);
    map.insert("value", "42 loc");

    let token = env::var("SERIESCI_TOKEN").unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post("https://seriesci.com/seriesci/demo-one-rust")
        .header(AUTHORIZATION, format!("Token {}", token))
        .json(&map)
        .send()
        .await?;

    println!("{:#?}", res);
    Ok(())
}