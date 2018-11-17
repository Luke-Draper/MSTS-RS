extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use crate::file_io::get_file_contents;

#[derive(Serialize, Deserialize)]
pub struct JSONSource {
	pub tasks: Vec<SourceTask>,
}

#[derive(Serialize, Deserialize)]
pub struct SourceTask {
	pub targets: Vec<SourceTarget>,
	pub variables: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct SourceTarget {
	pub templates: Vec<String>,
	pub destination: String,
}

fn parse_source_json(data: &str) -> JSONSource {
	let output: JSONSource = serde_json::from_str(data).expect("Failed to deserialize input JSON");
	return output;
}

pub fn get_source_json(path: &str) -> JSONSource {
	return parse_source_json(get_file_contents(path).as_str());
}
