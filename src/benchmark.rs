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

pub struct Benchmark<'_> {
  list: Vec<Box<&'a BenchmarkItem>>
}

impl<'a> Benchmark<'a> {
  pub fn new(path: &str) -> Benchmark<'a> {
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

  pub fn execute(&self, threads: i64, iterations: i64, base_url: String) {
    let mut children = vec![];

    for _i in 0..threads {
      let base_url_clone = base_url.to_owned();
      let benchmark_clone = self.list.clone();

      children.push(thread::spawn(move || {
        for _j in 0..iterations {
          let mut responses:HashMap<String, Value> = HashMap::new();
          let mut context:HashMap<&str, String> = HashMap::new();

          for item in &benchmark_clone {
            let mut response = item.execute(&base_url_clone, &context, &responses);

            self.assign_response(&item, &mut response, &mut responses)
          }
        }
      }));
    }

    for child in children {
      // Wait for the thread to finish. Returns a result.
      let _ = child.join();
    }
  }

  fn assign_response(&self, item: &BenchmarkItem, response: &mut Response, responses: &mut HashMap<String, Value>) {
    if item.assign.is_some() {
      let mut data = String::new();

      response.read_to_string(&mut data).unwrap();

      let value: Value = serde_json::from_str(&data).unwrap();

      responses.insert(item.assign.unwrap().to_string(), value);
    }
  }
}

#[derive(Clone)]
struct BenchmarkItem<'a> {
  name: String,
  url: String,
  pub assign: Option<&'static str>,
}

impl<'a> BenchmarkItem<'a> {
  fn new(item: &Yaml) -> BenchmarkItem<'a> {
    BenchmarkItem {
      name: item["name"].as_str().unwrap().to_string(),
      url: item["request"]["url"].as_str().unwrap().to_string(),
      assign: item["assign"].as_str()
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
