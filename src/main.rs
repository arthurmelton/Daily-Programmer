use curl::easy::Easy;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;

fn main() {
    let mut time:i64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis().to_string().parse().unwrap();
    let mut dates:Vec<&str> = Vec::new();
    let mut stop = false;
    while !stop {
        let mut dst = Vec::new();
        let mut easy = Easy::new();
        easy.url(&["https://api.pushshift.io/reddit/search/submission/?subreddit=dailyprogrammer&sort=desc&sort_type=created_utc&after=0&before=", time.to_string().as_str(), "&size=1000"].join("")).unwrap();
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
            if item["title"].to_string().chars().nth(1).unwrap() == '[' {
                let name_out = name(item["title"].to_string()).to_owned();
                dates.push(name_out[0].as_str());
                if !dates.contains(&name_out[0].as_str()) {
                    println!("{:?}", name_out);
                } else {
                    stop = true;
                }
            }
        }
        time = v["data"][0]["created_utc"].to_string().parse::<i64>().unwrap() - 1;
    }
}

fn name(name:String) -> Vec<String> {
    vec![name.clone().split("[").nth(1).unwrap().split("]").nth(0).unwrap().to_string(), name.clone().split("#").nth(1).unwrap().split("[").nth(0).unwrap().trim().to_string(), name.clone().split("[").nth(2).unwrap().split("]").nth(0).unwrap().trim().to_string(), name.clone().split("]").nth(2).unwrap().trim().to_string()]
}