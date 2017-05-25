use std::collections::HashMap;

extern crate regex;
use self::regex::{Regex, Captures};

extern crate serde_json;
use self::serde_json::Value;

extern crate colored;
use self::colored::*;

extern crate yaml_rust;
use self::yaml_rust::Yaml;

pub fn resolve_interpolations(url: &String, context: &HashMap<&str, Yaml>, responses: &HashMap<String, Value>) -> String {
  let re = Regex::new(r"\{\{ *([a-z\.]+) *\}\}").unwrap();

  let result = re.replace(url.as_str(), |caps: &Captures| {
    let cap_path: Vec<&str> = caps[1].split(".").collect();

    let (cap_root, cap_tail) = cap_path.split_at(1);

    match context.get(cap_root[0]) {
      Some(value) => {
        println!("Tail {:?}", cap_tail);

        // if cap_tail.is_empty() {
        //   return value;
        // }

        match &value {
          // Yaml::Hash(v) => "hax".to_string(),
          // Yaml::Integer(v) => "int".to_string(),
          _ => "???".to_string(),
        }
      },
      _ => {
        match responses.get(&caps[1]) {
          Some(value) => "lol".to_string(),
          _ => {
            println!("{} Unknown '{}' variable!", "WARNING!".yellow().bold(), &caps[1]);
            "wat".to_string()
          }
        }
      }
    };

    "hehe".to_string()
  });

  result.to_string()
}
