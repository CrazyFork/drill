extern crate colored;
use self::colored::*;

mod config;
mod interpolator;
mod benchmark;
mod reader;
mod actions;
mod expandable;

fn main() {
  let config = config::Config::new("./config.yml");

  println!("{} {}", "Threads".yellow(), config.threads.to_string().purple());
  println!("{} {}", "Iterations".yellow(), config.iterations.to_string().purple());
  println!("{} {}", "Base URL".yellow(), config.base_url.to_string().purple());
  println!("");

  let suite = benchmark::Benchmark::new("./benchmark.yml");
  suite.execute(config.threads, config.iterations, config.base_url);
}
