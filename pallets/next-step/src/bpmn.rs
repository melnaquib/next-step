use super::bpm;
use super::sax;
use super::types;

use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;
// use sp_std::iter::map;

pub type BpmnXml = types::Str;

// pub fn read_bpmn(bpmn_xml: &BpmnXml) -> bpm::Model {
// 	let mut id: types::ModelId;
// 	let mut flows: Map<types::Action, types::Action> = Map::new();
// 	let mut actors: Map<types::ActionId, types::Actor> = Map::new();
// 	let mut action_ids_names: Map<types::ActionId, types::ActionName> = Map::new();
// 	let mut action_names_ids: Map<types::ActionName, types::ActionId> = Map::new();

// 	let mut actor: types::Actor = "";

// 	// let on_element_end = |id, flows, actors, actionIdsNames, actionNamesIds| (path: &Vec<&str>, attrs: &Vec<Map<&str, types::Str> >) {
// 	let on_element_end = |path: &Vec<&str>, attrs: &Vec<Map<&str, types::Str>>| {
// 		match *path.last().unwrap() {
// 			"bpmn:definitions" => {
// 				id = attrs.last().unwrap()["id"].clone();
// 			}
// 			"bpmn:userTask" | "bpmn:startEvent" | "bpmn:endEvent" => {
// 				action_ids_names.insert(
// 					attrs.last().unwrap()["id"].clone(),
// 					attrs.last().unwrap()["name"].clone(),
// 				);
// 			}

// 			"bpmn:sequenceFlow" => {
// 				flows.insert(
// 					action_ids_names[&attrs.last().unwrap()["sourceRef"]].clone(),
// 					action_ids_names[&attrs.last().unwrap()["targetRef"]].clone(),
// 				);
// 			}
// 			"zeebe:assignmentDefinition" => {
// 				// actors.insert(
// 				// 	action_ids_names[&attrs[attrs.len() - 3]["id"]].clone(),
// 				// 	attrs.last().unwrap()["assignee"].clone(),
// 				// );
// 			}
// 			_ => {}
// 		}
// 	};

// 	sax::visit_element_end(bpmn_xml, on_element_end);

// }
