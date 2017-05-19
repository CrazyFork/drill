use std::thread;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

  let base_url = config_doc["base_url"].as_str().unwrap();
  let port = config_doc["port"].as_i64().unwrap();

  println!("Threads {}", n_threads);
  println!("Iterations {}", n_iterations);
  println!("Base URL {}", base_url);
  println!("Port {}", port);

  let benchmark_docs = YamlLoader::load_from_str(benchmark_file.as_str()).unwrap();
  let benchmark_doc = &benchmark_docs[0];

  println!("Request {}", benchmark_doc.as_vec().unwrap().len());

  let mut children = vec![];

  for i in 0..n_threads {
    children.push(thread::spawn(move || {
      for j in 0..n_iterations {
        // TODO: read benchmark_doc
        println!("this is thread number {} iteration {}", i, j)
      }
    }));
  }

  for child in children {
    // Wait for the thread to finish. Returns a result.
    let _ = child.join();
  }

  let client = Client::new();

  let response = client.get(base_url).send().unwrap();

  println!("< status code: {}", response.status);
}
