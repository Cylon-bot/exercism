/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(Debug, PartialEq, PartialOrd)]
enum PokerHands {
    FiveOfAKind = 10,
    StraightFlush = 9,
    FourOfAKInd = 8,
    FullHouse = 7,
    Flush = 6,
    Straight = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

enum CardType {
    Spade,
    Diamond,
    Heart,
    Club,
}

pub fn evaluate_poker_hand(cards: &[Card]) -> PokerHands {
    let mut value_counts = std::collections::HashMap::new();
    for card in cards {
        let count = value_counts.entry(&card.value).or_insert(0);
        *count += 1;
    }
    let mut values: Vec<u8> = value_counts.keys().cloned().collect();
    let is_straight = (0..values.len() - 1).all(|i| values[i] + 1 == values[i + 1]);

    let pair_count = value_counts.values().filter(|&&count| count == 2).count();
    if value_counts.values().any(|&count| count == 2) {
        return PokerHands::OnePair;
    }
    if pair_count == 2 {
        return PokerHands::TwoPair;
    }
    if value_counts.values().any(|&count| count == 3) {
        return PokerHands::ThreeOfAKind;
    }
    if value_counts.values().any(|&count| count == 4) {
        return PokerHands::FourOfAKInd;
    }
    if value_counts.values().any(|&count| count == 5) {
        return PokerHands::FiveOfAKind;
    }
    PokerHands::HighCard
}
#[derive(PartialEq, Eq, Hash)]
#[repr(u8)]
enum Value {
    Numeric(u8),
    Jack = 10,
    Queen = 12,
    King = 13,
    Ace = 14,
}

struct Card {
    suit: CardType,
    value: Value,
}
fn parse_card_value(value_str: &str) -> Value {
    match value_str {
        "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" => {
            Value::Numeric(value_str.parse().unwrap())
        }
        "J" => Value::Jack,
        "Q" => Value::Queen,
        "K" => Value::King,
        "A" => Value::Ace,
        _ => panic!("Invalid card value"),
    }
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut best_hands: Vec<&'a str> = vec![];
    let mut category_on_board: PokerHands = PokerHands::HighCard;
    for hand in hands {
        let cards: Vec<Card> = hand
            .split_whitespace()
            .map(|s| {
                let (suit_str, value_str) = s.split_at(1);
                let suit = match suit_str {
                    "S" => CardType::Spade,
                    "D" => CardType::Diamond,
                    "H" => CardType::Heart,
                    "C" => CardType::Club,
                    _ => panic!("Invalid suit"),
                };
                let value = parse_card_value(value_str);
                Card { suit, value }
            })
            .collect();
        let hand_category: PokerHands = evaluate_poker_hand(&cards);
        if hand_category > category_on_board {
            category_on_board = hand_category;
            best_hands.clear();
            best_hands.push(hand);
        } else if hand_category == category_on_board {
            best_hands.push(hand);
        }
    }
    best_hands
}
