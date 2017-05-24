use std::thread;
use std::collections::HashMap;
use yaml_rust::{YamlLoader, Yaml};

extern crate colored;
use colored::*;

extern crate serde_json;
use serde_json::Value;

extern crate hyper;
use self::hyper::client::{Client, Response};

use interpolator;
use reader;

pub struct Benchmark {
  list: Vec<Box<BenchmarkItem>>
}

impl Benchmark {
  pub fn new(path: &str) -> Benchmark {
    let benchmark_file = reader::read_file(path);
    let docs = YamlLoader::load_from_str(benchmark_file.as_str()).unwrap();
    let doc = &docs[0];
    let items = doc.as_vec().unwrap();

    let mut list = Vec::new();

    for item in items {
      list.push(Box::new(BenchmarkItem::new(item)));
    }

    Benchmark{
      list: list
    }
  }

  pub fn execute(&self, threads: i64, iterations: i64) {
    let mut children = vec![];

    for _i in 0..threads {
      children.push(thread::spawn(move || {
        for _j in 0..iterations {
          println!("Thread!");
        }
      }));
    }

    for child in children {
      // Wait for the thread to finish. Returns a result.
      let _ = child.join();
    }
  }
}

struct BenchmarkItem {
  name: String,
  url: String
}

impl BenchmarkItem {
  fn new(item: &Yaml) -> BenchmarkItem {
    BenchmarkItem {
      name: item["name"].as_str().unwrap().to_string(),
      url: item["request"]["url"].as_str().unwrap().to_string()
    }
  }

  fn execute(&self, base_url: &String, context: &HashMap<&str, String>, responses: &HashMap<String, Value>) -> Response {
    let result = interpolator::resolve_interpolations(&self.url, &context, &responses);

    let final_url = base_url.to_string() + &result;

    let response = self.send_request(&final_url);

    println!("{:width$} {} {}", self.name.green(), final_url.blue().bold(), response.status.to_string().yellow(), width=25);

    response
  }

  fn send_request(&self, url: &str) -> Response {
    let client = Client::new();

    client.get(url).send().unwrap()
  }
}
