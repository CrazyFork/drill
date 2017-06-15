use std::collections::HashMap;
use std::io::Read;

use yaml_rust::Yaml;
use colored::*;
use serde_json;
use time;

use hyper::client::{Client, Response};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::UserAgent;

use interpolator;

use actions::{Runnable, Report};

static USER_AGENT: &'static str = "drill";

#[derive(Clone)]
pub struct Request {
  name: String,
  url: String,
  time: f64,
  method: String,
  pub body: Option<String>,
  pub with_item: Option<Yaml>,
  pub assign: Option<String>,
}

impl Request {
  pub fn is_that_you(item: &Yaml) -> bool{
    item["request"].as_hash().is_some()
  }

  pub fn new(item: &Yaml, with_item: Option<Yaml>) -> Request {
    let reference: Option<&str> = item["assign"].as_str();
    let body: Option<&str> = item["request"]["body"].as_str();
    let method;

    if let Some(v) = item["request"]["method"].as_str() {
      method = v.to_string().to_uppercase();
    } else {
      method = "GET".to_string()
    }

    Request {
      name: item["name"].as_str().unwrap().to_string(),
      url: item["request"]["url"].as_str().unwrap().to_string(),
      time: 0.0,
      method: method,
      body: body.map(str::to_string),
      with_item: with_item,
      assign: reference.map(str::to_string)
    }
  }

  fn send_request(&self, url: &str) -> (Response, f64) {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let begin = time::precise_time_s();

    let request;

    if self.method == "GET" {
      request = client.get(url);
    } else if self.method == "POST" {
      request = client.post(url).body("foo=bar&arg=0");
    } else {
      panic!("Unknown method '{}'", self.method);
    }

    let response = request
        .header(UserAgent(USER_AGENT.to_string()))
        .send();

    if let Err(e) = response {
      panic!("Error connecting '{}': {:?}", url, e);
    }

    (response.unwrap(), time::precise_time_s() - begin)
  }
}

impl Runnable for Request {
  fn execute(&self, context: &mut HashMap<String, Yaml>, responses: &mut HashMap<String, serde_json::Value>, reports: &mut Vec<Report>) {
    if self.with_item.is_some() {
      context.insert("item".to_string(), self.with_item.clone().unwrap());
    }

    let final_url;

    // Resolve the url
    {
      let interpolator = interpolator::Interpolator::new(&context, &responses);
      final_url = interpolator.resolve(&self.url);
    }

    let (mut response, duration_ns) = self.send_request(&final_url);
    let duration_ms = duration_ns * 1000.0;

    println!("{:width$} {} {} {}{}", self.name.green(), final_url.blue().bold(), response.status.to_string().yellow(), duration_ms.round().to_string().cyan(), "ms".cyan(), width=25);

    reports.push(Report { name: self.name.to_owned(), duration: duration_ms });

    if let Some(ref key) = self.assign {
      let mut data = String::new();

      response.read_to_string(&mut data).unwrap();

      let value: serde_json::Value = serde_json::from_str(&data).unwrap();

      responses.insert(key.to_owned(), value);
    }
  }

}
