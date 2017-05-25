use std::io::prelude::*;
use std::collections::HashMap;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};

extern crate serde_json;
use serde_json::Value;

extern crate colored;
use colored::*;

mod config;
mod interpolator;
mod benchmark;
mod reader;

// fn warn_multiple_assign(benchmark_item: &Yaml){
//   let assign = benchmark_item["assign"].as_str();
// 
//   if assign.is_some() {
//     println!("{} Assign '{}' is not supported for `with_items` statement!", "WARNING!".yellow().bold(), assign.unwrap());
//   }
// }

fn main() {
  let config = config::Config::new("./config.yml");

  println!("Threads {}", config.threads);
  println!("Iterations {}", config.iterations);
  println!("Base URL {}", config.base_url);

  let suite = benchmark::Benchmark::new("./benchmark.yml");
  suite.execute(config.threads, config.iterations, config.base_url);

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
