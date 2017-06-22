use yaml_rust::{YamlLoader, Yaml};

//:bm,
use expandable::{multi_request, multi_csv_request, include};
use actions;
use actions::Runnable;

use reader;

pub fn is_that_you(item: &Yaml) -> bool{
  item["include"].as_str().is_some()
}

pub fn expand(item: &Yaml, mut list: &mut Vec<Box<(Runnable + Sync + Send)>>) {
  let path = item["include"].as_str().unwrap();

  expand_from_filepath(path, &mut list, None);
}
/// 将yaml文件中的item解析出来生成对应的 Request 和 Action 对象，放到 list 中
/// @param: accessor, item 的 root 节点，在 benchmark 这个例子里边就是 plan 节点
pub fn expand_from_filepath(path: &str, mut list: &mut Vec<Box<(Runnable + Sync + Send)>>, accessor: Option<&str>) {
  let benchmark_file = reader::read_file(path);

  let docs = YamlLoader::load_from_str(benchmark_file.as_str()).unwrap();
  let doc = &docs[0];
  let items;

  if let Some(accessor_id) = accessor {
    items = doc[accessor_id].as_vec().unwrap();
  } else {
    items = doc.as_vec().unwrap();
  }

  for item in items {
    if multi_request::is_that_you(&item) { // request & with_items
      multi_request::expand(&item, &mut list);
    } else if multi_csv_request::is_that_you(&item) { // request & with_items_from_csv
      multi_csv_request::expand(&item, &mut list);
    } else if include::is_that_you(&item) { // include
      include::expand(&item, &mut list);
    } else if actions::Assign::is_that_you(&item) { // assign
      list.push(Box::new(actions::Assign::new(item, None)));
    } else if actions::Request::is_that_you(&item){ // request
      list.push(Box::new(actions::Request::new(item, None)));
    }
  }
}
