use curl::easy::Easy;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;
use clap::*;
use crossterm::{QueueableCommand, cursor};
use std::io::{Write, stdout};
use strip_markdown::*;
use std::process;
use rand::Rng;

fn main() {
    let matches = App::new("Daily Programmer")
        .version("1.0")
        .about("Gets you a random challenge")
        .arg(Arg::with_name("Type")
            .short("t")
            .long("type")
            .help("lets the difficulty easy, intermediate, difficult")
            .takes_value(true))
        .arg(Arg::with_name("Number")
            .short("n")
            .long("number")
            .help("gets the challenge number")
            .takes_value(true))
        .get_matches();
    let mut time:i64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis().to_string().parse().unwrap();
    let mut dates: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut number: Vec<String> = Vec::new();
    let mut text: Vec<String> = Vec::new();
    let mut difficult: Vec<String> = Vec::new();
    let mut num = 0;
    let mut stdout = stdout();
    loop {
        stdout.queue(cursor::SavePosition);
        match num {
            0 => {stdout.write(format!("0oo").as_bytes());},
            1 => {stdout.write(format!("o0o").as_bytes());},
            2 => {stdout.write(format!("oo0").as_bytes());},
            _ => {}
        }
        stdout.queue(cursor::RestorePosition);
        stdout.flush();
        num = (num+1)%3;
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
            if item["title"].to_string().trim_matches('\"').chars().nth(0).unwrap() == '[' && item["title"].to_string().matches('[').count() == 2 && item["title"].to_string().matches('#').count() == 1 {
                let name_out = name(item["title"].to_string().trim_matches('\"').to_string()).to_owned();
                if name_out[1].to_string() != "1".to_string() {
                    dates.push(name_out[0].clone());
                    difficult.push(name_out[2].clone());
                    number.push(name_out[1].clone());
                    names.push(name_out[3].clone());
                    text.push(item["selftext"].to_string().trim_matches('\"').to_string());
                } else {
                    dates.push(name_out[0].clone());
                    difficult.push(name_out[2].clone());
                    number.push(name_out[1].clone());
                    names.push(name_out[3].clone());
                    text.push(item["selftext"].to_string().trim_matches('\"').to_string());
                    break;
                }
            }
        }
        if sub_value.last().ok_or(true).is_ok() == true {
            time = sub_value.last().unwrap()["created_utc"].to_string().parse::<i64>().unwrap() - 1;
        }
        else {
            break;
        }
    }
    let mut delete = Vec::new();
    if matches.is_present("Type") {
        for x in 0..difficult.len() {
            if difficult[x] != matches.value_of("Type").unwrap() {
                delete.push(x);
            }
        }
    }
    let mut deleted = 0;
    for x in delete {
        difficult.remove(x-deleted);
        number.remove(x-deleted);
        names.remove(x-deleted);
        text.remove(x-deleted);
        dates.remove(x-deleted);
        deleted += 1;
    }
    let mut delete = Vec::new();
    if matches.is_present("Number") {
        for x in 0..number.len() {
            if number[x] != matches.value_of("Number").unwrap() {
                delete.push(x);
            }
        }
    }
    let mut deleted = 0;
    for x in delete {
        difficult.remove(x-deleted);
        number.remove(x-deleted);
        names.remove(x-deleted);
        text.remove(x-deleted);
        dates.remove(x-deleted);
        deleted += 1;
    }
    if number.len() == 0 {
        println!("I have not found anything of what you wanted try and make your search more simple");
        process::exit(1);
    }
    
    let x:usize = rand::thread_rng().gen_range(0..number.len());
    println!("{}", x);
    println!("   [{}] Challenge #{} [{}] {}   ", dates[x].clone(), number[x].clone(), difficult[x].clone(), names[x].clone());
    let mut line = "".to_string();
    for x in 0..["   [", dates[x].as_str(), "] Challenge #", number[x].as_str(), " [", difficult[x].as_str(), "] ", names[x].as_str(), "   "].join("").to_string().len() {
        line.push_str("-");
    }
    println!("{}", line);
    println!("{}",  strip_markdown(&text[x]).replace("\\n", "\n"));
}

fn name(name:String) -> Vec<String> {
    vec![name.clone().split("[").nth(1).unwrap().split("]").nth(0).unwrap().to_string(), name.clone().split("#").nth(1).unwrap().split("[").nth(0).unwrap().trim().to_string(), name.clone().split("[").nth(2).unwrap().split("]").nth(0).unwrap().trim().to_string(), name.clone().split("]").nth(2).unwrap().trim().to_string()]
}