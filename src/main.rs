//! Implements a hello-world example for Arbitrum Stylus, providing a Solidity ABI-equivalent
//! Rust implementation of the Counter contract example provided by Foundry.
//! Warning: this code is a template only and has not been audited.
//! ```
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```

// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use alloy_primitives::Address;
/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, prelude::*, msg, alloy_sol_types::{sol, SolError}, evm};

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
        mapping(address => uint256) followers;
        mapping(address => uint256) fees;
    }
}

sol! {
    event FollowerSet(address owner, address follower);
}

sol! {
    error WrongAmount();
}

pub enum ContractError {
    WrongAmount(WrongAmount)
}

impl From<ContractError> for Vec<u8> {
    fn from(value: ContractError) -> Vec<u8> {
        return match value {
            ContractError::WrongAmount(e) => e.encode()
        }
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl Counter {
    /// Gets the number from storage.
    pub fn number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
        self.number.set(new_number);
        Ok(())
    }

    /// Increments number and updates it values in storage.
    pub fn increment(&mut self) -> Result<(), Vec<u8>> {
        let number = self.number.get();
        self.number.set(number + U256::from(1));
        Ok(())
    }

    pub fn get_followers(&self, owner: Address) -> Result<U256, Vec<u8>> {
        Ok(self.followers.get(owner))
    }

    #[payable]
    pub fn set_followers(&mut self, owner: Address) -> Result<U256, ContractError> {
        if msg::value() < self.fees.get(owner) {
            return Err(ContractError::WrongAmount(().into()))
        }
        
        let followers = self.followers.get(owner);

        self.followers.replace(owner, followers + U256::from(1));

        evm::log(FollowerSet {
            owner,
            follower: msg::sender()
        });

        Ok(followers + U256::from(1))
    }

    pub fn add_values(&self, values: Vec<U256>) -> Result<U256, Vec<u8>> {
        Ok(values.iter().sum())
    }
}