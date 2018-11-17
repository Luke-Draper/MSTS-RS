#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_builder;
extern crate clap;
extern crate serde;

mod file_io;
mod json_parser;
mod tasks;
mod template_parser;

use clap::{App, Arg};
use std::fs::{read_dir, File};

fn main() {
	let matches = App::new("msts_rs")
		.version("1.0")
		.author("Luke Draper. <lukepdraper@gmail.com>")
		.about("My Simple Template System - Rust -=- A simple template system to output a bunch of files from the JSON provided.")
		.arg(
			Arg::with_name("in_json")
				.short("j")
				.long("in_json")
				.value_name("IN_JSON")
				.help("Sets the relative filepath to the input JSON file.")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("template_folder")
				.short("t")
				.long("template")
				.value_name("TEMPLATE_FOLDER")
				.help("Sets the relative filepath to the folder containing the template files.")
				.takes_value(true),
		)
		.get_matches();

	let json_path: &str;
	let tmpl_path: &str;

	if let Some(j) = matches.value_of("in_json") {
		let f = File::open(j);
		json_path = j;
		match f {
			Ok(file) => file,
			Err(error) => panic!(
				"There was a problem opening the input json file: {:?}",
				error
			),
		};
	} else {
		panic!("Command line argument -j='json_source_file_path' required. Run again with --help for more information");
	}
	println!("Relative filepath to the input JSON file is valid");

	if let Some(t) = matches.value_of("template_folder") {
		let d = read_dir(t);
		tmpl_path = t;
		match d {
			Ok(directory) => directory,
			Err(error) => panic!(
				"There was a problem opening the input template folder: {:?}",
				error
			),
		};
	} else {
		panic!("Command line argument -t='template_folder_path' required. Run again with --help for more information");
	}
	println!("Relative filepath to the folder containing the template files");

	let test1 = json_parser::get_source_json(json_path);

	// let test2 = template_parser::get_template_map(tmpl_path);

	println!("Output test: {}", "Hi!");
	println!("Output: {}", test1.tasks[0].targets[0].templates[0]);
	println!(
		"Output: {}",
		test1.tasks[0].variables["name"].as_str().unwrap()
	);
	// println!("Output: {:?}", test2);

	// let input = test2.get("test1").unwrap().to_owned();

	// println!("Output: {}", input);
	// let test3 = template_parser::prep_template_for_task_parser(input);
	// println!("Output: {}", test3);

	println!("Yay");
}
