use super::bpmn;
use super::types;

use sp_io::hashing;
use sp_std::str;
use sp_std::vec::Vec;

// pub fn model_id(data: &[u8]) -> bpm::ModelId {
// 	hashing::twox_64(data)
// }

pub trait DeProcessStorage {
	fn count_get() -> u128;
	fn count_set(count: &u128);

	//flows
	// fn flows_get(deprocess: &types::DeProcessId, action: &types::Action) -> types::Action;
	// fn flows_set(deprocess: &types::DeProcessId, action: &types::Action, target: types::Action);

	//actors
	// fn actors_get(deprocess: &types::DeProcessId, action: &types::Action) -> types::Actor;
	// fn actors_set(deprocess: &types::DeProcessId, action: &types::Action, actor: &types::Actor);

	fn current_get(deprocess: &types::DeProcessId) -> &types::Action;
	fn current_set(deprocess: &types::DeProcessId, action: &types::Action);
	fn current_has(deprocess: &types::DeProcessId) -> bool;

	// fn data_get(deprocess: &types::DeProcessId, action: &types::Action) -> types::Data;
	// fn data_set(deprocess: &types::DeProcessId, action: &types::Action, data: &types::Data);
	// fn data_has(deprocess: &types::DeProcessId, action: &types::Action) -> bool;
}

// pub const START: &'static types::ActionName = &types::from_str("START");
// pub const END: types::ActionName = types::from_str("END");

// pub fn from_bpmn(bpmn_xml: &bpmn::BpmnXml) {
// 	// bpmn::read_bpmn(bpmn_xml)
// }

// pub fn actor(deprocess: &types::DeProcessId, action: &types::Action) -> &types::Actor {
// 	self.actors.get(action).unwrap()
// }
// pub fn target(deprocess: &types::DeProcessId, action: &types::Action) -> &types::Action {
// 	self.flows.get(action).unwrap()
// }

// pub fn first(deprocess: &types::DeProcessId) -> &types::Action {
// 	target(deprocess, &START)
// }

// pub fn start(deprocess: &types::DeProcessId, current: Option<&types::Action>) {
// 	let id = model.id;
// 	let current = current.unwrap_or_else(|| model.first());
// 	Self {
// 		id,
// 		model,
// 		current: current.to_vec(),
// 	};
// }

// pub fn ended(deprocess: &types::DeProcessId, action: Option<&types::ActionName>) -> bool {
// 	let action = action.unwrap_or_else(|| &END);
// 	self.done.contains_key(action)
// }

// pub fn step(
// 	deprocess: &types::DeProcessId,
// 	actor: &types::Actor,
// 	action: types::ActionName,
// ) -> bool {
// 	if action != self.current {
// 		return false;
// 	}
// 	if actor != self.model.actor(action) {
// 		return false;
// 	}
// 	self.done.insert(self.current, {});
// 	self.current = self.model.next(self.current);
// 	true
// }
