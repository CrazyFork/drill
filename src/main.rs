mod config;
mod interpolator;
mod benchmark;
mod reader;

fn main() {
  let config = config::Config::new("./config.yml");

  println!("Threads {}", config.threads);
  println!("Iterations {}", config.iterations);
  println!("Base URL {}", config.base_url);

  let suite = benchmark::Benchmark::new("./benchmark.yml");
  suite.execute(config.threads, config.iterations, config.base_url);
}
