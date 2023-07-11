use std::fmt::{Display, Formatter};

pub type Hand = Vec<Card>;

#[derive(Clone, Copy)]
pub struct Card {
    pub face_up: bool,
    pub suit: Suit,
    pub value: u8,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if !self.face_up {
            f.write_str("##")?;
            return Ok(());
        }      

        let value_string = match self.value {
            1 => String::from("A"),
            2..=9 => format!("{}", self.value),
            10 => String::from("10"),
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            other => panic!("unexpected card value: {}", other),
        };

        /*
        let suit_str = match self.suit {
            Suit::Clubs => "C",
            Suit::Hearts => "H",
            Suit::Spades => "S",
            Suit::Diamonds => "D",
        };
        */

        f.write_str(&value_string)?;
        // f.write_str(suit_str)?;

        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum Suit {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
} 

pub struct GameState {
    pub dealer_hand: Hand,
    pub player_hands: Vec<Hand>,
    pub current_player_hand_idx: usize,
    pub current_person: Person,
}

#[derive(Clone, Copy)]
pub enum Person {
    Player,
    Dealer,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for card in self.dealer_hand.iter() {
            f.write_fmt(format_args!("{} ", card))?;    
        }   

        f.write_str("\n\n")?;

        for (i, hand) in self.player_hands.iter().enumerate() {
            if i == self.current_player_hand_idx {
                f.write_str("* ")?;    
            } 

            for card in hand.iter() {
                f.write_fmt(format_args!("{} ", card))?;   
            }

            f.write_str("\n")?;
        }    

        f.write_str("\n")?;

        Ok(())
    }
}

pub enum TurnAction {
    Hit,
    Stand,
    DoubleDown,
    Split,
    Surrender,
}

#[derive(Debug, Clone, Copy)]
pub enum HandValue {
    Hard(u8),
    Soft(u8),
    BlackJack,
    Bust(u8),
}

pub fn eval_hand(hand: Hand) -> HandValue {
    let mut ace_count = 
        hand.iter()
        .filter(|x| {
            x.value == 1
        })
        .count();

    let mut total =
        hand.iter()
        .fold(0, |acc, x| {
            acc + match x.value {
                1 => 11,
                11..=13 => 10,
                other => other,
            }
        });

    if hand.len() == 2 && total == 21 {
        return HandValue::BlackJack;   
    }

    while ace_count > 0 {
        if total <= 21 {
            return HandValue::Soft(total);    
        }   

        total -= 10;
        ace_count -= 1;
    }

    if total > 21 {
        HandValue::Bust(total) 
    } else {
        HandValue::Hard(total)   
    }
}

pub enum TurnResult {
    Unfinished,
    UnfinishedDoubled,
    Win,
    WinBlackJack,
    Draw,
    Loss,
    Surrender,
}

pub fn run_player_turn(state: &mut GameState, deck: &mut Vec<Card>) {
    todo!()   
}

pub fn run_dealer_turn(state: &mut GameState, deck: &mut Vec<Card>) {
    todo!()   
}

pub fn run_turn(state: &mut GameState, deck: &mut Vec<Card>) {
    match state.current_person {
        Person::Dealer => run_dealer_turn(state, deck),
        Person::Player => run_player_turn(state, deck),
    }
}
