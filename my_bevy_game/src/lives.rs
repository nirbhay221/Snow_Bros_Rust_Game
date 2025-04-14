use bevy::prelude::*;
use crate::player::{Player, GameOverText};
use crate::enemy;

#[derive(Resource)]
pub struct LivesStorage {
    pub current_lives: u8,
    pub max_lives: u8,
}

impl Default for LivesStorage {
    fn default() -> Self {
        Self {
            current_lives: 3,
            max_lives: 3,
        }
    }
}

// Startup system to initialize lives storage
pub fn setup_lives_storage(mut commands: Commands) {
    commands.insert_resource(LivesStorage::default());
}

pub fn handle_player_death(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Player, &Transform, &mut Handle<Image>)>,
    game_over_query: Query<Entity, With<GameOverText>>,
    asset_server: Res<AssetServer>,
    mut death_timer: Local<Option<Timer>>,
    mut respawn_timer: Local<Option<Timer>>,
    mut animation_frame: Local<usize>,
    mut wave_system: ResMut<enemy::WaveSystem>,
    mut lives_storage: ResMut<LivesStorage>,
) {
    if death_timer.is_none() {
        *death_timer = Some(Timer::from_seconds(0.2, TimerMode::Repeating));
    }
    
    if respawn_timer.is_none() {
        *respawn_timer = Some(Timer::from_seconds(2.0, TimerMode::Once));
    }
    
    let player_exists = !player_query.is_empty();
    
    if player_exists {
        for (entity, mut player, _transform, mut texture) in player_query.iter_mut() {
            if player.is_dead {
                let timer = death_timer.as_mut().unwrap();
                timer.tick(time.delta());
                
                if timer.just_finished() {
                    *animation_frame += 1;
                    
                    match *animation_frame {
                        1 => {
                            *texture = asset_server.load("player_death_first.png");
                        },
                        2 => {
                            *texture = asset_server.load("player_death_second.png");
                        },
                        3 => {
                            *texture = asset_server.load("player_death_third.png");
                        },
                        4 => {
                            // Decrease lives in both Player and LivesStorage
                            println!("Player died! Lives before: {}", lives_storage.current_lives);
                            lives_storage.current_lives = lives_storage.current_lives.saturating_sub(1);
                            player.lives = lives_storage.current_lives;
                            println!("Lives after: {}", lives_storage.current_lives);
                            
                            // Despawn player
                            commands.entity(entity).despawn();
                            
                            // Set respawn timer based on lives
                            let respawn_time = if lives_storage.current_lives > 0 { 2.0 } else { 4.0 };
                            *respawn_timer = Some(Timer::from_seconds(respawn_time, TimerMode::Once));
                            *animation_frame = 0;
                            
                            // If game over, spawn game over text
                            if lives_storage.current_lives == 0 {
                                commands.spawn((
                                    Text2dBundle {
                                        text: Text::from_section(
                                            "GAME OVER",
                                            TextStyle {
                                                font_size: 64.0,
                                                color: Color::RED,
                                                ..default()
                                            },
                                        ),
                                        transform: Transform::from_xyz(0.0, 0.0, 10.0),
                                        ..default()
                                    },
                                    GameOverText,
                                ));
                                
                                // Reset wave when player is completely out of lives
                                wave_system.current_wave = 0;
                                wave_system.spawn_timer.reset();
                                wave_system.spawning = false;
                                wave_system.wave_break_timer = Timer::from_seconds(5.0, TimerMode::Once);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    } else {
        if let Some(timer) = respawn_timer.as_mut() {
            timer.tick(time.delta());
            
            if timer.finished() {
                let mut game_over = false;
                
                for entity in game_over_query.iter() {
                    commands.entity(entity).despawn();
                    game_over = true;
                }
                
                // Reset lives if game over
                if game_over {
                    lives_storage.current_lives = lives_storage.max_lives;
                }
                
                println!("Respawning player with {} lives", lives_storage.current_lives);
                
                if lives_storage.current_lives > 0 {
                    commands.spawn((
                        SpriteBundle {
                            texture: asset_server.load("player_appear_first.png"),
                            transform: Transform::from_xyz(0.0, 100.0, 1.0),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(50.0, 50.0)),
                                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                                ..default()
                            },
                            ..default()
                        },
                        crate::Velocity { value: Vec2::new(0.0, 0.0) },
                        crate::Gravity,
                        Player { 
                            facing_right: true,
                            is_moving: false,
                            is_jumping: false,
                            on_ground: true,
                            is_dropping: false,
                            is_attacking: false,
                            attack_timer: Timer::from_seconds(0.3, TimerMode::Once),
                            attack_frame: 0,
                            drop_timer: Timer::from_seconds(0.2, TimerMode::Once),
                            animation_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                            animation_frame: 0,
                            x_button_held: false,
                            x_button_hold_time: 0.0,
                            max_hold_time: 1.5,
                            pushing_snowball: false,
                            kicking_snowball: false,
                            push_timer: Timer::from_seconds(0.4, TimerMode::Once),
                            push_frame: 0,
                            snowball_collision: false,
                            collision_direction: None,
                            health: 10,
                            max_health: 10,
                            invincibility_timer: Timer::from_seconds(6.0, TimerMode::Once), // 6 seconds of invincibility
                            is_dead: false,
                            lives: lives_storage.current_lives,
                            max_lives: lives_storage.max_lives,
                        },
                    ));
                    
                    *timer = Timer::from_seconds(0.2, TimerMode::Repeating);
                    *animation_frame = 1; 
                }
            }
        }
    }
    
    if player_exists {
        let mut found_alive_player = false;
        
        for (_entity, player, _, mut texture) in player_query.iter_mut() {
            if !player.is_dead {
                found_alive_player = true;
                
                if let Some(timer) = respawn_timer.as_mut() {
                    if !timer.finished() {
                        timer.tick(time.delta());
                        
                        if timer.just_finished() {
                            match *animation_frame {
                                1 => {
                                    *texture = asset_server.load("player_appear_first.png");
                                    *animation_frame += 1;
                                },
                                2 => {
                                    *texture = asset_server.load("player_appear_second.png");
                                    *animation_frame += 1;
                                },
                                3 => {
                                    *texture = asset_server.load("player_appear_third.png");
                                    *animation_frame += 1;
                                },
                                4 => {
                                    *texture = asset_server.load("player_appear_fourth.png");
                                    *animation_frame += 1;
                                },
                                5 => {
                                    *texture = asset_server.load("player_appear_fifth.png");
                                    *animation_frame = 0;
                                    *respawn_timer = None;
                                },
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        
        if found_alive_player {
            for entity in game_over_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}