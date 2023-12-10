use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    A = 14,
    K = 13,
    Q = 12,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    J = 1,
}

impl TryFrom<char> for Card {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'T' => Ok(Card::T),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            'J' => Ok(Card::J),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: impl Iterator<Item = Card>) -> Self {
        Self {
            cards: cards.collect_vec(),
        }
    }

    pub fn get_hand_type(&self) -> HandType {
        let jokers = self.cards.iter().filter(|c| **c == Card::J).count();
        let mut card_count = self
            .cards
            .iter()
            .filter(|c| **c != Card::J)
            .sorted()
            .enumerate()
            .group_by(|(_, card)| *card)
            .into_iter()
            .map(|(_, group)| group.count())
            .sorted()
            .rev();
        match card_count.next() {
            Some(5) => HandType::FiveOfAKind,
            Some(4) => {
                if jokers > 0 {
                    HandType::FiveOfAKind
                } else {
                    HandType::FourOfAKind
                }
            }
            Some(2) => match card_count.next() {
                Some(2) => {
                    if jokers == 1 {
                        HandType::FullHouse
                    } else {
                        HandType::TwoPair
                    }
                }
                Some(1) => {
                    if jokers == 2 {
                        HandType::FourOfAKind
                    } else if jokers == 1 {
                        HandType::ThreeOfAKind
                    } else {
                        HandType::OnePair
                    }
                }
                None => HandType::FiveOfAKind,
                _ => panic!("Invalid number of cards"),
            },
            Some(3) => match card_count.next() {
                Some(2) => HandType::FullHouse,
                Some(1) => {
                    if jokers > 0 {
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                }
                None => HandType::FiveOfAKind,
                _ => panic!("Invalid number of cards"),
            },
            Some(1) => match jokers {
                4 => HandType::FiveOfAKind,
                3 => HandType::FourOfAKind,
                2 => HandType::ThreeOfAKind,
                1 => HandType::OnePair,
                _ => HandType::HighCard,
            },
            None => HandType::FiveOfAKind,
            _ => panic!("Invalid number of cards"),
        }
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hand")
            .field("cards", &self.cards)
            .field("HandType", &self.get_hand_type())
            .finish()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_type = self.get_hand_type();
        let other_type = other.get_hand_type();
        match my_type.cmp(&other_type) {
            std::cmp::Ordering::Equal => {
                for (my_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    if my_card != other_card {
                        return other_card.cmp(my_card);
                    }
                }
                return std::cmp::Ordering::Equal;
            }
            other => return other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}