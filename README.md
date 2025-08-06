[![Ko-fi](https://img.shields.io/badge/Ko--fi-Support%20me-orange?logo=kofi)](https://ko-fi.com/ubruntu65048)

# Twitch RPG

Idle RPG controlled through Twitch chat

### Current challenges
* Smooth character movement
* AI
* Spells and abilities
* Do I need a motion system?
* bounding box around all players in party, when a player pushes an edge of the box outward the camera position moves
* player pathfinding system
```rust
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Moving {
    pub from: Position,
    pub to: Position,
    pub start_time: f32,       // seconds since game start
    pub duration: f32,         // total duration of movement animation
}
```

### Player Pathfinding:
* When in a room, target enemies 1 at a time until they're gone
* Then target any treasure in the room
* Then target stairs if any
* Then target a corridor to go to another room

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
