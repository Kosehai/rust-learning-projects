mod deck;
fn main() {
    let cards = deck::new_deck();
    for card in &cards {
        print!("{} ", card);
    }
    println!("{}", &cards.len())
}
