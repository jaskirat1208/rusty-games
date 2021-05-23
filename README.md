# rusty-games
A platform on top of which you can create different mini-puzzle games and their AI.
A series of small game theoretic puzzle games, and an AI to play with. 

Currently supported games: 

## Guess the number
A game played between n players. The last person who guesses the number wins. Contains two different types of AI: the first one is a random number generator, while the second one is a binary search based AI.

Adding a new AI to the game: 
- Create a new struct which must have the following traits implemented:
```
player_traits::Player
player_traits::Play
player_traits::UpdateState
player_traits::Name
```
- Once you created this, in your main file, create a trait object(a box pointer to a trait object) mentioned in the library as PlayerBox, and pass it in a vector. In this vector, you can also pass in the inbuilt AIs too. Then call the function `guess_game::game::new_w_custom_bots` and pass the vector into it. Your main file would look something similar to this:
```
    let bots = Vec::new();
    boxed_ai.push(Box::new(ComputerHard), Box::new(MY_CUSTOM_AI_BOT));
    let mut game = guess_game::game::GuessingGame::new_w_custom_bots(bots);

    engine::start(game);
```
- `cargo run` 
- Enjoy the fight :)
- For references, you can take a look at some of the examples I have created in the examples directory.


```
NOTE: More games would be added to the list later. Would really like if someone comes up with a nice AI and we could have a nice one-on-one too :)
```
