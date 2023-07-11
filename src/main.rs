use blackjack::*;

fn main() {
    loop {
        let mut hand: Hand = Vec::new();

        for _ in 0..3 {
            let mut card_value = String::new();   
            std::io::stdin().read_line(&mut card_value).expect("couldn't read stdin");
            let card_value = card_value.trim().parse::<u8>().expect("not an integer");
            hand.push(Card { face_up: true, suit: Suit::Hearts, value: card_value });
        }

        println!("{:?}", eval_hand(hand));
    }
}
