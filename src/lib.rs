extern crate cfg_if;
extern crate wasm_bindgen;
extern crate regex;

mod utils;

use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    log("Hello, fuzzy-search!");
}

#[wasm_bindgen]
pub fn fuzzy(c: &str, s: &str) -> String {
	let rgx = Regex::new(c).unwrap();
	let res = rgx.replace_all(s, "<span>$0</span>");

	res.to_string()
}
