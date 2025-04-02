// src/ball.rs
use crate::components::bat::Bat;
use crate::constants::{HEIGHT};

pub struct Ball {
        pub pos: (i8, i8),
        pub dir: (i8, i8),
}

impl Ball {
        pub fn new(pos: (i8, i8), dir: (i8, i8)) -> Self {
                Ball { pos, dir }
        }

        pub fn bounce_vertical(&mut self) {
                self.dir = (self.dir.0, -self.dir.1);
        }

        pub fn bounce_horizontal(&mut self) {
                self.dir = (-self.dir.0, self.dir.1);
        }

        pub fn draw(&self) {
                print!("o");
        }

        pub fn update(&mut self, bat_player: &Bat, bat_bot: &Bat) {
                self.pos.0 += self.dir.0;
                self.pos.1 += self.dir.1;

                let player_hit: bool = self.pos.0 == bat_player.pos.0 + 1 && self.pos.1 == bat_player.pos.1;
                let bot_hit: bool = self.pos.0 == bat_bot.pos.0 - 1 && self.pos.1 == bat_bot.pos.1;

                if player_hit || bot_hit {
                        self.bounce_horizontal();
                        self.dir = (self.dir.0, self.dir.1);
                }

                if self.pos.1 < 2 || self.pos.1 >= HEIGHT - 2 {
                        self.bounce_vertical();
                }
        }
}