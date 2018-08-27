# snake-rs
This is an implementation of the classic snake game in [Rust](https://github.com/rust-lang/rust) using [Piston](https://github.com/PistonDevelopers/piston) as the game engine. I started with the code in a [tutorial by Tensor Programming](https://www.youtube.com/watch?v=DnT_7M7L7vo), but since then it has evolved. I am planning to introduce a lot more changes to add following missing features:
- [x] Random apple position on launch
- [ ] Score
- [ ] Gameover screen
- [ ] Pause option
- [ ] Gameover indicator (blinking snake to indicate it is dead)

## Prerequisites
You need to have Rust installed on your machine to run this program. Follow the [installation intructions on Rust's official website](https://www.rust-lang.org/en-US/install.html).

## Building and Running
Once you have cloned this repository to your machine, open terminal, navigate to your local repository folder and enter the following command to build the project:
```
cargo build
```
To run, enter:
```
cargo run
```

## Game controls
Use the arrow keys to direct the snake and press Esc to exit.
