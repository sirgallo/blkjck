use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;


/*
Repository Account:

The repository account is a program derived account that is associated with newly created repositories.

For commit history, instead of storing a mapping or reference to the commits in the repository account, since accounts are limited in size, 
let's just do a query against the transactions where the repository account is the recipient.

Files are not stored on Solana, and all repository accounts will include a link/ref in the form of data_addr, which will provide the location for the repository data.
The repository account acts more as a way to provide metadata on the repository itself.
*/
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Repository {
  // pub owner: Pubkey, // pub key generated as program derived address --> this has to be owner, weird enforcement for pdas, just make implicit 
  pub repo_owner: Pubkey, // pub key associated with the wallet/user that owns the repository
  pub name: String,
  pub contributors: Vec<Pubkey>,
  pub is_public: bool,
  pub data_addr: Option<String> // link to location where repository data is stored
}