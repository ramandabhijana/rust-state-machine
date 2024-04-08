use std::{collections::BTreeMap, ops::Add};

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
	/// The current block number.
	block_number: u32,
	/// A map from an account to their nonce.
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &str) {
		let nonce = self.nonce.get(who).unwrap_or(&0).add(1);
		self.nonce.insert(who.to_string(), nonce);
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::new();
		let alice = "alice";
		system.inc_block_number();
		system.inc_nonce(alice);

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(alice).unwrap(), &1);
	}
}
