## confetty_rs

Particle System written in rust and rendered in the terminal via [ratatui](https://github.com/ratatui-org/ratatui).
Mostly a rust port of [confetty](https://github.com/maaslalani/confetty).

![confetty demo in terminal](https://i.imgur.com/EjpdJXA.gif)
![fireworks demo in terminal](https://i.imgur.com/VPwOALP.gif)



### Homebrew Particle System

Also made my own simulation:

![shooting stars demo in terminal](https://i.imgur.com/v6yRjxR.gif)

```bash
# Confetti
cargo run --release

# Fireworks
cargo run --release -- --name fireworks

# Shooting Stars
cargo run --release -- --name stars
```
Press any key for particles. `Cntrl-c` or `q` to quit.
