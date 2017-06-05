extern crate yaml_rust;
use self::yaml_rust::Yaml;

use actions::{Request, Runnable};

pub fn is_that_you(item: &Yaml) -> bool{
  item["request"].as_hash().is_some() &&
  item["with_items"].as_vec().is_some()
}

pub fn expand(item: &Yaml, list: &mut Vec<&Runnable>) {
  let with_items_option = item["with_items"].as_vec();

  if with_items_option.is_some() {
    let with_items = with_items_option.unwrap().clone();

    for with_item in with_items {
      list.push(&Request::new(item, Some(with_item)))
    }
  }
}
