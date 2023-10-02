use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use tabled::{builder::Builder, settings::Style};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
   #[arg(short, long, default_value_t = 1000)]
   threshold: u32,
   #[arg(short, long, default_value = "/var/log/vault/audit.log")]
   path: String,
}

#[derive(Deserialize, Debug)]
struct Entry {
    time: String,
    r#type: String,
    request: Request,
}

#[derive(Deserialize, Debug)]
struct Request {
    id: String,
    client_id: Option<String>,
    operation: String,
    mount_type: Option<String>,
    path: String,
    remote_address: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut scores: HashMap<String, u32> = HashMap::new();
    let file = File::open(args.path).expect("couldn't open file");

    let mut total: u64 = 0;
    for line in BufReader::new(file).lines() {
        let line = line.expect("couldn't get line");
        let my_item: Entry = serde_json::from_str(&line).expect("couldn't deserialize");
        let count = scores.entry(my_item.request.path).or_insert(0);
        *count += 1;
        total += 1;
    }

    
    let mut builder = Builder::new();
    builder.
        set_header(["Path", "Request Count"]);
    for (path, count) in scores {
        if count > args.threshold.into() {
            builder.push_record([path, count.to_string()]); 
        }
    }

    let mut table = builder.build();
    table.with(Style::ascii_rounded());

    println!("{}", table);
}

