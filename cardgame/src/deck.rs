use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use colored::*;


pub fn shuffle(cards: &mut Vec<Card>){
    let mut rng = thread_rng();
    cards.shuffle(&mut rng);
}

pub fn new_deck() -> Vec<Card> {
    let ranks: Vec<Rank> = vec![
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King
    ];
    let suits: Vec<Suit> = vec![
        Suit::HEART,
        Suit::DIAMOND,
        Suit::SPADE,
        Suit::CLUB  
    ];
    let mut cards = Vec::new();
    for s in suits {
        for r in &ranks {
            cards.push(Card {
                rank: r.clone(),
                suit: s.clone()
            });
        }
    }
    shuffle(&mut cards);
    return cards;
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suitout = self.suit.to_string();
        let rankout: String;

        let color: Color;

        match self.rank as u32 {
            11 => (rankout = "J".to_string()),
            12 => (rankout = "Q".to_string()),
            13 => (rankout = "K".to_string()),
            1 => (rankout = "A".to_string()),
            _ => (rankout = ((self.rank.clone()) as u32).to_string())
        }

        match self.suit {
            Suit::CLUB => (color = Color::Black),
            Suit::SPADE => (color = Color::Black),
            Suit::HEART => (color = Color::Red),
            Suit::DIAMOND => (color = Color::Red)
            
        };

        write!(f, "{}{}", &suitout.color(color).on_white(), &rankout.color(color).on_white())
    }
}

pub struct Card {
    pub rank: Rank,
    pub suit: Suit
}
#[derive(Clone, Debug, Copy)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13
}

#[derive(Clone)]
pub enum Suit {
    HEART,
    DIAMOND,
    SPADE,
    CLUB
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::CLUB => write!(f, "\u{2667}"),
            Suit::DIAMOND => write!(f, "\u{2662}"),
            Suit::HEART => write!(f, "\u{2661}"),
            Suit::SPADE => write!(f, "\u{2664}"),
        }
    }
}