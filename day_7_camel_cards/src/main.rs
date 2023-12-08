use std::{env, fs, collections::{BTreeMap, BinaryHeap}, cmp::{Ordering, Reverse}, str::FromStr};
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum HandType {
    HighCard, Pair, TwoPairs, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum CardType {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace
}
impl FromStr for CardType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(CardType::Two),
            "3" => Ok(CardType::Three),
            "4" => Ok(CardType::Four),
            "5" => Ok(CardType::Five),
            "6" => Ok(CardType::Six),
            "7" => Ok(CardType::Seven),
            "8" => Ok(CardType::Eight),
            "9" => Ok(CardType::Nine),
            "T" => Ok(CardType::Ten),
            "J" => Ok(CardType::Jack),
            "Q" => Ok(CardType::Queen),
            "K" => Ok(CardType::King),
            "A" => Ok(CardType::Ace),
            _ => Err(())
        }
    }
}
struct Hand {
    cards: [CardType; 5],
    hand_type: HandType,
    bid: usize,
    jacks_wild: bool
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {return false;}
        }
        return true;
    }
}
impl Eq for Hand {}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        }
        else if self.hand_type != other.hand_type {
            return (self.hand_type as usize).cmp(&(other.hand_type as usize));
        }
        else {
            if self.jacks_wild {
                for i in 0..5 {
                    if self.cards[i] != other.cards[i] {
                        if self.cards[i] == CardType::Jack {
                            return Ordering::Less;
                        }
                        else if other.cards[i] == CardType::Jack {
                            return Ordering::Greater;
                        }
                        else {
                            return self.cards[i].cmp(&other.cards[i]);
                        }
                    }
                }
            }
            else {
                for i in 0..5 {
                    if self.cards[i] != other.cards[i] {
                        return self.cards[i].cmp(&other.cards[i]);
                    }
                }
            }
            return Ordering::Equal;
        }
    }
}

fn parse_line(source: &String, index: &mut usize) -> ([CardType; 5], usize) {
    let mut cards: [CardType; 5] = [CardType::Ace; 5];
    for i in 0..5 {
        cards[i] = CardType::from_str(source.get(*index..*index+1).unwrap()).expect("failed to read card");
        *index += 1;
    }
    *index += 1;

    let mut bid = 0;
    while source.as_bytes()[*index] != b'\n' {
        bid = (bid * 10) + (source.as_bytes()[*index] as usize - 48);
        *index += 1;
    }
    *index += 1;

    return (cards, bid);
}

fn get_hand_type_pt_1(cards: [CardType; 5]) -> HandType {
    let mut seen: BTreeMap<CardType, usize> = BTreeMap::new();
    for i in 0..5 {
        if seen.contains_key(&cards[i]) {
            seen.insert(cards[i], seen.get(&cards[i]).unwrap() + 1);
        }
        else {
            seen.insert(cards[i], 1);
        }
    }
    return match seen.len() {
        1 => HandType::FiveOfAKind,
        2 => 
            if seen.values().any(|x| *x == 4) {
                HandType::FourOfAKind
            }
            else {
                HandType::FullHouse
            }
        ,
        3 =>
            if seen.values().any(|x| *x == 3) {
                HandType::ThreeOfAKind
            }
            else {
                HandType::TwoPairs
            }
        ,
        4 => HandType::Pair,
        _ => HandType::HighCard
    };
}
fn get_hand_type_pt_2(cards: [CardType; 5]) -> HandType {
    let mut seen: BTreeMap<CardType, usize> = BTreeMap::new();
    for i in 0..5 {
        if seen.contains_key(&cards[i]) {
            seen.insert(cards[i], seen.get(&cards[i]).unwrap() + 1);
        }
        else {
            seen.insert(cards[i], 1);
        }
    }
    if seen.contains_key(&CardType::Jack) && seen.len() > 1 {
        let &jacks = seen.get(&CardType::Jack).unwrap();
        seen.remove(&CardType::Jack);
        let mut max_index = seen.keys().last().unwrap();
        for c in seen.keys() {
            if seen.get(c) > seen.get(max_index) {
                max_index = c;
            }
        }
        seen.insert(
            *max_index,
            seen.get(max_index).unwrap() + jacks
        );
    }
    return match seen.len() {
        1 => HandType::FiveOfAKind,
        2 => 
            if seen.values().any(|x| *x == 4) {
                HandType::FourOfAKind
            }
            else {
                HandType::FullHouse
            }
        ,
        3 =>
            if seen.values().any(|x| *x == 3) {
                HandType::ThreeOfAKind
            }
            else {
                HandType::TwoPairs
            }
        ,
        4 => HandType::Pair,
        _ => HandType::HighCard
    };
}

fn get_winnings(source: &String, jacks_wild: bool) -> usize {
    let mut rounds = BinaryHeap::new();
    let mut index = 0;
    while index < source.len() {
        let r = parse_line(&source, &mut index);
        rounds.push(Reverse(Hand{
            cards: r.0, 
            hand_type: if !jacks_wild{get_hand_type_pt_1(r.0)} else {get_hand_type_pt_2(r.0)}, 
            bid: r.1, 
            jacks_wild
        }));
    }

    let mut total = 0;
    for r in 1..rounds.len()+1 {
        let Reverse(this_round) = rounds.pop().unwrap();
        total += r * this_round.bid;
    }
    return total;
}

// code designed for LF .txt input
fn main() {
    let args: Vec<String> = env::args().collect();
    let source = fs::read_to_string(&args[1]).expect("failed to read input");

    println!("{}", get_winnings(&source, false));
    println!("{}", get_winnings(&source, true));
}
