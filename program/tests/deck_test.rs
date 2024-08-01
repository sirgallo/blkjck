use blkjck::state::deck::Deck;

#[test]
fn test_deck() {
  let deck = Deck::new();
  println!("{:?}", deck.cards);
}