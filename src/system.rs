use std::{
	collections::BTreeMap,
	ops::{Add, AddAssign},
};

use num::{One, Zero};

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: AddAssign + Copy + Zero + One;
	type Nonce: Zero + One + Add + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = self.nonce.get(who).unwrap_or(&T::Nonce::zero()).add(T::Nonce::one());
		self.nonce.insert(who.clone(), nonce);
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let mut system = super::Pallet::<TestConfig>::new();
		let alice = "alice";
		system.inc_block_number();
		system.inc_nonce(&alice);

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(alice).unwrap(), &1);
	}
}
