use super::types;

use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;
use reader_for_microxml::*;
use sp_std::boxed::Box;

pub type XmlStr = types::Str;
pub type OnElementEnd<'a> = dyn FnMut(&Vec<&str>, &Vec<BTreeMap<&str, types::Str>>) + 'a;

pub fn visit_element_end(xml_str: &XmlStr, on_element_end: & mut OnElementEnd) {
	// micro Xml cannot process xml declaration
	// so skipping first line; first newline character
	// check if there is a PI
	let index = if '?' == xml_str[1] as char {
		1 + xml_str.iter().position(|&c| c == b'\n').unwrap()
	} else {
		0
	};
	let xml_str = &xml_str[index ..].to_vec();

	let mut reader_iterator = ReaderForMicroXml::new(types::to_str(xml_str));

	let mut path: Vec<&str> = Vec::new();
	let mut attrs: Vec<BTreeMap<&str, types::Str>> = Vec::new();

	for result_token in reader_iterator {
		match result_token {
			Ok(token) => match token {
				Token::StartElement(name) => {
					path.push(&name);
					attrs.push(BTreeMap::new());
				}
				Token::Attribute(name, value) => {
					attrs
						.last_mut()
						.unwrap()
						.insert(name, types::from_str(value).clone());
				}
				Token::Comment(txt) => {}
				Token::TextNode(txt) => {}
				Token::EndElement(name) => {

					on_element_end(&path, &attrs);

					attrs.pop();
					path.pop();
				}
			},
			Err(err_msg) => {
				(err_msg);
			}
		}
	}
}
