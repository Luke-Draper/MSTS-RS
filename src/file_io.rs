use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn get_file_contents(path: &str) -> String {
	let file = File::open(path).expect("Failed to open file passed to get_file_contents");
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader
		.read_to_string(&mut contents)
		.expect("Failed to read file opened by get_file_contents");
	return contents;
}

pub fn get_file_paths_in_folder(path: &str) -> Vec<String> {
	let folder = read_dir(path).expect("Failed to open folder passed to get_file_paths_in_folder");
	let mut output: Vec<String> = Vec::new();
	for entry in folder {
		let entry = entry.expect("Error opening entry");
		if entry
			.metadata()
			.expect("Error opening entry metadata")
			.is_file()
		{
			output.push(String::from(entry.path().to_str().unwrap()));
		}
	}
	return output;
}

pub fn get_all_files_contents(paths: &Vec<String>) -> Vec<String> {
	let mut output: Vec<String> = Vec::new();
	for path in paths {
		output.push(get_file_contents(path.as_str()));
	}
	return output;
}

pub fn path_to_base_name(path: &String) -> String {
	return String::from(
		Path::new(path.as_str())
			.file_stem()
			.unwrap()
			.to_str()
			.unwrap(),
	);
}

pub fn all_paths_to_base_names(paths: &Vec<String>) -> Vec<String> {
	let mut output: Vec<String> = Vec::new();
	for path in paths {
		output.push(path_to_base_name(path));
	}
	return output;
}

pub fn map_files_to_contents(files: Vec<String>, contents: Vec<String>) -> HashMap<String, String> {
	let mut output: HashMap<String, String> = HashMap::new();
	for x in 0..contents.len() {
		if !contents.get(x).unwrap().is_empty() && !files.get(x).unwrap().is_empty() {
			output.insert(
				String::from(files.get(x).unwrap().as_str()),
				String::from(contents.get(x).unwrap().as_str()),
			);
		}
	}
	return output;
}
