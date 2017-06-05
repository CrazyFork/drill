use std::collections::HashMap;

extern crate yaml_rust;
use self::yaml_rust::Yaml;

extern crate serde_json;
use self::serde_json::Value;

#[derive(Clone)]
pub struct Assign {
  name: String,
  pub assign: Option<String>,
}

impl Assign {
  pub fn is_that_you(item: &Yaml) -> bool{
    item["assign"].as_hash().is_some()
  }

  fn new(item: &Yaml, with_item: Option<Yaml>) -> Assign {
    let reference: Option<&str> = item["assign"].as_str();

    Assign {
      name: item["name"].as_str().unwrap().to_string(),
      assign: reference.map(str::to_string)
    }
  }

  fn execute(&mut self, base_url: &String, context: &mut HashMap<&str, Yaml>, _responses: &HashMap<String, Value>) {
    context.insert("foo", yaml_rust::Yaml::String("bar".to_string()));
  }
}
