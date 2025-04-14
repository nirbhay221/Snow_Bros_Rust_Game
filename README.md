Snow Bros: Frosty Fury Blizzard Bash
A 2D platformer game built with the Bevy game engine inspired by the classic arcade game Snow Bros, where players fight enemies by freezing them into snowballs.
Game Overview
In this Snow Bros-inspired game, you play as a character who can freeze enemies into snowballs and then use those snowballs to defeat other enemies. Roll the snowballs into other enemies or kick them across the screen to clear each wave. The game features a wave-based progression system with increasing difficulty.
Core Mechanics

Freeze Enemies: Use your beam attack to turn enemies into snowballs
Control Snowballs: Push, kick, and roll snowballs to defeat other enemies
Power-ups: Collect different colored drinks for special abilities:

Red: Speed boost
Green: Extra life
Blue: Health restore


Wave System: Fight through progressively more difficult waves of enemies

Enemy Types

Basic Enemy: Standard enemy with basic movement
FireBeam Enemy: Can shoot beams in different directions
Enemy3: Special enemy type with unique behaviors
Enemy5: Advanced enemy with more health and stronger attacks

Controls

Left/Right Arrow: Move left/right
Up Arrow: Jump
Down Arrow: Drop through platforms
X (tap): Fire beam attack
X (hold): Charge beam attack for more power
X (near snowball): Push or kick snowball

Project Structure

player.rs: Player character logic and interactions
enemy.rs: Enemy behaviors, types, and animations
snowball.rs: Snowball physics and transformations
drinks.rs: Power-up system and effects
lives.rs: Life management and respawn system
ui.rs: Game interface elements
main.rs: Core game loops and systems integration

Development
This game is built with the Bevy game engine and uses Entity Component System (ECS) architecture. The project demonstrates:

Sprite animation systems
Physics-based interactions
State management
Wave-based enemy spawning
Power-up systems
Player progression

Building and Running
Make sure you have Rust and Cargo installed, then:
cargo run --release
Graphics
The game uses pixel art sprites with animations for all game elements. Asset files should be placed in the project's asset directory.
