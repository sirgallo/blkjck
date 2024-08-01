use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, Debug)]
pub enum Suit {
  Hearts,
  Diamonds,
  Clubs,
  Spades,
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
    Ace,
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(num_decks: usize) -> Self {
        let mut cards = Vec::new();
        for _ in 0..num_decks {
            for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
                for rank in &[
                    Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
                    Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
                    Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
                ] {
                    cards.push(Card { suit: *suit, rank: *rank });
                }
            }
        }
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.as_mut_slice().shuffle(&mut rng);
    }
}
