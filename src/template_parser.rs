use crate::file_io;
use crate::tasks;
use std::collections::HashMap;
use std::vec::Vec;
extern crate derive_builder;
extern crate serde_json;

trait TaskFunctionality {
	fn replace_regex();
	fn replace_proxy();
	fn clone_n_and_replace_proxy();
	fn expand_task();
}

#[derive(Builder)]
struct TaskData {
	task_type: String,
	task_header: Vec<&'static str>,
	task_content: Option<String>,
	sub_tasks: Option<Vec<&'static TaskData>>,
}
// let test = TaskDataBuilder::default()
// 	.task_type(String::from("for"))
// 	.task_header(Vec::new())
// 	.build()
// 	.unwrap();

pub struct Template {
	template_name: String,
	template_string: String,
}
/*
impl Template {
	fn run_template(&self, input_vars: serde_json::Value) -> String {
		let mut output = format!("{}", self.template_string);
		while true {
			let sliced: Vec<&str> = output.split(&format!("{}", TASK_OPEN_DELIM)).collect();
			let mut expansion_index: usize = sliced.len();
			let mut expansion_type: &str = "";

			for i in 0..sliced.len() {
				if sliced[i].starts_with(&format!("{}{}", TASK_FOR, TASK_INTERNAL_DELIM)) {
					expansion_index = i;
					expansion_type = &TASK_FOR;
					break;
				}
				if sliced[i].starts_with(&format!("{}{}", TASK_IF, TASK_INTERNAL_DELIM)) {
					expansion_index = i;
					expansion_type = &TASK_IF;
					break;
				}
			}

			if expansion_index == sliced.len() {
				break;
			}

			let expansion_header: Vec<&str> = sliced[expansion_index]
				.split(TASK_CLOSE_DELIM)
				.nth(0)
				.unwrap()
				.split(TASK_INTERNAL_DELIM)
				.collect();
			let expansion_id: &str = expansion_header[1];
			let mut expansion_end_index: usize = sliced.len();

			for i in (expansion_index + 1)..sliced.len() {
				if sliced[i].starts_with(&format!(
					"{}{}{}",
					expansion_type, TASK_INTERNAL_DELIM, expansion_id
				)) {
					expansion_end_index = i;
					break;
				}
			}

			if expansion_end_index == sliced.len() {
				panic!(
					"Unable to locate closing {} tag with id: {}",
					expansion_type, expansion_id
				);
			}

			let mut pre_expansion_content: &str = "";
			for i in 0..expansion_index {
				pre_expansion_content =
					&format!("{}{}{}", pre_expansion_content, TASK_OPEN_DELIM, sliced[i])
			}

			let mut expansion_content: &str = sliced[expansion_index]
				.split(TASK_CLOSE_DELIM)
				.nth(0)
				.unwrap();
			for i in (expansion_index + 1)..expansion_end_index {
				expansion_content = &format!("{}{}{}", expansion_content, TASK_OPEN_DELIM, sliced[i])
			}

			let mut post_expansion_content: &str = "";
			for i in (expansion_end_index + 1)..sliced.len() {
				post_expansion_content =
					&format!("{}{}{}", post_expansion_content, TASK_OPEN_DELIM, sliced[i])
			}

			let mut expanded_content: &str = "";
			if expansion_type == TASK_FOR {
				let mut start_num: usize = 0;
				let mut end_num: usize = 0;
				let proxy_name = expansion_header[2];
				let mut prepend_proxy_accessor = false;
				if expansion_header[3].contains(TASK_VAR_RANGE_DELIM) {
					let range_vals: Vec<&str> = expansion_header[3].split(TASK_VAR_RANGE_DELIM).collect();
					start_num = range_vals[0].parse().unwrap();
					end_num = range_vals[1].parse().unwrap();
				} else {
					prepend_proxy_accessor = true;
					let end_num =
						get_value_from_accessor_string(String::from(expansion_header[3]), input_vars)
							.as_array()
							.unwrap()
							.len();
				}
				for i in start_num..end_num {
					if prepend_proxy_accessor {
						let prepend_proxy = &format!(
							"{}{}{}{}{}",
							TASK_INTERNAL_DELIM,
							expansion_header[3],
							TASK_VAR_ARRAY_OPEN_DELIM,
							i,
							TASK_VAR_ARRAY_CLOSE_DELIM
						);
						let proxy_in_accessor = &format!("{}{}", TASK_INTERNAL_DELIM, proxy_name);
						expanded_content = &format!(
							"{}{}",
							expanded_content,
							expansion_content.replace(proxy_in_accessor, prepend_proxy)
						)
					} else {
						let prepend_proxy = &format!("{}{}", TASK_INTERNAL_DELIM, i);
						let proxy_in_accessor = &format!("{}{}", TASK_INTERNAL_DELIM, proxy_name);
						let prepend_proxy_type_b = &format!("{}{}", TASK_VAR_ARRAY_OPEN_DELIM, i);
						let proxy_in_accessor_type_b = &format!("{}{}", TASK_VAR_ARRAY_OPEN_DELIM, proxy_name);
						expanded_content = &format!(
							"{}{}",
							expanded_content,
							expansion_content
								.replace(proxy_in_accessor, prepend_proxy)
								.replace(proxy_in_accessor_type_b, prepend_proxy_type_b)
						)
					}
				}
				output = format!(
					"{}{}{}",
					pre_expansion_content, expanded_content, post_expansion_content
				)
			} else if expansion_type == TASK_IF {
				output = format!(
					"{}{}{}",
					pre_expansion_content, expansion_content, post_expansion_content
				)
			}
		}
		return output;
	}
}*/

const TASK_OPEN_DELIM: &str = "|+";
const TASK_CLOSE_DELIM: &str = "+|";
const TASK_INTERNAL_DELIM: &str = ":";
const TASK_VAR_TRAVERSAL_DELIM: &str = ".";
const TASK_VAR_RANGE_DELIM: &str = "..";
const TASK_VAR_ARRAY_OPEN_DELIM: &str = "[";
const TASK_VAR_ARRAY_CLOSE_DELIM: &str = "]";
// accessor means any variable accessor. all instances of a proxyname from a loop will be replaced.
const TASK_ESCAPE: &str = "esc"; // esc:string_to_escape
const TASK_VAR: &str = "var"; // var:accessor
const TASK_VAR_REGEX: &str = "varre"; // varre:accessor:regex:regex_replace_with
const TASK_FOR: &str = "for"; // for:id:proxyname:range or for:id:proxyname:array_accessor
const TASK_IF: &str = "if"; // if:id:accessor_or_number:comparator:accessor_or_number or if:id:accessor:exists
const TASK_IF_EXISTS: &str = "ex";
const TASK_ELSE_IF: &str = "elif";
const TASK_ELSE: &str = "else";
const TASK_TEMPLATE: &str = "tmpl"; // tmpl:template_name:accessor

fn prep_template_string_for_parsing<'a>(template_string: String) -> Vec<&'a str> {
	let splitting_delim = "5\n%\rH";
	let template_string_edit_1 = format!(
		"{}{}{}{}{}",
		template_string, TASK_OPEN_DELIM, TASK_ESCAPE, TASK_INTERNAL_DELIM, TASK_CLOSE_DELIM
	);
	let template_string_edit_2 = template_string_edit_1.replace(
		TASK_OPEN_DELIM,
		&format!("{}{}", splitting_delim, TASK_OPEN_DELIM),
	);
	let mut template_string_edit_3: Vec<&str> =
		template_string_edit_2.split(splitting_delim).collect();
	let mut template_string_edit_4: Vec<&str> = Vec::new();
	for input in template_string_edit_3 {
		if input.starts_with(TASK_OPEN_DELIM) {
			template_string_edit_4.push(
				&input
					.replace(TASK_OPEN_DELIM, &"")
					.replace(TASK_CLOSE_DELIM, &""),
			);
		} else {
			template_string_edit_4.push(&format!("{}{}{}", TASK_ESCAPE, TASK_INTERNAL_DELIM, input));
		}
	}
	return template_string_edit_4;
}
