const WIDTH: i8 = 16;
const HEIGHT: i8 = 5;

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

    fn update(&mut self, player_act: Action) {
        let bot_act = self.bot_bat.calculate_action(self.ball.pos);

        self.player_bat.exec(player_act);
        self.bot_bat.exec(bot_act);
        self.ball.update(&self.player_bat, &self.bot_bat);
    }
}
struct Ball {
    pos: (i8, i8),
    dir: (i8, i8)
}

impl Ball {
    fn new(pos: (i8, i8), dir: (i8, i8)) -> Self {
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

    fn update(&mut self, bat_player: &Bat, bat_bot: &Bat) {
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;

        let player_hit: bool = self.pos.0 == bat_player.pos.0 && self.pos.1 == bat_player.pos.1;
        let bot_hit: bool = self.pos.0 == bat_bot.pos.0 && self.pos.1 == bat_bot.pos.1;

        if player_hit || bot_hit {
            self.bounce_horizontal();
        }

        if self.pos.1 < 2 || self.pos.1 >= HEIGHT - 2 {
            self.bounce_vertical();
        }
    }
}

struct Bat {
    pos: (i8, i8)
}

impl Bat {
    fn new(pos: (i8, i8)) -> Self {
        Bat { pos }
    }

    fn up(&mut self) {
        self.pos.1 -= 1;
        self.pos.1 = self.pos.1.max(1);
    }

    fn down(&mut self) {
        self.pos.1 += 1;
        self.pos.1 = self.pos.1.min(HEIGHT - 2);
    }

    fn draw(&self) {
        print!("@");
    }

    fn exec(&mut self, action: Action) {
        match action {
            Action::UP => self.up(),
            Action::DOWN => self.down()
        }
    }

    fn calculate_action(&self, ball_pos: (i8, i8)) -> Action {
        if ball_pos.1 > self.pos.1 {
            Action::UP
        } else {
            Action::DOWN
        }
    }
}

fn main() {
    let ball = Ball::new((5, 1), (1, 1));
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
            _ => continue
        };
        state.update(action);
        state.draw();
    }
}