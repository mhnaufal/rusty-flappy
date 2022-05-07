use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 50;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

impl GameState for State {
    // tick() will run on every frames
    fn tick(&mut self, ctx: &mut BTerm) {
        // takes x and y coordinates as the first two arguments and a string as the last one
        // ctx.print(1, 1, "Welcome to Flappy Rusty 🚀");

        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Rusty");
        ctx.print_centered(8, "[P] Play");
        ctx.print_color_centered(
            10,
            RGB::from_u8(0, 0, 0),
            RGB::from_u8(255, 255, 255),
            "[Q] Quit",
        );

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(LIGHTCYAN);
        ctx.print_color(
            0,
            0,
            RGB::from_u8(0, 0, 0),
            RGB::from_u8(255, 255, 255),
            "Press SPACE to fly!",
        );

        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);

        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    // Game over menu
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(
            12,
            RGB::from_u8(255, 0, 0),
            RGB::from_u8(0, 0, 0),
            "GAME OVER",
        );
        ctx.print_centered(14, "[P] Play Again");
        ctx.print_centered(16, "[Q] Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quit(),
                _ => {}
            }
        }
    }
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(10, self.y, RED, LIGHT_CYAN, to_cp437('X'));
    }

    fn gravity_and_move(&mut self) {
        // act like the accelerator
        self.velocity = self.velocity + 0.2;

        // adds the accelerator to the y position so that it looks like gravity falling effect
        self.y = self.y + self.velocity as i32;
        self.x = self.x + 1;

        // rusty goes over top of the window
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        // upward is -y
        // although we change the velocity here, the y position will be change later
        // inside gravity_and_move() function and then goes to the render()
        self.velocity = -2.0;
    }
}

fn main() -> BError {
    // create a terminal instance
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT).unwrap()
        .with_title("Flappy Rusty")
        .with_vsync(false)
        .build()?; // will pass the errors to parent a.k.a main() function

    // pass the created terminal into the game state
    main_loop(context, State::new())
}
