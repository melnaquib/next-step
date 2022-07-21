use super::*;
use frame_support::pallet_prelude::*;

fn get_target_action<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
	action: &types::Action,
	action_data: &types::ActionData
) -> types::Action {
	let target = <DeModelActionFlows<T>>::get(deprocess, types::to_bounded::<T>(action.to_vec()));
	types::from_bounded::<T>(&target)
}

enum Permission {

}

fn _step<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
	action: &types::Action,
	action_data: &types::ActionData
) -> DispatchResult {

	let current = &<DeProcessCurrent<T>>::get(deprocess);
	let current = types::from_bounded::<T>(current);
	if *action != current {
		// Error::<T>::NotDeProcessCurrentAction
		return Ok(());
	}

	if ! access::is_process_action_actor::<T>(deprocess, &current, sender) {
		// Error::NotAssignedActorOfAction
		return Ok(());
	}

	let target = get_target_action::<T>(sender, deprocess, action, action_data);
	<DeProcessCurrent<T>>::insert(deprocess, types::to_bounded::<T>(target.to_vec()));
	<DeProcessActionData<T>>::insert(deprocess, types::to_bounded::<T>(action.to_vec()), types::to_bounded::<T>(action_data.to_vec()));
	
	Pallet::<T>::deposit_event(Event::<T>::Step { deprocess: *deprocess, src: action.to_vec(), dst: target, data: action_data.to_vec() });

	Ok(())
}

pub fn start<T: Config>(owner: &T::AccountId, deprocess: &types::DeProcessId,
	action_data: &types::ActionData) -> DispatchResult {
	<DeProcessCurrent<T>>::insert(deprocess, types::to_bounded::<T>(types::from_str("START")));
	_step::<T>(owner, deprocess, &types::from_str("START"), action_data)
}

pub fn step<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
	action: &types::Action,
	action_data: &types::ActionData
) -> DispatchResult {
	_step::<T>(sender, deprocess, action, action_data)
}

pub fn is_acted<T: Config>(deprocess: &types::DeProcessId, action: &types::Action) -> bool {
	<DeProcessActionData<T>>::contains_key(deprocess, types::to_bounded::<T>(action.to_vec()))
}

pub fn action_data<T: Config>(deprocess: &types::DeProcessId, action: &types::Action) -> types::ActionData {
	types::from_bounded::<T>(&<DeProcessActionData<T>>::get(deprocess, types::to_bounded::<T>(action.to_vec())))
}
