extern crate yaml_rust;
use self::yaml_rust::Yaml;

use actions::Request;

use reader;

pub fn is_that_you(item: &Yaml) -> bool{
  item["request"].as_hash().is_some() &&
    item["with_items_from_csv"].as_str().is_some()
}

pub fn expand(item: &Yaml, list: &mut Vec<Request>) {
  let with_items_from_csv_option = item["with_items_from_csv"].as_str();

  if with_items_from_csv_option.is_some() {
    let with_items_path = with_items_from_csv_option.unwrap();
    let with_items_file = reader::read_csv_file_as_yml(with_items_path);

    for with_item in with_items_file {
      list.push(Request::new(item, Some(with_item)))
    }
  }
}
