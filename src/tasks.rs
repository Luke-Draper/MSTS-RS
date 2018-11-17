extern crate derive_builder;
extern crate serde_json;
// accessor means any variable accessor. all instances of a proxyname from a loop will be replaced.
const TASK_ESCAPE: &'static str = "esc"; // esc:string_to_escape
const TASK_VAR: &'static str = "var"; // var:accessor
const TASK_VAR_REGEX: &'static str = "varre"; // varre:accessor:regex:regex_replace_with
const TASK_FOR: &'static str = "for"; // for:id:proxyname:range or for:id:proxyname:array_accessor
const TASK_FOR_EACH_TYPE_NAME: &'static str = "for_each"; // for:id:proxyname:range or for:id:proxyname:array_accessor
const TASK_IF: &'static str = "if"; // if:id:accessor_or_number:comparator:accessor_or_number or if:id:accessor:exists
const TASK_IF_EXISTS: &'static str = "ex";
const TASK_ELSE_IF: &'static str = "elif";
const TASK_ELSE: &'static str = "else";
const TASK_END: &'static str = "end"; // ends current expandable statement
const TASK_TEMPLATE: &'static str = "tmpl"; // tmpl:template_name:accessor

const TASK_OPEN_DELIM: &'static str = "|+";
const TASK_CLOSE_DELIM: &'static str = "+|";
const TASK_PROXY_DELIM: &'static str = "~"; // prepends all instances of the proxy except the declaration
const TASK_INTERNAL_DELIM: &'static str = ":";
const TASK_VAR_RANGE_DELIM: &'static str = "..";
const TASK_VAR_TRAVERSAL_DELIM: &'static str = ".";
const TASK_VAR_ARRAY_OPEN_DELIM: &'static str = "[";
const TASK_VAR_ARRAY_CLOSE_DELIM: &'static str = "]";

trait Task<'a> {
	fn get_type(&self) -> &str;
	fn run_task(&self, run_vars: serde_json::Value) -> String {
		return String::from("");
	}
	fn expand_task(&self, run_vars: serde_json::Value) -> Option<Vec<&Task>> {
		return None;
	}
	// self.sub_tasks = in_sub_tasks;
	fn set_sub_tasks(&self, in_sub_tasks: Vec<&'a Task<'a>>) {}
	// .replace(format!("{}{}",TASK_PROXY_DELIM,proxy),replacement);
	fn replace_proxy_in_task(&self, proxy: String, replacement: String);
}

struct StartTask<'a> {
	sub_tasks: Vec<&'a Task<'a>>,
}

struct EscapeTask {
	escaped_string: String,
}
impl<'a> Task<'a> for EscapeTask {
	fn get_type(&self) -> &str {
		return TASK_ESCAPE;
	}
	fn run_task(&self, run_vars: serde_json::Value) -> String {
		return self.escaped_string;
	}
	fn replace_proxy_in_task(&self, proxy: String, replacement: String) {
		self.escaped_string = self
			.escaped_string
			.replace(&format!("{}{}", TASK_PROXY_DELIM, proxy), &replacement);
	}
}

struct EndTask {}
impl<'a> Task<'a> for EndTask {
	fn get_type(&self) -> &str {
		return TASK_END;
	}
	fn replace_proxy_in_task(&self, proxy: String, replacement: String) {}
}

struct VarTask {
	accessor_string: String,
}
impl<'a> Task<'a> for VarTask {
	fn get_type(&self) -> &str {
		return TASK_VAR;
	}
	fn run_task(&self, run_vars: serde_json::Value) -> String {
		return get_value_from_accessor_string(self.accessor_string, run_vars).to_string();
	}
	fn replace_proxy_in_task(&self, proxy: String, replacement: String) {
		self.accessor_string = self
			.accessor_string
			.replace(&format!("{}{}", TASK_PROXY_DELIM, proxy), &replacement);
	}
}

struct ForEachTask<'a> {
	proxy: String,
	array_accessor_string: String,
	sub_tasks: Vec<&'a Task<'a>>,
}
impl<'a> Task<'a> for ForEachTask<'a> {
	fn set_sub_tasks(&self, in_sub_tasks: Vec<&'a Task<'a>>) {
		self.sub_tasks = in_sub_tasks;
	}
	fn get_type(&self) -> &str {
		return TASK_FOR_EACH_TYPE_NAME;
	}
	fn replace_proxy_in_task(&self, proxy: String, replacement: String) {
		self.array_accessor_string = self
			.array_accessor_string
			.replace(&format!("{}{}", TASK_PROXY_DELIM, proxy), &replacement);
		for sub in &self.sub_tasks {
			sub.replace_proxy_in_task(proxy, replacement);
		}
	}
	fn expand_task(&self, run_vars: serde_json::Value) -> Option<Vec<&Task>> {
		return Some();
	}
}

fn task_factory(input_string: &String) -> &Task {
	let split_input: Vec<&str> = input_string.split(TASK_INTERNAL_DELIM).collect();
	let task_type = split_input[0];
	if task_type == TASK_ESCAPE {
		return &EscapeTask {
			escaped_string: split_input[1].to_owned(),
		} as &Task;
	} else if task_type == TASK_VAR {
		return &VarTask {
			accessor_string: split_input[1].to_owned(),
		} as &Task;
	} else if task_type == TASK_END {
		return &EndTask {} as &Task;
	} else if task_type == TASK_VAR {
	} else if task_type == TASK_VAR {
	} else if task_type == TASK_VAR {
	} else {
		return &EscapeTask {
			escaped_string: String::from(""),
		} as &Task;
	}
}

fn get_value_from_accessor_string(
	input_var_string: String,
	input_vars: serde_json::Value,
) -> serde_json::Value {
	let mut output = input_vars;
	let escaped_indexes_string = input_var_string
		.replace(
			TASK_VAR_ARRAY_OPEN_DELIM,
			&format!("{}{}", TASK_VAR_TRAVERSAL_DELIM, TASK_VAR_ARRAY_OPEN_DELIM),
		)
		.replace(TASK_VAR_ARRAY_CLOSE_DELIM, TASK_VAR_TRAVERSAL_DELIM);
	let vars: Vec<&str> = escaped_indexes_string
		.split(TASK_VAR_TRAVERSAL_DELIM)
		.collect();
	for variable in vars {
		if variable.contains(TASK_VAR_ARRAY_OPEN_DELIM) {
			let index: usize = variable[1..].parse().unwrap();
			output = output[index];
		} else {
			output = output[variable];
		}
	}
	return output;
}
