pub use super::*;
use sp_runtime::traits::UniqueSaturatedInto;

pub fn now<T: Config>() -> types::TimeStamp {
	pallet_timestamp::Pallet::<T>::get().unique_saturated_into()
}
