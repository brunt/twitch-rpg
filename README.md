[![Ko-fi](https://img.shields.io/badge/Ko--fi-Support%20me-orange?logo=kofi)](https://ko-fi.com/ubruntu65048)

# Twitch RPG

Idle RPG controlled through Twitch chat

### Current challenges
* Smooth character movement
* Attack buildup rather than attack cooldown.
* Spell buildup rather along with spell cooldown.
* crit and spell animations
* persisting to database


### Architecture
* leptos frontend, reacting to server sent events, no events, click, submit etc
* webserver that receives user interactions continuously sends server-sent events
* internal ECS server that models player characters, enemies, combat, pathfinding

### Build
* compile frontend first (may need a makefile or build.rs file)
* compile webserver with the compiled frontend code (css, wasm, js, html) using rust-embed into single binary
* .env file specifying channel and webserver port

### Quick local run
* front end: `cd` into client folder, run `nix develop`
* back end: `cargo run --package twitch-rpg-server --bin twitch-rpg-server`

All tiles have been drawn by David E. Gervais, and are published under the [Creative Commons licence](http://creativecommons.org/licenses/by/3.0/).
Note: There's been at least one tiny change made to the tiles.

The tiles can be downloaded here: http://pousse.rapiere.free.fr/tome/

The tiles were originally created for [Dungeon Odyssey](http://www.malfador.com/domain.html).

Shop background image is unlicensed with permitted usage by [James B](https://www.artstation.com/james--b)
