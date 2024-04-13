use std::collections::BTreeMap;

type AccountId = String;
type Balance = u128;

/// Balances module
/// Keeps track of how much balance each account has in this state machine
/// NOT how pallet storage works in Polkadot SDK just a simple emulation of the behaviours
#[derive(Debug)]
pub struct Pallet {
	// A simple storage mapping from accounts (`String`) to their balances (`u128`).
	balances: BTreeMap<AccountId, Balance>,
}

impl Pallet {
	/// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
		self.balances.insert(who.clone(), amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&0)
	}

	/// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: &AccountId,
		to: &AccountId,
		amount: Balance,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(caller);
		let to_balance = self.balance(to);

		let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(amount).ok_or("Funds exceed limit.")?;

		self.set_balance(caller, new_caller_balance);
		self.set_balance(to, new_to_balance);

		Ok(())
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

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::new();

		let alice = "alice";
		let bob = "bob";

		assert!(balances.transfer(alice, bob, 100).is_err());

		balances.set_balance(alice, 100);
		assert!(balances.transfer(alice, bob, 50).is_ok());

		assert_eq!(balances.balance(alice), 50);
		assert_eq!(balances.balance(bob), 50);
	}
}
