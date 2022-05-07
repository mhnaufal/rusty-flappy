use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear the screen
        ctx.cls();
        // takes x and y coordinates as the first two arguments and a string as the last one
        ctx.print(1, 1, "Hello from Flappy Rust");
    }
}

fn main() -> BError {
    // create a terminal instance
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Rusty")
        .build()?; // will pass the errors to parent a.k.a main() function

    // pass the created terminal into the game state
    main_loop(context, State {})
}
