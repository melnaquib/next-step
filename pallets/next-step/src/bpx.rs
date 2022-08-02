use super::*;
use frame_support::pallet_prelude::*;

#[inline(always)]
pub fn start<T: Config>(owner: &T::AccountId, bpmn_str: &bpmn::BpmnStr, action_data: &types::ActionData)
	-> DispatchResult {
	let deprocess = <DeProcessCount<T>>::get() + 1;
	<DeProcessCount<T>>::set(deprocess);
	<DeProcessOwners<T>>::insert(deprocess, owner);

	bpmn::store_model_spec_bpmn::<T>(&deprocess, bpmn_str);
	bpm::start::<T>(owner, &deprocess, action_data);

	Ok(())
}

#[inline(always)]
pub fn step<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
	action: &types::Action,
	action_data: &types::ActionData,
) -> DispatchResult {
	bpm::step::<T>(sender, deprocess, action, action_data)
}

pub fn assign<T: Config>(
	owner: &T::AccountId,
	role: &types::Str,
	account: &T::AccountId,
) -> DispatchResult {
	access::assign::<T>(owner, role, account)
}

pub fn unassign<T: Config>(
	owner: &T::AccountId,
	role: &types::Str,
) -> DispatchResult {
	access::unassign::<T>(owner, role)
}
