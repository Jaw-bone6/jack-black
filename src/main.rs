use std::io::{self, Write};
use blackjack::*;

fn main() {
    let mut chips;
    let mut deck = Vec::new();

    for value in 1..=13 {
        deck.push(Card { face_up: true, suit: Suit::Clubs, value });   
        deck.push(Card { face_up: true, suit: Suit::Hearts, value });   
        deck.push(Card { face_up: true, suit: Suit::Spades, value });   
        deck.push(Card { face_up: true, suit: Suit::Diamonds, value });   
    }

    loop {
        print!("Starting Chips? ");
        let flush_result = io::stdout().flush();

        if let Err(err) = flush_result {
            eprintln!("An unexpected I/O error occured: {err}");
            continue;
        }

        let mut starting_chips_string = String::new();
        let starting_chips_result = io::stdin().read_line(&mut starting_chips_string);

        if let Err(err) = starting_chips_result {
            eprintln!("An unexpected I/O error occured: {err}");
            continue;
        }

        let starting_chips_result = starting_chips_string.trim().parse::<u32>();

        if starting_chips_result.is_err() {
            println!("That's not a valid number of chips");
            continue;
        }

        chips = starting_chips_result.unwrap();
        break;
    }

    loop {
        if chips <= 0 {
            println!("You're broke!");
            break;
        }
        run_hand(&mut chips, &mut deck);   
    }
}
