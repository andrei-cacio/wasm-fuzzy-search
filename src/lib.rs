extern crate cfg_if;
extern crate wasm_bindgen;
extern crate regex;
#[macro_use]
extern crate serde_derive;

mod utils;

use wasm_bindgen::prelude::*;
use regex::Regex;

#[derive(Deserialize, Serialize)]
pub struct Book {
	author: String,
	title: String,
    // country: String,
    // image_link: String,
    // language: String,
    // link: String,
    // pages: u32,
    // year: u8
}

#[derive(Deserialize, Serialize)] 
pub struct Books {
	col: Vec<Book>,
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    log("Hello, fuzzy-search!");
}

#[wasm_bindgen]
pub fn fuzzy(query: &str, content: &str) -> String {
	if query == "" {
		return content.to_string();
	}

	let rgx = Regex::new(query).unwrap();
	let res = rgx.replace_all(content, "<span class='match'>$0</span>");

	res.to_string()
}

#[wasm_bindgen]
pub fn filter_books(js_book: &JsValue, by_author: &str) -> JsValue {
	let books: Books = js_book.into_serde().unwrap();
	
	let col: Vec<Book> = books.col
	.into_iter()
	.filter(|book| {
		let rgx = Regex::new(by_author).unwrap();

		rgx.is_match(&book.author)
	})
	.map(|mut book| {
		book.author = fuzzy(by_author, &book.author);

		book
	})
	.collect();

	let b = Books { col };

	return JsValue::from_serde(&b).unwrap();
}
