#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use sp_std::vec;
use core::convert::TryFrom;
use fp_evm::{ExitError, ExitSucceed, LinearCostPrecompile, PrecompileFailure};

use sp_std::{
	convert::{ TryInto},
	marker::PhantomData,
};

pub use pallet_next_step;

pub struct NextStep<T: 'static = ()>( PhantomData<T>, );

impl<T> LinearCostPrecompile for NextStep<T> where T: pallet_next_step::Config {
	const BASE: u64 = 15;
	const WORD: u64 = 3;

	fn execute(input: &[u8], _: u64) -> Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {

		if input.len() < 99 {
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("input must contain 99 bytes".into()),
			});
		}

		let mut selector = [0u8; 4];
		selector.copy_from_slice(&input[0..4]);


		let mut deprocess = [0u8; 4];
		deprocess.copy_from_slice(&input[32..36]);
		let deprocess = u32::from_be_bytes(deprocess) as u128;

		let action_len = input[99] as usize;
		let action = input[100 .. 100 + action_len].to_vec();
	
		let is_acted = pallet_next_step::bpm::is_acted::<T>(&deprocess, &action);

		let mut output = [0; 32];
		output[0] = if is_acted {1} else {0};

		// let output: u8 = if is_acted { 1 } else { 0 };
	
		Ok((ExitSucceed::Returned, output.to_vec()))
	}
}
