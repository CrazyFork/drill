use std::thread;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;
use regex::Captures;

extern crate yaml_rust;
use yaml_rust::YamlLoader;

extern crate serde_json;
use serde_json::Value;

static NTHREADS: i64 = 3;
static NITERATIONS: i64 = 2;

extern crate hyper;
use hyper::client::{Client, Response};

fn read_file(filepath: &str) -> String {
  // Create a path to the desired file
  let path = Path::new(filepath);
  let display = path.display();

  // Open the path in read-only mode, returns `io::Result<File>`
  let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(file) => file,
  };

  // Read the file contents into a string, returns `io::Result<usize>`
  let mut content = String::new();
  match file.read_to_string(&mut content) {
    Err(why) => panic!("couldn't read {}: {}", display, why),
    Ok(_) => {},
  }

  content
}

fn resolve_interpolations(url:&str, context: &HashMap<&str, &str>, responses: &HashMap<&str, Value>) -> String {
  let re = Regex::new(r"\{\{ *([a-z\.]+) *\}\}").unwrap();
  let result = re.replace(url, |caps: &Captures| {
    let cap_path: Vec<&str> = caps[1].split(".").collect();
    let cap_root = cap_path.get(0).unwrap();

    println!("- cap path {}", cap_path.len() as i32);
    println!("- cap root {}", cap_root);

    match context.get(&caps[1]) {
      Some(value) => value.to_string(),
      _ => {
        match responses.get(&caps[1]) {
          Some(value) => value.to_string(),
          _ => {
            println!("WARNING! Unknown '{}' variable!\n", &caps[1]);
            "".to_string()
          }
        }
      }
    }
  });

  result.to_string()
}

fn send_request(url: &str) -> Response {
  let client = Client::new();

  client.get(url).send().unwrap()
}

fn main() {
  let config_file = read_file("./config.yml");
  let benchmark_file = read_file("./benchmark.yml");

  let config_docs = YamlLoader::load_from_str(config_file.as_str()).unwrap();
  let config_doc = &config_docs[0];

  let n_threads: i64 = match config_doc["threads"].as_i64() {
    Some(value) => value,
    None => {
      println!("Invalid threads value!");

      NTHREADS
    },
  };

  let n_iterations: i64 = match config_doc["iterations"].as_i64() {
    Some(value) => value,
    None => {
      println!("Invalid iterations value!");

      NITERATIONS
    },
  };

  let base_url = config_doc["base_url"].as_str().unwrap().clone();

  println!("Threads {}", n_threads);
  println!("Iterations {}", n_iterations);
  println!("Base URL {}", base_url);

  let benchmark_docs = YamlLoader::load_from_str(benchmark_file.as_str()).unwrap();
  let benchmark_doc = &benchmark_docs[0];
  let benchmark = benchmark_doc.as_vec().unwrap();

  println!("Request {}\n", benchmark.len());

  let mut children = vec![];

  for _i in 0..n_threads {
    let base_url_clone = base_url.to_owned();
    let benchmark_clone = benchmark.clone();

    children.push(thread::spawn(move || {
      for _j in 0..n_iterations {
        let mut responses:HashMap<&str, Value> = HashMap::new();

        let mut context = HashMap::new();
        context.insert("foo.body.id", "2");

        for benchmark_item in &benchmark_clone {

          let benchmark_item_url = benchmark_item["request"]["url"].as_str().unwrap();

          println!("- {}", benchmark_item["name"].as_str().unwrap());

          let with_items = benchmark_item["with_items"].as_vec();
          if with_items.is_some() {
            println!("- Items: {:?}", with_items.unwrap());

            for with_item in &with_items {
              context.insert("item", with_item.to_string());

              let result = resolve_interpolations(benchmark_item_url, &context, &responses);

              let final_url = base_url_clone.to_string() + &result;

              print!("  {} => {} : ", benchmark_item_url, final_url);

              let mut response = send_request(&final_url);

              println!("{}\n", response.status);
            }
          } else {
            let result = resolve_interpolations(benchmark_item_url, &context, &responses);

            let final_url = base_url_clone.to_string() + &result;

            print!("  {} => {} : ", benchmark_item_url, final_url);

            let mut response = send_request(&final_url);

            println!("{}\n", response.status);
          }



          // Post process...
          let assign = benchmark_item["assign"].as_str();

          if assign.is_some() {
            let mut data = String::new();

            response.read_to_string(&mut data).unwrap();

            let value: Value = serde_json::from_str(&data).unwrap();

            responses.insert(assign.unwrap(), value);
          }
        }
      }
    }));
  }

  for child in children {
    // Wait for the thread to finish. Returns a result.
    let _ = child.join();
  }
}
