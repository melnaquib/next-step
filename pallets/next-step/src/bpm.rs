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
	Pallet::<T>::deposit_event(Event::<T>::Step { deprocess: *deprocess, src: action.to_vec(), dst: target });

	Ok(())
}

pub fn start<T: Config>(owner: &T::AccountId, deprocess: &types::DeProcessId) -> DispatchResult {
	_step::<T>(owner, deprocess, &types::from_str("START"), &types::ActionData::new())
}

pub fn step<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
	action: &types::Action,
	action_data: &types::ActionData
) -> DispatchResult {
	_step::<T>(sender, deprocess, action, action_data)
}
