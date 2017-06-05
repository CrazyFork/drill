extern crate yaml_rust;
use self::yaml_rust::Yaml;

use actions::Request;

pub trait Expandable {
  fn expand(item: &Yaml, list: &mut Vec<Request>);
}
