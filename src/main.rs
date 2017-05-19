use std::thread;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate yaml_rust;
use yaml_rust::YamlLoader;

static NTHREADS: i64 = 10;
static NITERATIONS: i64 = 5;

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

  let mut n_threads: i64 = NTHREADS;
  let mut n_iterations: i64 = NITERATIONS;

  if !config_doc["threads"].is_badvalue() {
    n_threads = config_doc["threads"].as_i64().unwrap();
  }

  if !config_doc["iterations"].is_badvalue() {
    n_iterations = config_doc["iterations"].as_i64().unwrap();
  }

  let base_url = config_doc["base_url"].as_str().unwrap();

  println!("Threads {}", n_threads);
  println!("Iterations {}", n_iterations);
  println!("Base URL {}", base_url);

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
}
