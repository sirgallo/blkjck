use borsh::{BorshSchema, BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::utils::data_map::DataMap;

use super::deck::{Card, Deck};


/*
Table Account:

The table account is a program derived account that is associated with newly created blackjack tables.

Each table consists of:
	- the name of the table
	- the creator
	- the current deck
	- the current dealer cards
	- the players, stored in a b-tree, where each node is the 
		pubkey
		the chair:
			- the current balance
			- the current hand
*/
#[derive(BorshSerialize, BorshDeserialize, BorshSchema, Clone, Debug)]
pub struct Chair {
	pub balance: u32,
	pub hand: Vec<Card>
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Table {
  // pub owner: Pubkey, // pub key generated as program derived address --> this has to be owner, weird enforcement for pdas, just make implicit 
  pub table_creator: Pubkey, // pub key associated with the wallet/user that owns the repository
  pub table_name: String,
	pub table_deck: Deck, // the deck also contains the current round and the status of the round
	pub table_dealer: Vec<Card>,
	pub table_players: DataMap<Pubkey, Chair>
}