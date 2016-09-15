use std::io;
use std::ops::Sub;
use std::str::FromStr;
use std::cmp::Ordering;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn main() {
    println!("Pipe the puzzle input to stdin");
    let mut p1_win = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let (h1, h2) = line.split_at(14);
        let h1: Hand = h1.parse().unwrap();
        let h2: Hand = h2.parse().unwrap();
        if h1 > h2 {
            p1_win += 1;
        }
    }
    println!("{}", p1_win);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Copy, Hash)]
enum Rank {
    Value(isize),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum HandRank {
    HighCard(Rank),
    OnePair(Rank),
    TwoPairs(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank),
    FourOfAKind(Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Card {
    suit: Suit,
    rank: Rank,
}


#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    rank: HandRank,
}


impl FromStr for Suit {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Suit, Self::Err> {
        match input {
            "S" => Ok(Suit::Spades),
            "H" => Ok(Suit::Hearts),
            "D" => Ok(Suit::Diamonds),
            "C" => Ok(Suit::Clubs),
            _ => Err("invalid suit"),
        }
    }
}

impl FromStr for Rank {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Rank, Self::Err> {
        match input {
            "A" => Ok(Rank::Ace),
            "K" => Ok(Rank::King),
            "Q" => Ok(Rank::Queen),
            "J" => Ok(Rank::Jack),
            "T" => Ok(Rank::Value(10)),
            "9" => Ok(Rank::Value(9)),
            "8" => Ok(Rank::Value(8)),
            "7" => Ok(Rank::Value(7)),
            "6" => Ok(Rank::Value(6)),
            "5" => Ok(Rank::Value(5)),
            "4" => Ok(Rank::Value(4)),
            "3" => Ok(Rank::Value(3)),
            "2" => Ok(Rank::Value(2)),
            _ => Err("invalid rank"),
        }
    }
}

impl FromStr for Card {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Card, Self::Err> {
        let input = input.trim();
        if input.len() != 2 {
            return Err("invalid card string length");
        }
        let (r, s) = input.split_at(1);
        let r = try!(r.parse::<Rank>());
        let s = try!(s.parse::<Suit>());
        Ok(Card { rank: r, suit: s })
    }
}


impl FromStr for Hand {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        let cards = input.split_whitespace()
                         .map(|c| c.parse::<Card>().unwrap())
                         .collect::<Vec<_>>();
        Hand::from(cards).ok_or("invalid hand")
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank == other.rank {
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()).rev() {
                if c1.rank != c2.rank {
                    return c1.rank.partial_cmp(&c2.rank);
                }
            }
            Some(Ordering::Equal)
        } else {
            self.rank.partial_cmp(&other.rank)
        }
    }
}

impl Hand {
    fn from(cards: Vec<Card>) -> Option<Hand> {
        if cards.len() != 5 {
            None
        } else {
            let mut cards = cards;
            cards.sort_by_key(|card| card.rank);
            let r = Hand::rank(&cards);
            Some(Hand {
                cards: cards,
                rank: r,
            })
        }
    }

    fn rank(cards: &Vec<Card>) -> HandRank {
        let first_suit = cards[0].suit;
        let highest_rank = cards[4].rank;
        let is_flush = cards.iter().all(|card| card.suit == first_suit);
        let is_straight = cards.windows(2).all(|pair| (pair[0] - pair[1]).abs() == 1);
        let mut multiples = HashMap::<Rank, u32>::new();
        for &card in cards.iter() {
            *multiples.entry(card.rank).or_insert(0) += 1;
        }
        let multiples = multiples;
        let has_n = |n| multiples.values().any(|&i| i == n);

        if highest_rank == Rank::Ace && is_flush && is_straight {
            HandRank::RoyalFlush
        } else if is_flush && is_straight {
            HandRank::StraightFlush(highest_rank)
        } else if is_flush {
            HandRank::Flush(highest_rank)
        } else if has_n(4) {
            let r = *multiples.iter().find(|&(_, &n)| n == 4).unwrap().0;
            HandRank::FourOfAKind(r)
        } else if has_n(3) && has_n(2) {
            HandRank::FullHouse(highest_rank)
        } else if is_flush {
            HandRank::Flush(highest_rank)
        } else if is_straight {
            HandRank::Straight(highest_rank)
        } else if has_n(3) {
            let r = *multiples.iter().find(|&(_, &n)| n == 3).unwrap().0;
            HandRank::ThreeOfAKind(r)
        } else if has_n(2) && multiples.len() == 3 {
            let ranks = multiples.iter()
                                 .filter(|&(_, &n)| n == 2)
                                 .map(|(&r, _)| r)
                                 .collect::<Vec<_>>();
            HandRank::TwoPairs(ranks[0], ranks[1])
        } else if has_n(2) {
            let r = *multiples.iter().find(|&(_, &n)| n == 2).unwrap().0;
            HandRank::OnePair(r)
        } else {
            HandRank::HighCard(highest_rank)
        }
    }
}

impl Sub for Card {
    type Output = isize;

    fn sub(self, _rhs: Card) -> isize {
        let value = |rank| match rank {
            Rank::Ace => 14,
            Rank::King => 13,
            Rank::Queen => 12,
            Rank::Jack => 11,
            Rank::Value(n) => n,
        };
        value(self.rank) - value(_rhs.rank)
    }
}
