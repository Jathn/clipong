// src/main.rs
use clipong::{Action, Ball, Bat, State, WIDTH, HEIGHT};  // assuming your crate is named "pong"

fn main() {
    let ball = Ball::new((WIDTH / 2, HEIGHT / 2), (1, 1));
    let player_bat = Bat::new((0, HEIGHT / 2));
    let bot_bat = Bat::new((WIDTH - 1, HEIGHT / 2));

    let mut state = State::new(ball, player_bat, bot_bat);

    let mut input = String::new();
    state.draw();
    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let action: Action = match input.trim() {
            "w" => Action::UP,
            "s" => Action::DOWN,
            "" => Action::STAY,
            _ => continue
        };
        state.update(action);
        state.draw();
    }
}