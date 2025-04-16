use crate::gameplay::action::Action;
use crate::constants::HEIGHT;

pub struct Bat {
        pub pos: (i8, i8),
}

impl Bat {
        pub fn new(pos: (i8, i8)) -> Self {
                Bat { pos }
        }

        pub fn up(&mut self) {
                self.pos.1 -= 1;
                self.pos.1 = self.pos.1.max(1);
        }

        pub fn down(&mut self) {
                self.pos.1 += 1;
                self.pos.1 = self.pos.1.min(HEIGHT - 2);
        }

        pub fn draw(&self) {
                print!("@");
        }

        pub fn exec(&mut self, action: Action) {
                match action {
                        Action::UP => self.up(),
                        Action::DOWN => self.down(),
                        Action::STAY => {}
                }
        }

        pub fn calculate_action(&self, ball_pos: &(i8, i8), ball_dir: &(i8, i8)) -> Action {
                let ball_modifier_y = &ball_dir.1;

                if ball_dir.0 < 0 {
                        return Action::STAY;
                }

                match (ball_pos.1, self.pos.1) {
                        (ball_y, self_y) if ball_y + ball_modifier_y < self_y => Action::UP,
                        (ball_y, self_y) if ball_y + ball_modifier_y > self_y => Action::DOWN,
                        _ => Action::STAY,
                }
        }
}