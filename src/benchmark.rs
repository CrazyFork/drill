use std::thread;
use yaml_rust::{YamlLoader, Yaml};

use reader;

pub struct Benchmark {
  list: Vec<Box<BenchmarkItem>>
}

impl Benchmark {
  pub fn new() -> Benchmark {
    Benchmark{
      list: Vec::new()
    }
  }

  pub fn load(&mut self) {
    let benchmark_file = reader::read_file("./benchmark.yml");
    let benchmark_docs = YamlLoader::load_from_str(benchmark_file.as_str()).unwrap();
    let benchmark_doc = &benchmark_docs[0];
    let benchmark_items = benchmark_doc.as_vec().unwrap();

    for benchmark_item in benchmark_items {
      self.list.push(Box::new(BenchmarkItem::new(benchmark_item)));
    }
  }

  pub fn execute(&self, threads: i64, iterations: i64) {
    let mut children = vec![];

    for _i in 0..threads {
      children.push(thread::spawn(move || {
        for _j in 0..iterations {
          println!("Thread!");
        }
      }));
    }

    for child in children {
      // Wait for the thread to finish. Returns a result.
      let _ = child.join();
    }
  }
}

struct BenchmarkItem {
  name: String
}

impl BenchmarkItem {
  fn new(item: &Yaml) -> BenchmarkItem {
    BenchmarkItem {
      name: item["name"].as_str().unwrap().to_string()
    }
  }
}
