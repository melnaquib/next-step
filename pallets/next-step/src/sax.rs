use super::types;

use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;
// use sp_std::iter::map;

// use roxmltree;
// extern crate sxd_document;
// extern crate sxd_xpath;

// use txml;
use reader_for_microxml::*;

pub type on_element_end = fn(path: &Vec<&str>, attrs: &Vec<BTreeMap<&str, types::Str>>);

pub trait SaxHandler {
	fn on_element_end(&mut self, path: &Vec<&str>, attrs: &Vec<BTreeMap<&str, types::Str>>);
}

pub type XmlStr = types::Str;

pub fn visit_element_end(xml_str: &XmlStr, mut sax_handler: &mut dyn SaxHandler) {
	// micro Xml cannot process xml declaration
	// so skipping first line; first newline character
	let index = xml_str.iter().position(|&c| c == 0x0a).unwrap();
	let xml_str = &xml_str[index..].to_vec();

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
					sax_handler.on_element_end(&path, &attrs);
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
