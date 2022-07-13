use super::bpm;
use super::bpmn;
use super::types;

// use sp_io::hashing;

#[inline(always)]
pub fn deploy_bpmn(bpmn_xml: bpmn::BpmnXml) -> types::DeProcessId {
	types::Str::new()
}

#[inline(always)]
pub fn start(deprocess: types::DeProcessId) -> types::DeProcessId {
	types::Str::new()
}

trait TT {}

#[inline(always)]
pub fn deploy_bpmn_and_start<Storage: bpm::DeProcessStorage>(
	bpmn_xml: bpmn::BpmnXml,
) -> types::DeProcessId {
	// let deprocess: types::DeProcessId = Count::get();

	let deprocess = deploy_bpmn(bpmn_xml);
	let deprocess = start(deprocess);
	deprocess
}

#[inline(always)]
pub fn step(deprocess: types::DeProcessId, action: types::Action) -> bool {
	true
}

#[inline(always)]
pub fn action_actor(deprocess: types::DeProcessId, action: &types::Action) -> types::Actor {
	types::Str::new()
}
