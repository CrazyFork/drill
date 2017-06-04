use std::thread;
use std::collections::HashMap;

extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml};

extern crate colored;
use self::colored::*;

extern crate serde_json;
use self::serde_json::Value;

extern crate hyper;
use self::hyper::client::{Client, Response};

extern crate time;

use interpolator;
use reader;
use actions;

#[derive(Clone)]
pub struct Benchmark {
  list: Vec<BenchmarkItem>
}

impl Benchmark {
  pub fn new(path: &str) -> Benchmark {
    let benchmark_file = reader::read_file(path);
    let docs = YamlLoader::load_from_str(benchmark_file.as_str()).unwrap();
    let doc = &docs[0];
    let items = doc.as_vec().unwrap();

    let mut list = Vec::new();

    for item in items {
      let with_items_option = item["with_items"].as_vec();
      let with_items_from_csv_option = item["with_items_from_csv"].as_str();

      println!("Assign: {}", actions::Assign::is_that_you(&item));
      println!("Request: {}", actions::Request::is_that_you(&item));
      println!("MultiRequest: {}", actions::MultiRequest::is_that_you(&item));
      println!("MultiCSVRequest: {}\n", actions::MultiCSVRequest::is_that_you(&item));

      if with_items_option.is_some() {
        let with_items = with_items_option.unwrap().clone();

        for with_item in with_items {
          list.push(BenchmarkItem::new(item, Some(with_item)))
        }
      } else if with_items_from_csv_option.is_some() {
        let with_items_path = with_items_from_csv_option.unwrap();
        let with_items_file = reader::read_csv_file_as_yml(with_items_path);

        for with_item in with_items_file {
          list.push(BenchmarkItem::new(item, Some(with_item)))
        }
      } else {
        list.push(BenchmarkItem::new(item, None));
      }
    }

    Benchmark{
      list: list
    }
  }

  pub fn execute(&self, threads: i64, iterations: i64, base_url: String) {
    let mut children = vec![];

    for _i in 0..threads {
      let base_url_clone = base_url.to_owned();
      let mut benchmark_clone = self.list.clone();
      let self_clone = self.clone();

      children.push(thread::spawn(move || {
        for _j in 0..iterations {
          let mut responses:HashMap<String, Value> = HashMap::new();
          let mut context:HashMap<&str, Yaml> = HashMap::new();

          for mut item in &mut benchmark_clone {
            let mut response = item.execute(&base_url_clone, &mut context, &responses);

            self_clone.assign_response(&item, &mut response, &mut responses)
          }
        }
      }));
    }

    for child in children {
      // Wait for the thread to finish. Returns a result.
      let _ = child.join();
    }
  }

  fn assign_response(&self, item: &BenchmarkItem, _response: &mut Response, _responses: &mut HashMap<String, Value>) {
    if item.assign.is_some() {
      // let mut data = String::new();
      // let ref option = item.assign;
      // let kaka = option.unwrap();

      // response.read_to_string(&mut data).unwrap();

      // let value: Value = serde_json::from_str(&data).unwrap();

      // responses.insert(kaka, value);
    }
  }
}

#[derive(Clone)]
struct BenchmarkItem {
  name: String,
  url: String,
  time: f64,
  pub with_item: Option<Yaml>,
  pub assign: Option<String>,
}

impl BenchmarkItem {
  fn new(item: &Yaml, with_item: Option<Yaml>) -> BenchmarkItem {
    let reference: Option<&str> = item["assign"].as_str();

    BenchmarkItem {
      name: item["name"].as_str().unwrap().to_string(),
      url: item["request"]["url"].as_str().unwrap().to_string(),
      time: 0.0,
      with_item: with_item,
      assign: reference.map(str::to_string)
    }
  }

  fn execute(&mut self, base_url: &String, context: &mut HashMap<&str, Yaml>, responses: &HashMap<String, Value>) -> Response {
    if self.with_item.is_some() {
      context.insert("item", self.with_item.clone().unwrap());
    }

    let interpolator = interpolator::Interpolator::new(&base_url, &context, &responses);

    let final_url = interpolator.resolve(&self.url);

    let response = self.send_request(&final_url);

    // println!("{:width$} {} {} {:?}", self.name.green(), final_url.blue().bold(), response.status.to_string().yellow(), self.with_item, width=25);

    println!("{:width$} {} {} {}{}", self.name.green(), final_url.blue().bold(), response.status.to_string().yellow(), (self.time * 1000.0).round().to_string().cyan(), "ms".cyan(), width=25);

    response
  }

  fn send_request(&mut self, url: &str) -> Response {
    let client = Client::new();
    let begin = time::precise_time_s();

    let response = client.get(url).send();

    if let Err(e) = response {
      panic!("Error connecting '{}': {:?}", url, e);
    }

    self.time = time::precise_time_s() - begin;

    response.unwrap()
  }
}
