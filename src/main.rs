extern crate clap;
use clap::{App, Arg, ArgMatches};
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

	validate_cli(matches);

	println!("Yay");
}

fn validate_cli(matches: ArgMatches) {
	if let Some(j) = matches.value_of("in_json") {
		let f = File::open(j);

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
}
