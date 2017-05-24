use std::io::prelude::*;
use std::collections::HashMap;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};

extern crate serde_json;
use serde_json::Value;

extern crate hyper;
use hyper::client::{Client, Response};

extern crate colored;
use colored::*;

mod config;
mod interpolator;
mod benchmark;
mod reader;

fn send_request(url: &str) -> Response {
  let client = Client::new();

  client.get(url).send().unwrap()
}

fn run_benchmark_item(base_url: &String, benchmark_item: &Yaml, context: &HashMap<&str, String>, responses: &HashMap<String, Value>) -> Response {
  let benchmark_item_name = benchmark_item["name"].as_str().unwrap();

  let benchmark_item_url = benchmark_item["request"]["url"].as_str().unwrap();
  let result = interpolator::resolve_interpolations(benchmark_item_url, &context, &responses);

  let final_url = base_url.to_string() + &result;

  let response = send_request(&final_url);

  println!("{:width$} {} {}", benchmark_item_name.green(), final_url.blue().bold(), response.status.to_string().yellow(), width=25);

  response
}

fn add_assign_response(benchmark_item: &Yaml, response: &mut Response, responses: &mut HashMap<String, Value>) {
  let assign = benchmark_item["assign"].as_str();

  if assign.is_some() {
    let mut data = String::new();

    response.read_to_string(&mut data).unwrap();

    let value: Value = serde_json::from_str(&data).unwrap();

    responses.insert(assign.unwrap().to_string(), value);
  }
}

fn warn_multiple_assign(benchmark_item: &Yaml){
  let assign = benchmark_item["assign"].as_str();

  if assign.is_some() {
    println!("{} Assign '{}' is not supported for `with_items` statement!", "WARNING!".yellow().bold(), assign.unwrap());
  }
}

fn main() {
  let mut config2 = config::Config::new();

  println!("Threads {}", config2.threads);
  println!("Iterations {}", config2.iterations);
  println!("Base URL {}", config2.base_url);

  let mut benchmark2 = benchmark::Benchmark::new();
  benchmark2.load();
  benchmark2.execute(config2.threads, config2.iterations);

//  let mut children = vec![];
//
//  for _i in 0..n_threads {
//    let base_url_clone = base_url.to_owned();
//    let benchmark_clone = benchmark2.list.clone();
//
//    children.push(thread::spawn(move || {
//      for _j in 0..n_iterations {
//
//        // Context objects
//        let mut responses:HashMap<String, Value> = HashMap::new();
//        let mut context:HashMap<&str, String> = HashMap::new();
//
//        for benchmark_item in &benchmark_clone {
//
//          let with_items_option = benchmark_item["with_items"].as_vec();
//          if with_items_option.is_some() {
//            warn_multiple_assign(benchmark_item);
//
//            let with_items = with_items_option.unwrap();
//            for with_item in with_items {
//              context.insert("item", with_item.as_i64().unwrap().to_string());
//
//              run_benchmark_item(&base_url_clone, benchmark_item, &context, &responses);
//            }
//          } else {
//            let mut response = run_benchmark_item(&base_url_clone, benchmark_item, &context, &responses);
//
//            add_assign_response(benchmark_item, &mut response, &mut responses)
//          }
//        }
//      }
//    }));
//  }
//
//  for child in children {
//    // Wait for the thread to finish. Returns a result.
//    let _ = child.join();
//  }
}
