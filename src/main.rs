mod balances;
mod system;

mod types{
	pub type AccountId = String;
	pub type Balance = u128;

	pub type BlockNumber = u32;
	pub type Nonce = u32;
}


// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	/* TODO:
		- Create a field `system` which is of type `system::Pallet`.
		- Create a field `balances` which is of type `balances::Pallet`.
	*/
	system: system:: Pallet<Self>,
	balances: balances::Pallet<Self>
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type AccountId = types::AccountId;
	type Balance = types::Balance;
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		/* TODO: Create a new `Runtime` by creating new instances of `system` and `balances`. */
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}
fn main() {
	// println!("Hello, world!");
		/* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
	/* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */

	// start emulating a block
	/* TODO: Increment the block number in system. */
	/* TODO: Assert the block number is what we expect. */

	// first transaction
	/* TODO: Increment the nonce of `alice`. */
	/* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
		- The transfer _could_ return an error. We should use `map_err` to print
		  the error if there is one.
		- We should capture the result of the transfer in an unused variable like `_res`.
	*/

	// second transaction
	/* TODO: Increment the nonce of `alice` again. */
	/* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice, 100);

	
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	
	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice.clone(), bob, 30)
		.map_err(|e| eprintln!("{}", e));

	
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{}", e));

	println!("{:#?}", runtime);
}
