use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::utils::DataMap;


/*
User Account:

The user account acts as a way to associate existing or new repositories with a particular wallet.
This is not a program derived account.

The user account should act more as a lookup, where a user account will have 1-n repositories.
*/
#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
  // pub owner: Pubkey, // pub key of wallet for user --> this has to be owner, weird enforcement, just make implicit 
  pub display_name: String,
  pub repositories: DataMap<String, Pubkey>
}


pub fn try_from_slice(mut data: &[u8]) -> std::io::Result<User> {
  User::deserialize(&mut data)
}