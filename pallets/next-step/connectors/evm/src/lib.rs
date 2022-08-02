#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use sp_std::vec;
use core::convert::TryFrom;
use fp_evm::{ExitError, ExitSucceed, LinearCostPrecompile, PrecompileFailure, PrecompileHandle,
	Precompile, PrecompileResult, PrecompileOutput};

use sp_runtime::traits::TrailingZeroInput;

use sp_core::Decode;
use sp_std::{
	convert::{ TryInto},
	marker::PhantomData,
};

// use sp_core::Deref;
// use pallet_evm::HashedAddressMapping;
// use sp_runtime::BlakeTwo256;
use sp_core::{Hasher, H160, H256, U256};
use sp_runtime::{
	traits::{BadOrigin, Saturating, UniqueSaturatedInto, Zero},
	// BlakeTwo256,
	AccountId32, DispatchErrorWithPostInfo,
};

use pallet_next_step;
use pallet_evm;
use pallet_evm::AddressMapping;

pub struct NextStep<T: 'static = ()>( PhantomData<T>, );

fn is_acted<T: pallet_next_step::Config>(input: &[u8]) -> PrecompileResult {

	if input.len() < 132 {
		return Err(PrecompileFailure::Error {
			exit_status: ExitError::Other("input must contain 132 bytes".into()),
		});
	}

	let mut deprocess = [0u8; 4];
	deprocess.copy_from_slice(&input[32..36]);
	let deprocess = u32::from_be_bytes(deprocess) as u128;

	let action_len = input[99] as usize;
	let action = input[100 .. 100 + action_len].to_vec();

	let is_acted = pallet_next_step::bpm::is_acted::<T>(&deprocess, &action);

	let mut output = [0; 32];
	output[0] = if is_acted {1} else {0};

	// let output: u8 = if is_acted { 1 } else { 0 };

	// Ok((ExitSucceed::Returned, output.to_vec()));
	Ok(PrecompileOutput { exit_status: ExitSucceed::Returned, output: output.to_vec() })
}

fn act<T: pallet_next_step::Config + pallet_evm::Config>(input: &[u8], caller: &H160) -> PrecompileResult {

	if input.len() < 164 {
		return Err(PrecompileFailure::Error {
			exit_status: ExitError::Other("input must contain 164 bytes".into()),
		});
	}

	let mut deprocess = [0u8; 4]; //uint32
	deprocess.copy_from_slice(&input[32..36]);
	let deprocess = u32::from_be_bytes(deprocess) as u128;

	let mut action_data = [0u8; 32]; //uint256
	action_data.copy_from_slice(&input[36 .. 68]);
	let action_data = U256::from_big_endian(&action_data);

	let action_len = input[131] as usize;
	let action = input[132 .. 132 + action_len].to_vec();

	let caller = T::AddressMapping::into_account_id(caller.clone());

	pallet_next_step::bpm::step::<T>(&caller, &deprocess, &action, &action_data);

	let ok: bool = true;

	let mut output = [0; 32];
	output[0] = if ok {1} else {0};

	// Ok((ExitSucceed::Returned, output.to_vec()))
	Ok(PrecompileOutput { exit_status: ExitSucceed::Returned, output: output.to_vec() })
}

impl<T> Precompile for NextStep<T> where T: pallet_next_step::Config + pallet_evm::Config {
	// const BASE: u64 = 15;
	// const WORD: u64 = 3;

	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {

		let input: &[u8] = handle.input();
		let caller = handle.context().caller;

		if input.len() < 4 {
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("input must contain 4 bytes".into()),
			});
		}

		let mut selector = [0u8; 4];
		selector.copy_from_slice(&input[0..4]);
		// let selector = u32::from_bytes(selector);

		const Selector_act: [u8; 4] = [0xaa, 0xb4, 0x64, 0x47];

		match selector {
			Selector_act => act::<T>(input, &caller),
			_ => is_acted::<T>(input)
		}
	}
}
