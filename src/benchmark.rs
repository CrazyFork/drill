use std::thread;
use std::collections::HashMap;

extern crate yaml_rust;
use self::yaml_rust::Yaml;

extern crate serde_json;
use self::serde_json::Value;

extern crate time;

use expandable::include;
use actions::Request;

#[derive(Clone)]
pub struct Benchmark {
  list: Vec<Request>
}

impl Benchmark {
  pub fn new(path: &str) -> Benchmark {
    let mut list = Vec::new();

    include::expand_from_filepath(path, &mut list);

    Benchmark{
      list: list
    }
  }

  pub fn execute(&self, threads: i64, iterations: i64, base_url: String) {
    let mut children = vec![];

    for _ in 0..threads {
      let base_url_clone = base_url.to_owned();
      let mut benchmark_clone = self.list.clone();

      children.push(thread::spawn(move || {
        for _ in 0..iterations {
          let mut responses:HashMap<String, Value> = HashMap::new();
          let mut context:HashMap<&str, Yaml> = HashMap::new();

          for mut item in &mut benchmark_clone {
            item.execute(&base_url_clone, &mut context, &mut responses);
          }
        }
      }));
    }

    for child in children {
      // Wait for the thread to finish. Returns a result.
      let _ = child.join();
    }
  }
}
