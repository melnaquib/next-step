use super::*;
use frame_support::pallet_prelude::*;

pub fn assign<T: Config>(
	owner: &T::AccountId,
	role: &types::Str,
	account: &T::AccountId,
) -> DispatchResult {
	let role = types::to_bounded::<T>(role.to_vec());
	<OwnerRoles<T>>::insert(owner, role, account);
	Ok(())
}

pub fn unassign<T: Config>(
	owner: &T::AccountId,
	role: &types::Str,
	account: &T::AccountId,
) -> DispatchResult {
	let role = types::to_bounded::<T>(role.to_vec());
	<OwnerRoles<T>>::remove(owner, role);
	Ok(())
}

pub fn is_process_action_actor<T: Config>(deprocess: &types::DeProcessId, action: &types::Action, account: &T::AccountId) -> bool {
    // <DeProcessActionActor<T>>::get(deprocess, action, account)
    true
}

pub fn get_process_action_account<T: Config>(deprocess: &types::DeProcessId, action: &types::Action,
    lane: &types::Str, lane_set: &types::Str, process: &types::Str) -> T::AccountId {

	let owner = <DeProcessOwners<T>>::get(deprocess).unwrap();

	let account = <OwnerRoles<T>>::get(owner.clone(), types::to_bounded::<T>(lane.to_vec()));
	if account.is_some() {
		return account.unwrap();
	}

	let account = <OwnerRoles<T>>::get(owner.clone(), types::to_bounded::<T>(lane_set.to_vec()));
	if account.is_some() {
		return account.unwrap();
	}

	let account = <OwnerRoles<T>>::get(owner.clone(), types::to_bounded::<T>(process.to_vec()));
	account.unwrap_or(owner)

}
