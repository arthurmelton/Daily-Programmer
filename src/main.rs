use curl::easy::Easy;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde_json::{json, Value, from_str};
use std::ops::Index;
use std::thread::sleep;

fn main() {
    let mut dates:Vec<String> = Vec::new();
    let mut challenge:Vec<String> = Vec::new();
    let mut difficult:Vec<String> = Vec::new();
    let mut time = 0;
    
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url(&["https://api.pushshift.io/reddit/search/submission/?subreddit=dailyprogrammer&sort=desc&sort_type=created_utc&after=", &time.to_string(), "&before=", SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis().to_string().as_str(),"&size=1000"].join("")).unwrap();
    let _output = easy.custom_request("GET");
    let mut transfer = easy.transfer();
    transfer
        .write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })
        .unwrap();
    transfer.perform().unwrap();
    drop(transfer);
    
    let v: Value = serde_json::from_str(&dst.iter().map(|&c| c as char).collect::<String>().as_str()).unwrap();
    let sub_value: Vec<Value> = serde_json::from_value(v["data"].clone()).unwrap();
    for item in &sub_value {
        println!("{:?}", name(item["title"].to_string()));
    }
}

fn name(name:String) -> Vec<String> {
    vec![name.clone().split("[").nth(1).unwrap().split("]").nth(0).unwrap().to_string(), name.clone().split("#").nth(1).unwrap().split("[").nth(0).unwrap().trim().to_string(), name.clone().split("[").nth(2).unwrap().split("]").nth(0).unwrap().trim().to_string(), name.clone().split("]").nth(2).unwrap().trim().to_string()]
}