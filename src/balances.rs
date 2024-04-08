use std::collections::BTreeMap;

/// Balances module
/// Keeps track of how much balance each account has in this state machine
/// NOT how pallet storage works in Polkadot SDK just a simple emulation of the behaviours
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &str, amount: u128) {
		self.balances.insert(who.to_string(), amount);
	}

	pub fn balance(&self, who: &str) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.balance("alice"), 0);

		balances.set_balance("alice", 100);
		assert_eq!(balances.balance("alice"), 100);

		assert_eq!(balances.balance("bob"), 0)
	}
}
