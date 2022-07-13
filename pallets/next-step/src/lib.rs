#![cfg_attr(not(feature = "std"), no_std)]
// #![feature(associated_type_bounds)]

use frame_support::traits::{Currency, OnUnbalanced, ReservableCurrency};
pub use pallet::*;
// use sp_runtime::traits::{StaticLookup, Zero};
use sp_std::convert::TryInto;
use sp_std::prelude::*;

use sp_std::collections::btree_map::BTreeMap;
use bimap::btree::BiBTreeMap;
use sp_core::crypto::AccountId32;
use hex_literal::hex;
use sp_std::convert::From;


// use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto};
use sp_core::crypto;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

use sp_std::convert::AsMut;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
// type NegativeImbalanceOf<T> =
// 	<<T as Config>::Currency as Currency<AccountIdOf<T>>>::NegativeImbalance;

mod sax;
mod types;
mod utils;

mod bpm;
mod bpmn;
// mod bpx;

// mod dev;

use reader_for_microxml::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Reservation fee.
		#[pallet::constant]
		type ReservationFee: Get<BalanceOf<Self>>;

		/// The origin which may forcibly set or remove a name. Root can always do this.
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		/// The minimum length a name may be.
		#[pallet::constant]
		type MinLength: Get<u32>;

		/// The maximum length a name may be.
		#[pallet::constant]
		type MaxLength: Get<u32>;

		#[pallet::constant]
		type STR_NAME_MAX_LENGTH: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A name was set.
		NameSet {
			who: T::AccountId,
		},

		Start {
			deprocess: types::DeProcessId,
		},
		Step {
			deprocess: types::DeProcessId,
			src: types::Action,
			dst: types::Action,
		},
	}

	/// Error for the next_step pallet.
	#[pallet::error]
	pub enum Error<T> {
		TooShort,
		TooLong,
		Unnamed,
		NotAssignedActorOfAction,
		NotDeProcessCurrentAction,
	}

	pub type BoundedStr<T: Config> = BoundedVec<u8, T::STR_NAME_MAX_LENGTH>;

	fn to_bounded<T: Config>(s: types::Str) -> BoundedStr<T> {
		let res: BoundedVec<_, _> = s.try_into().map_err(|()| Error::<T>::TooLong).unwrap();
		res
	}

	fn from_bounded<T: Config>(s: &BoundedStr<T>) -> types::Str  {
		s.to_vec()
	}

	//Model Data
	#[pallet::storage]
	pub(super) type DeModelActionFlows<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		types::DeProcessId,
		Blake2_128Concat,
		BoundedStr<T>,
		BoundedStr<T>,
		ValueQuery,
	>;

	#[pallet::storage]
	pub(super) type DeModelActionActors<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		types::DeProcessId,
		Blake2_128Concat,
		BoundedStr<T>,
		BoundedStr<T>,
		ValueQuery,
	>;

	// DeProcess data

	// #[pallet::type_value]
	// pub fn DefaultZero() -> u128 {
	// 	0
	// }


	#[pallet::storage]
	pub(super) type DeProcessRoles<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedStr<T>, ValueQuery>;

	
	#[pallet::type_value]
	pub(super) fn DefaultZero<T: Config>() -> types::DeProcessId {
		0
	}

	#[pallet::storage]
	pub(super) type DeProcessCount<T: Config> =
		StorageValue<_, types::DeProcessId, ValueQuery, DefaultZero<T>>;

	#[pallet::storage]
	pub(super) type DeProcessCurrent<T: Config> =
		StorageMap<_, Blake2_128Concat, types::DeProcessId, BoundedStr<T>, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn start(origin: OriginFor<T>, bpmn_str: types::Str) -> DispatchResult {

			let index = bpmn_str.iter().position(|&c| c == 0x0a).unwrap();
			let bpmn_str = &bpmn_str[index + 1 ..].to_vec();

			//get new deprocess id
			let deprocess = <DeProcessCount<T>>::get() + 1;

			struct BpmnSaxHandler {
				callback: Box<dyn FnMut(&Vec<&str>, &Vec<BTreeMap<&str, types::Str>>)>,
			}

			impl BpmnSaxHandler {
				fn set_callback(&mut self, callback: impl FnMut(&Vec<&str>, &Vec<BTreeMap<&str, types::Str>>) + 'static) {
					self.callback = Box::new(callback);
				}
			
				// fn process_events(&mut self) {
				// 	(self.callback)();
				// }

				fn new(callback: impl FnMut(&Vec<&str>, &Vec<BTreeMap<&str, types::Str>>) + 'static) -> Self {
					let callback = Box::new(callback);
					Self {callback}
				}
			
			}

			impl sax::SaxHandler for BpmnSaxHandler {

				fn on_element_end(&mut self, path: &Vec<&str>, attrs: &Vec<BTreeMap<&str, types::Str>>) {
					(self.callback)(path, attrs);
				}

			}

			let mut process_participants: BTreeMap<types::Str, types::Actor> = BTreeMap::new();
			{
				// let get_process_participants =
				// 	|path: &Vec<&str>, attrs: &Vec<BTreeMap<&str, types::Str>>| {
				// 	match *path.last().unwrap() {
				// 		"participant" => {
				// 			process_participants.insert(
				// 				attrs.last().unwrap()["processRef"].clone(),
				// 				attrs.last().unwrap()["name"].clone(),
				// 			);
				// 		},
				// 		_ => {}
				// 	}
				// };
				// let mut handler = BpmnSaxHandler::new(get_process_participants);
				// sax::visit_element_end(&bpmn_str, &mut handler);

				let mut path: Vec<&str> = Vec::new();
				let mut attrs: Vec<BTreeMap<&str, types::Str>> = Vec::new();
				let mut reader_iterator = ReaderForMicroXml::new(types::to_str(bpmn_str));
				for result_token in reader_iterator {
					match result_token {
						Ok(token) => match token {
							Token::StartElement(name) => {
								path.push(&name);
								attrs.push(BTreeMap::new());
							},
							Token::Attribute(name, value) => {
								attrs
									.last_mut()
									.unwrap()
									.insert(name, types::from_str(value).clone());
							},
							Token::EndElement(name) => {
								match *path.last().unwrap() {
									"participant" | "bpmn:participant" => {
										 process_participants.insert(
											attrs.last().unwrap()["processRef"].clone(),
											attrs.last().unwrap()["name"].clone(),
										);
									},
									_ => {}
								};

								attrs.pop();
								path.pop();
							},
							_ => {},
						},
						Err(err_msg) => {
							(err_msg);
						},
					}
				}
			}

			let mut task_ids_names: BTreeMap<types::Str, types::Actor> = BTreeMap::new();
			{
				// let get_tasks = move |path: &Vec<&str>, attrs: &Vec<BTreeMap<&str, types::Str>>| match *path
				// 	.last()
				// 	.unwrap()
				// {
				// 	"startEvent" => {
				// 		task_ids_names.insert(attrs.last().unwrap()["id"].clone(), types::from_str("START"));
				// 	}
				// 	"endEvent" => {
				// 		task_ids_names.insert(attrs.last().unwrap()["id"].clone(), types::from_str("END"));
				// 	}
				// 	"task" => {
				// 		task_ids_names.insert(
				// 			attrs.last().unwrap()["id"].clone(),
				// 			attrs.last().unwrap()["name"].clone(),
				// 		);
				// 		let process_ref = attrs[attrs.len() - 2]["process_ref"].to_vec();
				// 		let actor = process_participants.get(&process_ref).clone().unwrap();
				// 		<DeModelActionActors<T>>::insert(process, to_bounded::<T>(
				// 			attrs.last().unwrap()["name"].to_vec()), to_bounded::<T>(actor.to_vec()));
				// 	}
				// 	_ => {}
				// };
				// let mut handler = BpmnSaxHandler::new(get_tasks);
				// sax::visit_element_end(&bpmn_str, &mut handler);

				let mut path: Vec<&str> = Vec::new();
				let mut attrs: Vec<BTreeMap<&str, types::Str>> = Vec::new();
				let mut reader_iterator = ReaderForMicroXml::new(types::to_str(bpmn_str));
				for result_token in reader_iterator {
					match result_token {
						Ok(token) => match token {
							Token::StartElement(name) => {
								path.push(&name);
								attrs.push(BTreeMap::new());
							},
							Token::Attribute(name, value) => {
								attrs
									.last_mut()
									.unwrap()
									.insert(name, types::from_str(value).clone());
							},
							Token::EndElement(name) => {
								match *path.last().unwrap() {
									"startEvent" | "bpmn:startEvent" => {
										task_ids_names.insert(attrs.last().unwrap()["id"].clone(), types::from_str("START"));
									},
									"endEvent" | "bpmn:endEvent" => {
										task_ids_names.insert(attrs.last().unwrap()["id"].clone(), types::from_str("END"));
									},
									"task" | "bpmn:task" => {
										task_ids_names.insert(
											attrs.last().unwrap()["id"].clone(),
											attrs.last().unwrap()["name"].clone(),
										);
										let process_ref = attrs[attrs.len() - 2]["id"].to_vec();
										let actor = process_participants.get(&process_ref).clone().unwrap();
										<DeModelActionActors<T>>::insert(deprocess, to_bounded::<T>(
											attrs.last().unwrap()["name"].to_vec()), to_bounded::<T>(actor.to_vec()));
									},
									_ => {}
								};

								attrs.pop();
								path.pop();
							},
							_ => {},
						},
						Err(err_msg) => {
							(err_msg);
						}
					}
				}

			}

			{

				// let get_flows = |path: &Vec<&str>, attrs: &Vec<BTreeMap<&str, types::Str>>| match *path
				// 	.last()
				// 	.unwrap()
				// {
				// 	"messageFlow" | "sequenceFlow" => {
				// 		let src = task_ids_names.get(&attrs.last().unwrap()["sourceRef"]).unwrap();
				// 		let dst = task_ids_names.get(&attrs.last().unwrap()["targetRef"]).unwrap();
				// 		<DeModelActionFlows<T>>::insert(process, to_bounded::<T>(src.to_vec()), to_bounded::<T>(dst.to_vec()));
				// 	},
				// 	_ => {}
				// };
				// let mut handler = BpmnSaxHandler::new(get_flows);
				// sax::visit_element_end(&bpmn_str, &mut handler);


				let mut path: Vec<&str> = Vec::new();
				let mut attrs: Vec<BTreeMap<&str, types::Str>> = Vec::new();
				let mut reader_iterator = ReaderForMicroXml::new(types::to_str(bpmn_str));
				for result_token in reader_iterator {
					match result_token {
						Ok(token) => match token {
							Token::StartElement(name) => {
								path.push(&name);
								attrs.push(BTreeMap::new());
							},
							Token::Attribute(name, value) => {
								attrs
									.last_mut()
									.unwrap()
									.insert(name, types::from_str(value).clone());
							},
							Token::EndElement(name) => {
								match *path.last().unwrap() {
										"messageFlow" | "sequenceFlow" | "bpmn:messageFlow" | "bpmn:sequenceFlow" => {
											let src = task_ids_names.get(&attrs.last().unwrap()["sourceRef"]).unwrap();
											let dst = task_ids_names.get(&attrs.last().unwrap()["targetRef"]).unwrap();
											<DeModelActionFlows<T>>::insert(deprocess, to_bounded::<T>(src.to_vec()), to_bounded::<T>(dst.to_vec()));
										},
									_ => {}
								}

								attrs.pop();
								path.pop();
							},
							_ => {},
						},
						Err(err_msg) => {
							(err_msg);
						}
					}
				}

			}

			<DeProcessCount<T>>::set(deprocess);
			// Self::deposit_event(Event::<T>::Start { deprocess });

			Pallet::<T>::step_prv(origin, deprocess, types::from_str("START"), true);

			// let deprocess = bpx::deploy_bpmn_and_start::<Storage<T>>(bpmn_str);
			Ok(())
		}		

		#[pallet::weight(1_000_000)]
		pub fn step(
			origin: OriginFor<T>,
			deprocess: types::DeProcessId,
			action: types::Str,
		) -> DispatchResult {

			Pallet::<T>::step_prv(origin, deprocess, action, false)
		}


		#[pallet::weight(1_000_000)]
		pub fn step_prv(
			origin: OriginFor<T>,
			deprocess: types::DeProcessId,
			action: types::Str,
			force: bool,
		) -> DispatchResult {

			let sender = ensure_signed(origin)?;

			
			let role = <DeProcessRoles<T>>::get(sender);
			let actor = <DeModelActionActors<T>>::get(deprocess, to_bounded::<T>(action.to_vec()));

			if !force && role != actor {
			// 	// return Error::NotAssignedActorOfAction;
				return Ok(());
			}



			let current = &<DeProcessCurrent<T>>::get(deprocess);
			let current = from_bounded::<T>(current);
			if !force && action != current {
				// return Error::NotDeProcessCurrentAction;
				return Ok(());
			}

			let target = <DeModelActionFlows<T>>::get(deprocess, to_bounded::<T>(action.to_vec()));
			let target = from_bounded::<T>(&target);

			<DeProcessCurrent<T>>::insert(deprocess, to_bounded::<T>(target.to_vec()));
			
			Self::deposit_event(Event::<T>::Step { deprocess, src: action, dst: target });

			// let start: BoundedStr<T> = types::from_str("START").try_into().unwrap();
			// let current = <DeModelActionFlows<T>>::get(deprocess, start);
			// <DeProcessCurrent<T>>::insert(deprocess, current);
			// //event step

			Ok(())
		}

		#[pallet::weight(1_000_000)]
		pub fn assign(
			origin: OriginFor<T>,
			role: types::Str,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			<DeProcessRoles<T>>::insert(sender, to_bounded::<T>(role));
			Ok(())
		}
	}

}
