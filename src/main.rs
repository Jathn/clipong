const WIDTH: u8 = 16;
const HEIGHT: u8 = 5;

enum Action {
    UP,
    DOWN,
}

struct State {
    ball: Ball,
    player_bat: Bat,
    bot_bat: Bat,
}

impl State {
    fn new(ball: Ball, player_bat: Bat, bot_bat: Bat) -> Self {
        State { ball, player_bat, bot_bat }
    }

    fn draw(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}, {}", self.ball.pos.0, self.ball.pos.1);
        for i in 0..HEIGHT*WIDTH {
            let x: u8 = i % WIDTH;
            let y: u8 = (i / WIDTH) % 10;
            let is_border: bool = i < WIDTH || i >= WIDTH * (HEIGHT - 1);
            if is_border {
                print!("_");
            } else if self.ball.pos == (x, y) {
                self.ball.draw();
            } else {
                print!(".");
            }

            if x == WIDTH - 1 {
                println!();
            }

        }
    }
}
struct Ball {
    pos: (u8, u8),
    dir: (i8, i8)
}

impl Ball {
    fn new(pos: (u8, u8), dir: (i8, i8)) -> Self {
        Ball { pos, dir }
    }

    fn bounce_vertical(&mut self) {
        self.dir = (self.dir.0, -self.dir.1);
    }

    fn bounce_horizontal(&mut self) {
        self.dir = (-self.dir.0, self.dir.1);
    }

    fn draw(&self) {
        print!("o");
    }
}

struct Bat {
    pos: (u8, u8)
}

impl Bat {
    fn new(pos: (u8, u8)) -> Self {
        Bat { pos }
    }

    fn up(&mut self) {
        self.pos.1 -= 1;
    }

    fn down(&mut self) {
        self.pos.1 += 1;
    }

    fn exec(&mut self, action: Action) {
        match action {
            Action::UP => self.up(),
            Action::DOWN => self.down()
        }
    }
}

fn main() {
    let ball = Ball::new((5, 1), (1, 1));
    let player_bat = Bat::new((0, 0));
    let bot_bat = Bat::new((WIDTH - 1, 0));

    let state = State::new(ball, player_bat, bot_bat);

    state.draw();

}

fn update_state(action: &Action) {

}