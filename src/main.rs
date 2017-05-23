use std::thread;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

extern crate yaml_rust;
use yaml_rust::YamlLoader;

static NTHREADS: i64 = 3;
static NITERATIONS: i64 = 2;

extern crate hyper;
use hyper::client::Client;

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

  println!("Request {}", benchmark.len());

  let mut children = vec![];

  for _i in 0..n_threads {
    let base_url_clone = base_url.to_owned();
    let benchmark_clone = benchmark.clone();

    children.push(thread::spawn(move || {
      for _j in 0..n_iterations {
        let mut context = HashMap::new();
        context.insert("Hi", "Hola");

        for benchmark_item in &benchmark_clone {

          let benchmark_item_url = benchmark_item["request"]["url"].as_str().unwrap();
          let final_url = base_url_clone.to_string() + benchmark_item_url;

          let re = Regex::new(r"\{\{(.*)\}\}").unwrap();

          println!("(*) {}", benchmark_item["name"].as_str().unwrap());
          println!("(U) {}", benchmark_item_url);

          for cap in re.captures_iter(benchmark_item_url) {
            println!("Item call? {}", &cap[1] );
          }


          let client = Client::new();
          let response = client.get(&final_url).send().unwrap();
          println!("< status code: {}\n", response.status);
        }
      }
    }));
  }

  for child in children {
    // Wait for the thread to finish. Returns a result.
    let _ = child.join();
  }
}
