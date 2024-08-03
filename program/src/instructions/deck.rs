use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::state::deck::{Card, Deck, Status, RANKS, SUITS, DECK_SIZE, TOTAL_DECKS};


/*
  Deck:
    basic operations for shuffling and dealing cards from a randomized card deck
*/
impl Deck {
  pub fn new() -> Self { 
    return Deck{
      cards: shuffle_cards(),
      round: 0,
      status: Status::Ready
    }
  }

  pub fn deal_card(&mut self) -> Option<Card> {
    if self.trigger_reshuffle() { // check if a reshuffle is required
      self.cards = shuffle_cards();
      self.status = Status::Ongoing;
    }

    return self.cards.pop(); // pop the last card off the deck
  }

  pub fn increment_round(&mut self) {
    if self.status == Status::Completed { // if round has completed, 
      self.status = Status::Ready;
      self.round += 1;
    }
  }

  pub fn trigger_reshuffle(&mut self) -> bool {
    let exceeded_threshold = self.cards.len() <= (DECK_SIZE * TOTAL_DECKS) / 2;
    let round_ready = self.status == Status::Ready;
    return exceeded_threshold && round_ready;
  }
}


/*
  shuffle_deck:
    1.) initialize the deck in order
    2.) shuffle the deck in place
*/
fn shuffle_cards() -> Vec<Card> {
  let mut cards: Vec<Card> = Vec::with_capacity(DECK_SIZE * TOTAL_DECKS);
  for _ in 0..TOTAL_DECKS {
    for &suit in &SUITS {
      for &rank in &RANKS { cards.push(Card{ suit, rank }); }
    }
  }

  let mut rand_num_gen = thread_rng();
  cards.shuffle(&mut rand_num_gen);
  return cards;
}