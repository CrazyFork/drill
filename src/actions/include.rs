extern crate yaml_rust;
use self::yaml_rust::Yaml;

use expandable;
use self::expandable::Expandable;

use actions::Request;

use reader;

pub struct Include;

impl Include {
  pub fn is_that_you(item: &Yaml) -> bool{
    item["include"].as_str().is_some()
  }
}

impl Expandable for Include {
  fn expand(item: &Yaml, list: &mut Vec<Request>) {
    // TODO
  }
}
