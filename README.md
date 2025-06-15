# Twitch RPG

Idle RPG controlled through Twitch chat

### Current challenges
* Simple ECS god I should be better at this
* What can I store in game state?
* Sprite mapping/animation

### Architecture
* leptos frontend, reacting to server sent events, no events, click, submit etc
* webserver that receives user interactions through POST calls, continuously sends server-sent events every 2s or so
* internal ECS server that models player characters, enemies, combat, pathfinding

### Ideas (to come up with)
* items
* quests

### Build
* compile frontend first (may need a makefile or build.rs file)
* compile webserver with the compiled frontend code (css, wasm, js, html) using rust-embed into single binary
* .env file specifying channel and webserver port