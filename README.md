# Bong

A full bevy game showcasing [`bevy-pigeon`](https://github.com/mitchellmarinodev/carrier-pigeon) and [`carrier-pigeon`](https://github.com/mitchellmarinodev/carrier-pigeon).

A game that is a combination of breakout and pong.

## How to play

The paddles can be controlled with the up/down arrow keys or `w` and `s`.
The paddles can be rotated with the left/right arrow keys or `e` and `q`.

The goal is to get the ball to hit the other player's crown.

## How to configure name and IP

Since bevy doesn't have text-fields built into it's UI yet, you can't input
the IP and name during the game. Instead, the ip address is passed as the 1st argument
and name is passed as the second. Like so: `cargo run -- 192.168.0.99:4455 John`.
This will run the game on the IP `192.168.0.99` on port `4455` with the name `John`.
