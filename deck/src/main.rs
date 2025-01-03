use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck{
    cards:Vec<String>
}

impl Deck{
    fn new() -> Self {
        let suits = ["Ace", "Spade", "Hearts"];
        let values = ["One", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits{
            for value in values{
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck {cards}
    }

    fn shuffle_cards(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, number_of_cards:usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - number_of_cards)  
    }
}



fn main(){
    let mut deck = Deck::new();
    deck.shuffle_cards();

    println!("Here is your deck of cards {:#?}", deck);

    let hand = deck.deal(3);

    println!("Here is your hand: {:#?}", hand);
    
}