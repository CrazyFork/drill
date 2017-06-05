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
use actions::Request;
use expandable::Expandable;

#[derive(Clone)]
pub struct Benchmark {
  list: Vec<Request>
}

impl Benchmark {
  pub fn new(path: &str) -> Benchmark {
    let benchmark_file = reader::read_file(path);
    let docs = YamlLoader::load_from_str(benchmark_file.as_str()).unwrap();
    let doc = &docs[0];
    let items = doc.as_vec().unwrap();

    let mut list = Vec::new();

    for item in items {
      if actions::MultiRequest::is_that_you(&item) {
        actions::MultiRequest::expand(&item, &mut list);
      } else if actions::MultiCSVRequest::is_that_you(&item) {
        actions::MultiCSVRequest::expand(&item, &mut list);
      } else if actions::Include::is_that_you(&item) {
        // TODO
      } else if actions::Assign::is_that_you(&item) {
        // TODO
      } else if actions::Request::is_that_you(&item){
        list.push(actions::Request::new(item, None));
      }
    }

    Benchmark{
      list: list
    }
  }

  pub fn execute(&self, threads: i64, iterations: i64, base_url: String) {
    let mut children = vec![];

    for _ in 0..threads {
      let base_url_clone = base_url.to_owned();
      let mut benchmark_clone = self.list.clone();
      let self_clone = self.clone();

      children.push(thread::spawn(move || {
        for _ in 0..iterations {
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

  fn assign_response(&self, item: &Request, _response: &mut Response, _responses: &mut HashMap<String, Value>) {
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
pub struct BenchmarkItem {
  name: String,
  url: String,
  time: f64,
  pub with_item: Option<Yaml>,
  pub assign: Option<String>,
}

impl BenchmarkItem {
  pub fn new(item: &Yaml, with_item: Option<Yaml>) -> BenchmarkItem {
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
