// src/state.rs
use crate::components::ball::Ball;
use crate::components::bat::Bat;
use crate::gameplay::action::Action;
use crate::constants::{HEIGHT, WIDTH};

pub struct State {
        pub ball: Ball,
        pub player_bat: Bat,
        pub bot_bat: Bat,
        pub score: (u8, u8),
}

impl State {
        pub fn new(ball: Ball, player_bat: Bat, bot_bat: Bat, score: (u8, u8)) -> Self {
                State { ball, player_bat, bot_bat, score }
        }

        pub fn draw(&self) {
                print!("\x1B[2J\x1B[1;1H");
                println!("Score: {} - {}", self.score.0, self.score.1);
                for i in 0..HEIGHT*WIDTH {
                        let x: i8 = i % WIDTH;
                        let y: i8 = (i / WIDTH) % 10;
                        let is_border: bool = i < WIDTH || i >= WIDTH * (HEIGHT - 1);
                        match (is_border, (x, y)) {
                                (true, _) => print!("_"),
                                (_, pos) if pos == self.ball.pos => self.ball.draw(),
                                (_, pos) if pos == self.player_bat.pos => self.player_bat.draw(),
                                (_, pos) if pos == self.bot_bat.pos => self.bot_bat.draw(),
                                _ => print!("."),
                        }

                        if x == WIDTH - 1 {
                                println!();
                        }
                }
        }

        pub fn update(&mut self, player_act: Action) {
                let bot_act = self.bot_bat.calculate_action(&self.ball.pos, &self.ball.dir);

                self.player_bat.exec(player_act);
                self.bot_bat.exec(bot_act);
                self.ball.update(&self.player_bat, &self.bot_bat);

                if self.ball.pos.0 < 0 {
                        self.score.1 += 1;
                        self.ball.pos = (WIDTH / 2, HEIGHT / 2);
                        self.ball.dir = (1, 1);
                }

                if self.ball.pos.0 > WIDTH - 1 {
                        self.score.0 += 1;
                        self.ball.pos = (WIDTH / 2, HEIGHT / 2);
                        self.ball.dir = (-1, 1);
                }
        }
}