#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, OnUnbalanced, ReservableCurrency};
pub use pallet::*;
use sp_std::convert::TryInto;
use sp_std::prelude::*;
use sp_std::collections::btree_map::BTreeMap;
use hex_literal::hex;
use sp_std::convert::From;
use sp_runtime::{
	traits::{IdentifyAccount, Verify, StaticLookup, Zero},
	Perbill,
};

use sp_core::U256;

use pallet_timestamp::{self as timestamp};

type AccountIdOf<T: frame_system::Config> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
// type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::NegativeImbalance;

mod sax;
pub mod types;
mod utils;

mod access;

pub mod bpm;
mod bpmn;
mod bpx;

mod dev;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + timestamp::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Reservation fee.
		// #[pallet::constant]
		type ReservationFee: Get<BalanceOf<Self>>;

		/// The origin which may forcibly set or remove a name. Root can always do this.
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		#[pallet::constant]
		type StrNameMaxLength: Get<u32>;

		#[pallet::constant]
		type MaxActorsPerRole: Get<u32>;

		#[pallet::constant]
		type ActorHierDepth: Get<u32>;

		#[pallet::constant]
		type RoleMaxActorAssigneeCount: Get<u32>;

		type Call: From<Call<Self>>;

	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		DBG {
			who: T::AccountId,
			msg: types::Str,
		},

		Start {
			deprocess: types::DeProcessId,
			account: T::AccountId,
		},
		Step {
			deprocess: types::DeProcessId,
			src: types::Action,
			dst: types::Action,
			data: types::ActionData,

			account: T::AccountId,
	
			branch: types::ActionData,
	
			src_id: types::ActionId,
			dst_id: types::ActionId,
	
			flow_id: types::FlowId,
		},
	}

	/// Error for the next_step pallet.
	#[pallet::error]
	pub enum Error<T> {
		//Bounded
		TooLong,

		//model parse
		InvalidBpmn, UnnamedRole, UnnamedTask,

		//EXEC
		RoleNotAssignedToAccount, NotDeProcessCurrentAction,
	}

	pub fn on_err<T: Config>(e: &Error<T>) -> DispatchResult {
		// TODO, handle errors
		Ok(())
	}	

	//Model Data

	#[pallet::storage]
	pub(super) type DeModelNameId<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, types::DeProcessId,
		Blake2_128Concat, types::BoundedStr<T>, //name
		types::BoundedStr<T>, ValueQuery, //id
	>;

	#[pallet::storage]
	pub(super) type DeModelNodes<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, types::DeProcessId,
		Blake2_128Concat, types::BoundedStr<T>, //id
		(bpm::NodeType, types::BoundedStr<T>,), //name, type,
		OptionQuery,
	>;

	#[pallet::storage]
	pub(super) type DeModelEdges<T: Config> = StorageNMap<
    _,
    (
        NMapKey<Twox64Concat, types::DeProcessId>,
        NMapKey<Blake2_128Concat, types::BoundedStr<T>>, // source id
        NMapKey<Blake2_128Concat, U256>, //guard
    ),
	(types::BoundedStr<T>, types::BoundedStr<T>, ), ValueQuery, //flow id, destination id,
	>;

	//Owner Data
	#[pallet::storage]
	pub(super) type OwnerRoles<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, T::AccountId,
		Blake2_128Concat, types::BoundedStr<T>,
		T::AccountId, OptionQuery,
	>;

	//Process Data
	#[pallet::type_value]
	pub(super) fn DefaultZero<T: Config>() -> types::DeProcessId {
		0
	}
		
	#[pallet::storage]
	pub(super) type DeProcessCount<T: Config> =
		StorageValue<_, types::DeProcessId, ValueQuery, DefaultZero<T>>;

	#[pallet::storage]
	pub(super) type DeProcessOwners<T: Config> = StorageMap<_,
		Twox64Concat, types::DeProcessId,
		T::AccountId, OptionQuery,
	>;

	#[pallet::storage]
	pub(super) type DeProcessCurrent<T: Config> = StorageMap<_,
		Twox64Concat, types::DeProcessId,
		types::BoundedStr<T>, ValueQuery,
	>;

	//Actors
	#[pallet::storage]
	pub(super) type DeProcessActionAccounts<T: Config> = StorageDoubleMap<_,
		Twox64Concat, types::DeProcessId,
		Blake2_128Concat, types::BoundedStr<T>,
		T::AccountId, OptionQuery,
	>;

	//Action Data
	// #[pallet::storage]
	// pub(super) type ProcessActionTimeStamp<T: Config> =
	// 	StorageMap<_, Blake2_128Concat, types::DeProcessId, types::BoundedStr<T>, ValueQuery>;

	// #[pallet::storage]
	// pub(super) type ProcessActionActedAccount<T: Config> =
	// 	StorageMap<_, Blake2_128Concat, types::DeProcessId, types::BoundedStr<T>, ValueQuery>;

	#[pallet::storage]
	pub(super) type DeProcessActionData<T: Config> = StorageDoubleMap<_,
		Twox64Concat, types::DeProcessId,
		Blake2_128Concat, types::BoundedStr<T>,
		(types::TimeStamp, types::ActionData, ), ValueQuery,
	>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn start(origin: OriginFor<T>, bpmn_str: types::Str, action_data: types::ActionData)
			-> DispatchResult {
			let sender = ensure_signed(origin)?;
			bpx::start::<T>(&sender, &bpmn_str, &action_data)
		}		

		#[pallet::weight(1_000_000)]
		pub fn step(
			origin: OriginFor<T>,
			deprocess: types::DeProcessId,
			action: types::Str,
			action_data: types::ActionData,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			bpx::step::<T>(&sender, &deprocess, &action, &action_data)
		}

		#[pallet::weight(1_000_000)]
		pub fn assign(
			origin: OriginFor<T>,
			role: types::Str,
			account: T::AccountId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// let account = T::Lookup::lookup(account)?;
			bpx::assign::<T>(&sender, &role, &
				account)
		}

		#[pallet::weight(1_000_000)]
		pub fn unassign(
			origin: OriginFor<T>,
			role: types::Str,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			bpx::unassign::<T>(&sender, &role)
		}

		#[pallet::weight(1_000_000)]
		pub fn zz_dbg(
			origin: OriginFor<T>,
			account: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let account = T::Lookup::lookup(account)?;	
			Ok(())
		}
	
	}
}
