# Rust Flappy

## Sources

[]()

### Core Concepts

#### Main

- **Game loop** runs by calling main application's `tick()` function in every frame
- `tick()` function doesn't know anything about current game state/status, so we need to handle it by storing the **game state**
- **Trait** can be define as _interface_ or _abstract class_ or contract of functions
- bracket-lib has trait for game state called **GameState** and we need to define the `tick()` function to implement it
- **ctx** inside the `tick()` function parameters is the currently running window in the machine
- **?** mark treated as error handler in Rust (similiar to unwrap and match statement). Pass the errors to the parent function where **?** get invoked
- bracket-lib use **builder pattern** to create complicated objects by chaining multiple function calls into a single giant function

#### More

- **~** tilde in cargo.toml pacakge version means choose version x.x.x but accept patches from x.x
