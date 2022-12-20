use super::*;
use frame_support::pallet_prelude::*;
use sp_std::str::FromStr;

enum Permission {

}

// use num_derive::FromPrimitive;    
// use num_traits::FromPrimitive;

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum NodeType {
	Undefined = 0,
	None = 1,
	Start = 2,
	End = 3,
	Task = 4,
	Script = 5,
	InclusiveGateway = 6,
	ExclusiveGateway = 7,
	ParallelGateway = 8,
	EventBasedGateway = 9,
	ComplexGateway = 10,
	Process = 11,
	Participant = 12,
	LaneSet = 13,
	Lane = 14,
	SequenceFlow = 15,
	MessageFlow = 16,
}

impl NodeType {
	pub fn is_gateway(&self) -> bool {
		[NodeType::InclusiveGateway, NodeType::ExclusiveGateway,
			NodeType::ParallelGateway,
			NodeType::EventBasedGateway, NodeType::ComplexGateway]
			.iter().any(|v: &Self| self == v)
	}

	pub fn is_auto(&self) -> bool {
		[
			NodeType::Start, NodeType::End,
			NodeType::InclusiveGateway, NodeType::ExclusiveGateway,
			NodeType::ParallelGateway,
			NodeType::EventBasedGateway, NodeType::ComplexGateway]
			.iter().any(|v: &Self| self == v)
	}
}

impl FromStr for NodeType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let start = if s.starts_with("bpmn:") {
            5
        } else {
			0
		};
		let s = &s[start ..];
		Ok(match s {
			"startEvent" => NodeType::Start,
			"endEvent" => NodeType::End,
			"task" => NodeType::Task,

			"inclusiveGateway" => NodeType::InclusiveGateway,
			"exclusiveGateway" => NodeType::ExclusiveGateway,

			"process" => NodeType::Process,
			"participant" => NodeType::Participant,
			"laneSet" => NodeType::LaneSet,
			"lane" => NodeType::Lane,

			"sequenceFlow" => NodeType::SequenceFlow,
			"messageFlow" => NodeType::MessageFlow,
			_ => NodeType::Undefined,
		})
	}
}

fn _do_transition<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
	action_data: &types::ActionData,
	branch: &types::ActionData,
) -> DispatchResult {
	let current_id = &<DeProcessCurrent<T>>::get(deprocess);
	let(current_typ, current_name) = <DeModelNodes<T>>::get(deprocess, current_id).unwrap();

	let (flow_id, flow_target_id,) = <DeModelEdges<T>>::get((deprocess, current_id, branch));

	let (flow_target_type, flow_target_name) = <DeModelNodes<T>>::get(deprocess, flow_target_id.clone()).unwrap();

	<DeProcessCurrent<T>>::insert(deprocess, flow_target_id.clone());
	<DeProcessActionData<T>>::insert(deprocess, current_id, (utils::now::<T>(), action_data,));
	
	Pallet::<T>::deposit_event(Event::<T>::Step {
		deprocess: *deprocess,
		src: types::from_bounded::<T>(&current_name),
		dst: types::from_bounded::<T>(&flow_target_name),
		data: *action_data,

		account: sender.clone(),

		branch: *branch,

		src_id: types::from_bounded::<T>(&current_id),
		dst_id: types::from_bounded::<T>(&flow_target_id),

		flow_id: types::from_bounded::<T>(&flow_id),
	});

	Ok(())
}

fn _transition_auto_node<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
) -> bool {

	let current_id = &<DeProcessCurrent<T>>::get(deprocess);
	let(typ, name) = <DeModelNodes<T>>::get(deprocess, current_id).unwrap();

	if NodeType::End == typ {
		return false;
	}
	if !typ.is_auto() {
		return false;
	}

	let branch = if NodeType::ExclusiveGateway == typ {
		let check_action_id = <DeModelNameId<T>>::get(deprocess, name);
		let (timestamp, action_data, ) = <DeProcessActionData<T>>::get(deprocess, check_action_id);
		action_data
	} else {
		types::ActionData::default()
	};
	_do_transition::<T>(sender, deprocess, &types::ActionData::default(), &branch);
	true
}

fn _transition_auto<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
	action_id: &types::Action,
	action_data: &types::ActionData
) -> DispatchResult {
	while _transition_auto_node::<T>(sender, deprocess) {};
	Ok(())
}

fn _transition_task<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
	action_id: &types::Action,
	action_data: &types::ActionData,
) -> DispatchResult {

	let current_id = <DeProcessCurrent<T>>::get(deprocess);
	if types::to_bounded::<T>(action_id.clone()) != current_id {
		return on_err::<T>(&Error::<T>::NotDeProcessCurrentAction);
	}

	let(typ, name) = <DeModelNodes<T>>::get(deprocess, current_id).unwrap();

	let allowed = access::is_process_action_account::<T>(deprocess, &action_id, sender);
	if ! allowed {
		return on_err::<T>(&Error::<T>::RoleNotAssignedToAccount);
	}

	_do_transition::<T>(sender, deprocess, action_data, &types::ActionData::default())
}

fn _step<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
	action_id: &types::Action,
	action_data: &types::ActionData,
) -> DispatchResult {

	let result = if action_id.is_empty() {
		Ok(())
	} else {
		_transition_task::<T>(sender, deprocess, action_id, action_data)
	};

	if result.is_ok() {
		_transition_auto::<T>(sender, deprocess, action_id, action_data)
	} else {
		result
	}
}

pub fn start<T: Config>(owner: &T::AccountId, deprocess: &types::DeProcessId, action_data: &types::ActionData) -> DispatchResult {
	
	let start_ = types::to_bounded::<T>(types::from_str("START"));
	let start_id = <DeModelNameId<T>>::get(deprocess, start_);
	<DeProcessCurrent<T>>::insert(deprocess, start_id.clone());

	let start_id = types::from_bounded::<T>(&start_id);
	_step::<T>(owner, deprocess, &types::str_default(), action_data)
}

pub fn step<T: Config>(
	sender: &T::AccountId,
	deprocess: &types::DeProcessId,
	action: &types::Action,
	action_data: &types::ActionData
) -> DispatchResult {
	let action = types::to_bounded::<T>(action.to_vec());
	let action_id = <DeModelNameId<T>>::get(deprocess, action);
	_step::<T>(sender, deprocess, &action_id, action_data)
}

pub fn is_acted<T: Config>(deprocess: &types::DeProcessId, action: &types::Action) -> bool {
	let action = types::to_bounded::<T>(action.to_vec());
	let action_id = <DeModelNameId<T>>::get(deprocess, action);
	<DeProcessActionData<T>>::contains_key(deprocess, action_id)
}

// pub fn act<T: Config>(deprocess: &types::DeProcessId, action: &types::Action, action_data: &types::ActionData) -> bool {
// 	let action = types::to_bounded::<T>(action.to_vec());
// 	let action_id = <DeModelNameId<T>>::get(deprocess, action);
// 	<DeProcessActionData<T>>::contains_key(deprocess, action_id)
// }

pub fn action_timestamp<T: Config>(deprocess: &types::DeProcessId, action: &types::Action) -> types::TimeStamp {
	let action = types::to_bounded::<T>(action.to_vec());
	let action_id = <DeModelNameId<T>>::get(deprocess, action);
	<DeProcessActionData<T>>::get(deprocess, action_id).0
}

pub fn action_data<T: Config>(deprocess: &types::DeProcessId, action: &types::Action) -> types::ActionData {
	let action = types::to_bounded::<T>(action.to_vec());
	let action_id = <DeModelNameId<T>>::get(deprocess, action);
	<DeProcessActionData<T>>::get(deprocess, action_id).1
}
