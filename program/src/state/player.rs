use borsh::{BorshDeserialize, BorshSerialize};


/*
User Account:

The user account acts as a way to associate existing or new repositories with a particular wallet.
This is not a program derived account.

The user account should act more as a lookup, where a user account will have 1-n repositories.
*/
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Player {
  // pub owner: Pubkey, // pub key of wallet for user --> this has to be owner, weird enforcement, just make implicit 
  pub display_name: String,
  pub rank: u64,
  pub rounds: u64,
  pub wins: u64,
}


pub fn try_from_slice(mut data: &[u8]) -> std::io::Result<Player> {
  Player::deserialize(&mut data)
}