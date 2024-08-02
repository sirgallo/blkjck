#[derive(Clone, Copy, Debug)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub struct Deck {
	pub cards: Vec<Card>, // 3 decks [Card; 52 * 3]
  pub round: u8, // the current round after each shuffle
  pub status: Status // the status of the current round
}


#[derive(Clone, Copy, Debug)]
pub enum Rank { 
  Two, 
  Three, 
  Four, 
  Five, 
  Six, 
  Seven, 
  Eight, 
  Nine, 
  Ten, 
  Jack, 
  Queen, 
  King, 
  Ace
}

#[derive(Clone, Copy, Debug)]
pub enum Suit { 
  Hearts, 
  Diamonds, 
  Clubs, 
  Spades
}

#[derive(PartialEq)]
pub enum Status {
  Ready, // awaiting new round
  Ongoing, // round in progress
  Completed // round completed
}


pub const DECK_SIZE: usize = 52;
pub const TOTAL_DECKS: usize = 3;
pub const SUITS: [Suit; 4] = [ Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades ];
pub const RANKS: [Rank; 13]= [
  Rank::Two, // 2
  Rank::Three, // 3
  Rank::Four, // 4
  Rank::Five, // 5
  Rank::Six, // 6
  Rank::Seven, // 7
  Rank::Eight, // 8
  Rank::Nine, // 9
  Rank::Ten, // 10
  Rank::Jack, // 10
  Rank::Queen, // 10
  Rank::King, // 10
  Rank::Ace, // 1 or 11
];