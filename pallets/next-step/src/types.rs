// use sp_std::prelude::*;
use sp_std::str;
use sp_std::vec::Vec;

use sp_io::hashing;

use frame_support::BoundedVec;

type _Str = Vec<u8>;
pub type Str = _Str;

type Name = Str;
type Id = Str;

// pub type DeModelId = Id;
// pub type DeProcessId = Id;
pub type DeProcessId = u128;

pub type ActionId = Id;
pub type ActorId = Id;
pub type ActionName = Name;
pub type ActorName = Name;
pub type Action = ActionName;
pub type Actor = ActorName;

pub type Pool = Name;
pub type Lane = Name;
pub type Assignee = Name;
// pub type Task = str;

pub type Message = _Str;

pub type Timestamp = i64;

//TODO, enrich action data
pub type Data = bool;

// pub fn toString(s: &_Str) -> String {
//     str::from_utf8(s).unwrap()
// }

pub fn from_str(s: &str) -> Str {
	s.as_bytes().to_vec()
}

pub fn to_str(s: &_Str) -> &str {
	&str::from_utf8(s).unwrap()
}

pub type Hash = [u8; 32];

fn hash(s: &Str) -> Hash {
	hashing::blake2_256(&s.to_vec())
}

// pub type STR_NAME_MAX_LENGTH: u128 = 256;
// pub type STR_NAME_MAX_LENGTH = ConstU32<256>;

// type STR_NAME_MAX_LENGTH: Get<u32>;

// pub type BoundedStr = BoundedVec<u8, 128>;
// pub type BoundedStr = Vec<[u8; 256]>;
// pub type BoundedStr<T> = BoundedVec<u8, T::STR_NAME_MAX_LENGTH>;

// pub fn from_bounded(s: &BoundedStr) -> Str {
// 	s.try_into().unwrap()
// }

// pub fn to_bounded(s: &Str) -> BoundedStr {
// 	s.try_into()
// }

// impl Str {
// 	pub fn hash(&self) -> Hash {
// 		hash(self)
// 	}

// 	pub fn bounded(&self) -> BoundedStr {
// 		to_bounded(self)
// 	}
// }
