/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
/// #[derive(Debug)]
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
    PokerHands::HighCard
}

enum Value {
    Numeric(u8),
    Jack,
    Queen,
    King,
    Ace,
}

struct Card {
    suit: CardType,
    value: Value,
}
fn parse_card_value(value_str: &str) -> Value {
    match value_str {
        "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => Value::Numeric(value_str.parse().unwrap()),
        "10" => Value::Numeric(10),
        "J" => Value::Jack,
        "Q" => Value::Queen,
        "K" => Value::King,
        "A" => Value::Ace,
        _ => panic!("Invalid card value"),
    }
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let best_hand: PokerHands;
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
        let hand_category = evaluate_poker_hand(&cards);
        if hand_category > best_hand {
            best_hand = hand_category;
        }
    }
    [""]
}
