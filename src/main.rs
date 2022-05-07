use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
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
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Rusty");
        ctx.print_centered(8, "[P] Play");
        ctx.print_color_centered(
            10,
            RGB {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
            RGB {
                r: 255.0,
                g: 255.0,
                b: 255.0,
            },
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
        self.mode = GameMode::Playing;
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::End;
    }

    // Game over menu
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(
            12,
            RGB {
                r: 255.0,
                g: 0.0,
                b: 0.0,
            },
            RGB {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
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

fn main() -> BError {
    // create a terminal instance
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Rusty")
        .build()?; // will pass the errors to parent a.k.a main() function

    // pass the created terminal into the game state
    main_loop(context, State::new())
}
