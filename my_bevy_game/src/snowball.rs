use bevy::prelude::*;
use crate::Velocity;
use crate::Gravity;
use crate::enemy; 
use crate::drinks;

use crate::player;
#[derive(Component)]
pub struct Snowball {
    pub state: SnowballState,
    pub snow_level: u8, 
    pub on_ground: bool,
    pub is_rolling: bool,
    pub animation_timer: Timer,
    pub animation_frame: usize,
    pub roll_timer: Timer,
    pub roll_frame: usize,
    pub kicked: bool,
    pub corner_visited: bool,
    pub visited_corners: Vec<u8>,
    pub melt_timer: Timer,
    pub is_melting: bool,
    pub original_enemy_type: Option<enemy::EnemyType>,
    pub melt_alpha: f32,               
    pub melt_flash_timer: Timer,       
    pub melt_color_shift: Color,  
    pub transform_to_enemy: bool,
    pub continuous_roll_timer: Timer,
}

#[derive(PartialEq)]
pub enum SnowballState {
    Idle,
    Pushed,
    Rolling,
    Kicked,
}

#[derive(Resource)]
pub struct SnowballSprites {
    pub snow_first: Handle<Image>,
    pub snow_second: Handle<Image>,
    pub snow_third: Handle<Image>,
    pub snow_roll: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub struct BaseCorners {
    pub positions: Vec<Vec2>,
}

pub fn setup_snowball_resource(
    mut commands: Commands, 
    asset_server: &Res<AssetServer>, 
    images: &mut ResMut<Assets<Image>>
) {
    let snow_first_handle = asset_server.load("snow_first.png");
    let snow_second_handle = asset_server.load("snow_second.png");
    let snow_third_handle = asset_server.load("snow_third.png");
    
    let snow_roll_first = asset_server.load("snow_roll_first.png");
    let snow_roll_second = asset_server.load("snow_roll_second.png");
    let snow_roll_third = asset_server.load("snow_roll_third.png");
    let snow_roll_fourth = asset_server.load("snow_roll_fourth.png");
    
    for handle in [
        &snow_first_handle, &snow_second_handle, &snow_third_handle,
        &snow_roll_first, &snow_roll_second, &snow_roll_third, &snow_roll_fourth,
    ].iter() {
        if let Some(image) = images.get_mut(*handle) {
            image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
        }
    }
    
    let snowball_sprites = SnowballSprites {
        snow_first: snow_first_handle.clone(),
        snow_second: snow_second_handle.clone(),
        snow_third: snow_third_handle.clone(),
        snow_roll: vec![
            snow_roll_first.clone(),
            snow_roll_second.clone(),
            snow_roll_third.clone(),
            snow_roll_fourth.clone(),
        ],
    };
    commands.insert_resource(snowball_sprites);
    
    // Setup the base corner positions - these mark the edges of the base platform
    let base_corners = BaseCorners {
        positions: vec![
            Vec2::new(-400.0, -250.0),
            Vec2::new(400.0, -250.0),  
        ],
    };
    commands.insert_resource(base_corners);
}


pub fn spawn_snowball(
    commands: &mut Commands,
    position: Vec2,
    sprites: &Res<SnowballSprites>,
    initial_snow_level: u8,
    enemy_type: Option<enemy::EnemyType>,
) {
    let initial_texture = match initial_snow_level {
        1 => sprites.snow_first.clone(),
        2 => sprites.snow_second.clone(),
        3 => sprites.snow_third.clone(),
        _ => sprites.snow_first.clone(), 
    };
    
    commands.spawn((
        SpriteBundle {
            texture: initial_texture,
            transform: Transform::from_xyz(position.x, position.y, 1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
        Velocity { value: Vec2::new(0.0, 0.0) },
        Gravity,
        Snowball {
            state: SnowballState::Idle,
            snow_level: initial_snow_level.clamp(1, 3),
            on_ground: true,
            is_rolling: false,
            animation_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            animation_frame: 0,
            roll_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            roll_frame: 0,
            kicked: false,
            corner_visited: false,
            visited_corners: Vec::new(),
            melt_timer: Timer::from_seconds(10.0, TimerMode::Once), 
            is_melting: false,
            original_enemy_type: enemy_type,
            transform_to_enemy: false,
            melt_alpha: 1.0,
            melt_flash_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            melt_color_shift: Color::WHITE,
            continuous_roll_timer: Timer::from_seconds(20.0, TimerMode::Once),
        },
    ));
}

pub fn handle_snowball_melting(
    time: Res<Time>,
    mut commands: Commands,
    mut snowball_query: Query<(Entity, &mut Snowball, &mut Transform, &mut Handle<Image>, &mut Sprite)>,
    snowball_sprites: Res<SnowballSprites>,
    enemy_sprites: Res<enemy::EnemySprites>,
    enemy2_sprites: Option<Res<enemy::Enemy2Sprites>>,
    enemy3_sprites: Option<Res<enemy::Enemy3Sprites>>,
    enemy5_sprites: Option<Res<enemy::Enemy5Sprites>>,
) {
    for (entity, mut snowball, mut transform, mut texture, mut sprite) in snowball_query.iter_mut() {
        if snowball.state == SnowballState::Kicked {
            snowball.melt_timer.reset();
            snowball.is_melting = false;
            snowball.melt_alpha = 1.0;
            sprite.color = Color::WHITE;
            continue;
        }
        
        snowball.melt_timer.tick(time.delta());
        
        let one_third_point = snowball.melt_timer.duration().as_secs_f32() / 3.0;
        if snowball.melt_timer.elapsed_secs() >= one_third_point && !snowball.is_melting {
            snowball.is_melting = true;
            println!("Snowball is starting to melt...");
            
            if snowball.snow_level > 1 {
                snowball.snow_level -= 1;
            }
        }
        
        if snowball.is_melting && !snowball.transform_to_enemy {
            snowball.melt_flash_timer.tick(time.delta());
            
            if snowball.melt_flash_timer.just_finished() {
                let melt_progress = snowball.melt_timer.elapsed_secs() / snowball.melt_timer.duration().as_secs_f32();
                
                if melt_progress > 0.7 {
                    // Flash between a reddish tint and normal
                    if sprite.color.r() > 1.1 {
                        sprite.color = Color::rgba(1.0, 0.8, 0.8, snowball.melt_alpha);
                    } else {
                        sprite.color = Color::rgba(1.5, 0.6, 0.6, snowball.melt_alpha);
                    }
                    
                    transform.rotation = Quat::from_rotation_z(rand::random::<f32>() * 0.02 - 0.01);
                } else {
                    // Gradually shift to a warmer color to indicate melting
                    sprite.color = Color::rgba(1.0 + melt_progress * 0.5, 
                                              1.0 - melt_progress * 0.4, 
                                              1.0 - melt_progress * 0.4, 
                                              snowball.melt_alpha);
                }
                
                // Gradually reduce alpha as melting progresses
                snowball.melt_alpha = 1.0 - melt_progress * 0.3;
                
                // Change texture to represent melting
                *texture = match snowball.snow_level {
                    1 => snowball_sprites.snow_first.clone(),
                    2 => snowball_sprites.snow_second.clone(),
                    3 => snowball_sprites.snow_third.clone(),
                    _ => snowball_sprites.snow_first.clone(),
                };
            }
            
            // Second threshold at two-thirds - drop another snow level
            let two_thirds_point = 2.0 * snowball.melt_timer.duration().as_secs_f32() / 3.0;
            if snowball.melt_timer.elapsed_secs() >= two_thirds_point && snowball.snow_level > 1 {
                snowball.snow_level = 1;
            }
        }
        
        // When timer is finished, transform back to enemy
        if snowball.melt_timer.finished() && !snowball.transform_to_enemy {
            snowball.transform_to_enemy = true;
            
            // Determine which enemy type to spawn
            let enemy_type = snowball.original_enemy_type.clone().unwrap_or(enemy::EnemyType::Basic);
            
            println!("Snowball melting complete, spawning enemy of type: {:?}", enemy_type);
            
            // Spawn the enemy resurrection effect - using the last death frame as the starting point
            let death_texture = match enemy_type {
                enemy::EnemyType::Basic => enemy_sprites.death[4].clone(), 
                enemy::EnemyType::FireBeam => {
                    if let Some(ref enemy2_sprites) = enemy2_sprites {
                        enemy2_sprites.death[4].clone()
                    } else {
                        enemy_sprites.death[4].clone() 
                    }
                },
                enemy::EnemyType::Enemy3 => {
                    if let Some(ref enemy3_sprites) = enemy3_sprites {
                        enemy3_sprites.death[4].clone()
                    } else {
                        enemy_sprites.death[4].clone() 
                    }
                },
                enemy::EnemyType::Enemy5 => {
                    if let Some(ref enemy5_sprites) = enemy5_sprites {
                        enemy5_sprites.death[4].clone()
                    } else {
                        enemy_sprites.death[4].clone()
                    }
                }
            };
            
            // Create a special entity that will play the enemy revival animation
            commands.spawn((
                SpriteBundle {
                    texture: death_texture,
                    transform: Transform::from_translation(transform.translation),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        ..default()
                    },
                    ..default()
                },
                RevivingEnemy {
                    animation_timer: Timer::from_seconds(0.05, TimerMode::Repeating), 
                    animation_frame: 0,
                    enemy_type: enemy_type.clone(),
                    max_frames: 5,
                }
            ));
            
            // Despawn the original snowball
            commands.entity(entity).despawn();
        }
    }
}
// Component for the reviving enemy animation
#[derive(Component)]
pub struct RevivingEnemy {
    pub animation_timer: Timer,
    pub animation_frame: usize,
    pub enemy_type: enemy::EnemyType,
    pub max_frames: usize,
}

// A system to handle the animation and spawning of the revived enemy
pub fn animate_reviving_enemies(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut RevivingEnemy, &mut Handle<Image>, &mut Transform)>,
    enemy_sprites: Res<enemy::EnemySprites>,
    enemy2_sprites: Option<Res<enemy::Enemy2Sprites>>,
    enemy3_sprites: Option<Res<enemy::Enemy3Sprites>>,
    enemy5_sprites: Option<Res<enemy::Enemy5Sprites>>,
) {
    for (entity, mut reviving, mut texture, mut transform) in query.iter_mut() {
        reviving.animation_timer.tick(time.delta());
        
        if reviving.animation_timer.just_finished() {
            if reviving.animation_frame < reviving.max_frames - 1 {
                reviving.animation_frame += 1;
                
                let frame_index = reviving.max_frames - 1 - reviving.animation_frame;
                
                match reviving.enemy_type {
                    enemy::EnemyType::Basic => {
                        if frame_index < enemy_sprites.death.len() {
                            *texture = enemy_sprites.death[frame_index].clone();
                        }
                    },
                    enemy::EnemyType::FireBeam => {
                        if let Some(ref enemy2_sprites) = enemy2_sprites {
                            if frame_index < enemy2_sprites.death.len() {
                                *texture = enemy2_sprites.death[frame_index].clone();
                            }
                        } else {
                            if frame_index < enemy_sprites.death.len() {
                                *texture = enemy_sprites.death[frame_index].clone();
                            }
                        }
                    },
                    enemy::EnemyType::Enemy3 => {
                        if let Some(ref enemy3_sprites) = enemy3_sprites {
                            if frame_index < enemy3_sprites.death.len() {
                                *texture = enemy3_sprites.death[frame_index].clone();
                            }
                        } else {
                            if frame_index < enemy_sprites.death.len() {
                                *texture = enemy_sprites.death[frame_index].clone();
                            }
                        }
                    },
                    enemy::EnemyType::Enemy5 => {
                        if let Some(ref enemy5_sprites) = enemy5_sprites {
                            if frame_index < enemy5_sprites.death.len() {
                                *texture = enemy5_sprites.death[frame_index].clone();
                            }
                        } else {
                            if frame_index < enemy_sprites.death.len() {
                                *texture = enemy_sprites.death[frame_index].clone();
                            }
                        }
                    }
                }
                
                let scale_factor = 1.0 + 0.1 * (reviving.animation_frame as f32 / reviving.max_frames as f32);
                transform.scale = Vec3::new(scale_factor, scale_factor, 1.0);
            } else {
                println!("Reviving animation complete, spawning enemy type: {:?}", reviving.enemy_type);
                
                // Spawn the appropriate enemy type at the current position
                match reviving.enemy_type {
                    enemy::EnemyType::Basic => {
                        commands.spawn((
                            SpriteBundle {
                                texture: enemy_sprites.idle.clone(),
                                transform: Transform::from_translation(transform.translation),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(50.0, 50.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            Velocity { value: Vec2::new(0.0, 0.0) },
                            Gravity,
                            enemy::Enemy { 
                                facing_right: true,
                                is_moving: false,
                                is_jumping: false,
                                on_ground: true,
                                is_dropping: false,
                                is_rolling: false,
                                movement_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                                decision_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                                animation_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                                animation_frame: 0,
                                roll_timer: Timer::from_seconds(0.8, TimerMode::Once),
                                roll_frame: 0,
                                death_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                                death_frame: 0,
                                is_dying: false,
                                state: enemy::EnemyState::Idle,
                                enemy_type: enemy::EnemyType::Basic,
                                is_attacking: false,
                                attack_direction: enemy::AttackDirection::Horizontal,
                                attack_timer: Timer::from_seconds(0.5, TimerMode::Once),
                                beam_timer: Timer::from_seconds(0.8, TimerMode::Once),
                                beam_active: false,
                                damage: 1,
                            },
                        ));
                    },
                    enemy::EnemyType::FireBeam => {
                        if let Some(ref enemy2_sprites) = enemy2_sprites {
                            commands.spawn((
                                SpriteBundle {
                                    texture: enemy2_sprites.idle.clone(),
                                    transform: Transform::from_translation(transform.translation),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(50.0, 50.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Velocity { value: Vec2::new(0.0, 0.0) },
                                Gravity,
                                enemy::Enemy { 
                                    facing_right: true,
                                    is_moving: false,
                                    is_jumping: false,
                                    on_ground: true,
                                    is_dropping: false,
                                    is_rolling: false,
                                    movement_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                                    decision_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                                    animation_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                                    animation_frame: 0,
                                    roll_timer: Timer::from_seconds(0.8, TimerMode::Once),
                                    roll_frame: 0,
                                    death_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                                    death_frame: 0,
                                    is_dying: false,
                                    state: enemy::EnemyState::Idle,
                                    enemy_type: enemy::EnemyType::FireBeam,
                                    is_attacking: false,
                                    attack_direction: enemy::AttackDirection::Horizontal,
                                    attack_timer: Timer::from_seconds(0.5, TimerMode::Once),
                                    beam_timer: Timer::from_seconds(0.8, TimerMode::Once),
                                    beam_active: false,
                                    damage : 3,
                                },
                            ));
                        }
                    },
                    enemy::EnemyType::Enemy3 => {
                        if let Some(ref enemy3_sprites) = enemy3_sprites {
                            commands.spawn((
                                SpriteBundle {
                                    texture: enemy3_sprites.idle.clone(),
                                    transform: Transform::from_translation(transform.translation),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(50.0, 50.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Velocity { value: Vec2::new(0.0, 0.0) },
                                Gravity,
                                enemy::Enemy { 
                                    facing_right: true,
                                    is_moving: false,
                                    is_jumping: false,
                                    on_ground: true,
                                    is_dropping: false,
                                    is_rolling: false,
                                    movement_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                                    decision_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                                    animation_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                                    animation_frame: 0,
                                    roll_timer: Timer::from_seconds(0.8, TimerMode::Once),
                                    roll_frame: 0,
                                    death_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                                    death_frame: 0,
                                    is_dying: false,
                                    state: enemy::EnemyState::Idle,
                                    enemy_type: enemy::EnemyType::Enemy3,
                                    is_attacking: false,
                                    attack_direction: enemy::AttackDirection::Horizontal,
                                    attack_timer: Timer::from_seconds(0.5, TimerMode::Once),
                                    beam_timer: Timer::from_seconds(0.8, TimerMode::Once),
                                    beam_active: false,
                                    damage : 2,
                                },
                            ));
                        }
                    },
                    enemy::EnemyType::Enemy5 => {
                        if let Some(ref enemy5_sprites) = enemy5_sprites {
                            commands.spawn((
                                SpriteBundle {
                                    texture: enemy5_sprites.idle.clone(),
                                    transform: Transform::from_translation(transform.translation),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(50.0, 50.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Velocity { value: Vec2::new(0.0, 0.0) },
                                Gravity,
                                enemy::Enemy { 
                                    facing_right: true,
                                    is_moving: false,
                                    is_jumping: false,
                                    on_ground: true,
                                    is_dropping: false,
                                    is_rolling: false,
                                    movement_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                                    decision_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                                    animation_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                                    animation_frame: 0,
                                    roll_timer: Timer::from_seconds(0.8, TimerMode::Once),
                                    roll_frame: 0,
                                    death_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                                    death_frame: 0,
                                    is_dying: false,
                                    state: enemy::EnemyState::Idle,
                                    enemy_type: enemy::EnemyType::Enemy5,
                                    is_attacking: false,
                                    attack_direction: enemy::AttackDirection::Horizontal,
                                    attack_timer: Timer::from_seconds(0.5, TimerMode::Once),
                                    beam_timer: Timer::from_seconds(0.8, TimerMode::Once),
                                    beam_active: false,
                                    damage : 4
                                },
                            ));
                        }
                    }
                }
                
                // Despawn the reviving entity
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn update_snowball(
    time: Res<Time>,
    mut commands: Commands,
    mut snowball_query: Query<(Entity, &mut Snowball, &mut Velocity, &mut Transform)>,
    base_corners: Option<Res<BaseCorners>>,
) {
    for (entity, mut snowball, mut velocity, mut transform) in snowball_query.iter_mut() {
        snowball.animation_timer.tick(time.delta());
        
        // Get the current position for corner checking
        let snowball_pos = transform.translation.truncate();
        let base_platform_y = -250.0; 
        
        let on_base_platform = snowball.on_ground && (snowball_pos.y - (base_platform_y + 25.0)).abs() < 30.0;
        
        // Track continuous rolling time
        let is_currently_rolling = (snowball.state == SnowballState::Rolling || snowball.state == SnowballState::Kicked) 
                                  && snowball.on_ground 
                                  && velocity.value.x.abs() > 20.0;
        
        if is_currently_rolling {
            snowball.continuous_roll_timer.tick(time.delta());
            
            if snowball.continuous_roll_timer.finished() {
                println!("Snowball rolled continuously for 20 seconds - despawning");
                commands.entity(entity).despawn();
                return; 
            }
        } else {
            snowball.continuous_roll_timer.reset();
        }
        
        // Check if the snowball is near any corner of the base platform
        if on_base_platform {
            if let Some(base_corners) = &base_corners {
                let mut approaching_corner = false;
                let mut closest_dist = f32::MAX;
                
                for corner_pos in &base_corners.positions {
                    let dist = (snowball_pos - *corner_pos).length();
                    closest_dist = closest_dist.min(dist);
                    
                    if dist < 150.0 && snowball.state == SnowballState::Kicked {
                        approaching_corner = true;
                    }
                    
                    if dist < 90.0 {
                        println!("Snowball reached a base platform corner at {:?} - despawning", corner_pos);
                        commands.entity(entity).despawn();
                        return; 
                    }
                }
                
                if approaching_corner && snowball.state == SnowballState::Kicked {
                    let slowdown_factor = (closest_dist / 150.0).clamp(0.3, 1.0);
                    
                    if snowball.on_ground {
                        let current_dir_x = if velocity.value.x > 0.0 { 1.0 } else { -1.0 };
                        let base_speed = 400.0 + (snowball.snow_level as f32 * 50.0);
                        let adjusted_speed = base_speed * slowdown_factor;
                        
                        velocity.value.x = current_dir_x * adjusted_speed;
                    }
                }
            }
        }
        
        // Handle kicked snowball physics
        if snowball.state == SnowballState::Kicked {
            // Always keep the snowball rolling quickly
            snowball.roll_timer.tick(time.delta());
            
            if snowball.roll_timer.just_finished() {
                snowball.roll_frame = (snowball.roll_frame + 1) % 4;
            }
            
            if snowball_pos.x <= -370.0 || snowball_pos.x >= 370.0 {
                
                if on_base_platform {
                    velocity.value.x = -velocity.value.x * 1.0; 
                    velocity.value.y = 100.0; 
                    if let Some(base_corners) = &base_corners {
                        for corner_pos in &base_corners.positions {
                            if (snowball_pos - *corner_pos).length() < 90.0 {
                                println!("Snowball reached a corner while bouncing - despawning");
                                commands.entity(entity).despawn();
                                return; 
                            }
                        }
                    }
                } 
                // If not on base platform but hit a wall, bounce in opposite direction
                else {
                    // Reverse horizontal direction with full bounce
                    velocity.value.x = -velocity.value.x * 1.0;
                    velocity.value.y = 80.0; 
                    
                    if snowball_pos.x <= -370.0 {
                        transform.translation.x = -369.0;
                    } else if snowball_pos.x >= 370.0 {
                        transform.translation.x = 369.0;
                    }
                }
            }
            
            // Keep the snowball rolling continuously with constant speed
            if snowball.on_ground {
                let current_dir_x = if velocity.value.x > 0.0 { 1.0 } else { -1.0 };
                
                if velocity.value.x.abs() > 150.0 {
                    let kick_speed = 400.0 + (snowball.snow_level as f32 * 50.0);
                    velocity.value.x = current_dir_x * kick_speed;
                }
            }
            
            if snowball.on_ground && (snowball_pos.y - base_platform_y).abs() < 30.0 && snowball_pos.y != base_platform_y {
                
                transform.translation.y = base_platform_y + 25.0; 
            }
            
            continue;
        }
        
        // Regular rolling snowball behavior for non-kicked snowballs
        if snowball.state == SnowballState::Rolling {
            snowball.roll_timer.tick(time.delta());
            
            if snowball.roll_timer.just_finished() {
                snowball.roll_frame = (snowball.roll_frame + 1) % 4;
            }
            
            if snowball_pos.x <= -370.0 || snowball_pos.x >= 370.0 {
                velocity.value.x = -velocity.value.x * 0.9; 
                
                if snowball_pos.x <= -370.0 {
                    transform.translation.x = -369.0;
                } else if snowball_pos.x >= 370.0 {
                    transform.translation.x = 369.0;
                }
            }
            
            if snowball.on_ground {
                // Slow down rolling snowballs more gradually
                velocity.value.x *= 0.995;
                
                if velocity.value.x.abs() < 20.0 {
                    snowball.state = SnowballState::Idle;
                    velocity.value.x = 0.0;
                    
                    // Reset the continuous roll timer when the snowball stops
                    snowball.continuous_roll_timer.reset();
                }
            }
        }
    }
}
pub fn animate_snowball(
    mut query: Query<(&mut Handle<Image>, &Snowball)>,
    snowball_sprites: Res<SnowballSprites>,
) {
    for (mut texture, snowball) in query.iter_mut() {
        if snowball.state == SnowballState::Rolling || snowball.state == SnowballState::Kicked {
            let roll_index = snowball.roll_frame % snowball_sprites.snow_roll.len();
            *texture = snowball_sprites.snow_roll[roll_index].clone();
        } else {
            *texture = match snowball.snow_level {
                1 => snowball_sprites.snow_first.clone(),
                2 => snowball_sprites.snow_second.clone(),
                3 => snowball_sprites.snow_third.clone(),
                _ => snowball_sprites.snow_first.clone(), 
            };
        }
    }
}

// Component for tracking enemy death animation
#[derive(Component)]
pub struct DyingEnemy {
    pub animation_timer: Timer,
    pub animation_frame: usize,
    pub original_position: Vec3,
    pub direction: f32, 
    pub enemy_type: enemy::EnemyType,
}

// Modified function to spawn drinks when a snowball hits an enemy
pub fn handle_snowball_collision(
    mut commands: Commands,
    mut snowball_query: Query<(Entity, &Transform, &mut Snowball, &mut Velocity)>,
    enemy_query: Query<(Entity, &Transform, &enemy::Enemy), Without<DyingEnemy>>,
    asset_server: Res<AssetServer>,
    drink_sprites: Option<Res<crate::drinks::DrinkSprites>>,
    enemy_sprites: Res<enemy::EnemySprites>,
    enemy2_sprites: Option<Res<enemy::Enemy2Sprites>>,
    enemy3_sprites: Option<Res<enemy::Enemy3Sprites>>,
    enemy5_sprites: Option<Res<enemy::Enemy5Sprites>>,
) {
    // First, handle collisions between snowballs and enemies for kicked snowballs
    let mut kicked_snowballs = Vec::new();
    for (entity, transform, snowball, _) in snowball_query.iter() {
        if snowball.state == SnowballState::Kicked {
            kicked_snowballs.push((entity, transform.translation, snowball.snow_level));
        }
    }
    
    // Check for collisions between kicked snowballs and enemies
    let mut enemy_collisions = Vec::new();
    for (snowball_entity, snowball_pos, snow_level) in kicked_snowballs.iter() {
        for (enemy_entity, enemy_transform, enemy) in enemy_query.iter() {
            let distance = (snowball_pos.truncate() - enemy_transform.translation.truncate()).length();
            
            // If a kicked snowball hits an enemy
            if distance < 50.0 {
                enemy_collisions.push((*snowball_entity, enemy_entity, *snowball_pos, enemy_transform.translation, *snow_level, enemy.enemy_type.clone()));
            }
        }
    }
    
    // Process enemy-snowball collisions
    for (snowball_entity, enemy_entity, snowball_pos, enemy_pos, snow_level, enemy_type) in enemy_collisions {
        let death_texture = match enemy_type {
            enemy::EnemyType::Basic => enemy_sprites.death[0].clone(),
            enemy::EnemyType::FireBeam => {
                if let Some(ref enemy2_sprites) = enemy2_sprites {
                    enemy2_sprites.death[0].clone()
                } else {
                    enemy_sprites.death[0].clone() 
                }
            },
            enemy::EnemyType::Enemy3 => {
                if let Some(ref enemy3_sprites) = enemy3_sprites {
                    enemy3_sprites.death[0].clone()
                } else {
                    enemy_sprites.death[0].clone()
                }
            },
            enemy::EnemyType::Enemy5 => {
                if let Some(ref enemy5_sprites) = enemy5_sprites {
                    enemy5_sprites.death[0].clone()
                } else {
                    enemy_sprites.death[0].clone() 
                }
            }
        };
        
        let death_animation_entity = commands.spawn((
            SpriteBundle {
                texture: death_texture,
                transform: Transform::from_translation(enemy_pos),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                ..default()
            },
            Velocity {
                value: Vec2::new(
                    if snowball_pos.x > enemy_pos.x { -150.0 } else { 150.0 },
                    250.0
                )
            },
            Gravity,
            DyingEnemy {
                animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                animation_frame: 0,
                original_position: enemy_pos,
                direction: if snowball_pos.x > enemy_pos.x { -1.0 } else { 1.0 },
                enemy_type, 
            }
        )).id();
        
        // Spawn a drink at the enemy's position
        if let Some(drink_sprites) = &drink_sprites {
            crate::drinks::spawn_drink(
                &mut commands,
                enemy_pos.truncate(),
                &drink_sprites,
            );
        }
        
        // Despawn the original enemy
        commands.entity(enemy_entity).despawn();
        
        if snow_level <= 1 {
            commands.entity(snowball_entity).despawn();
        } else {
            if let Ok((_, _, mut snowball, _)) = snowball_query.get_mut(snowball_entity) {
                snowball.snow_level -= 1;
                snowball.melt_timer.reset(); 
            }
        }
    }
}

pub fn animate_dying_enemies(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DyingEnemy, &mut Handle<Image>, &Transform)>,
    enemy_sprites: Res<enemy::EnemySprites>,
    enemy2_sprites: Option<Res<enemy::Enemy2Sprites>>,
    enemy3_sprites: Option<Res<enemy::Enemy3Sprites>>,
    enemy5_sprites: Option<Res<enemy::Enemy5Sprites>>,
) {
    for (entity, mut dying_enemy, mut texture, transform) in query.iter_mut() {
        dying_enemy.animation_timer.tick(time.delta());
        
        if dying_enemy.animation_timer.just_finished() {
            dying_enemy.animation_frame += 1;
            
            // Update the sprite based on the current animation frame and enemy type
            match dying_enemy.enemy_type {
                enemy::EnemyType::Basic => {
                    let frame_index = dying_enemy.animation_frame.min(enemy_sprites.death.len() - 1);
                    *texture = enemy_sprites.death[frame_index].clone();
                },
                enemy::EnemyType::FireBeam => {
                    if let Some(ref enemy2_sprites) = enemy2_sprites {
                        let frame_index = dying_enemy.animation_frame.min(enemy2_sprites.death.len() - 1);
                        *texture = enemy2_sprites.death[frame_index].clone();
                    } else {
                        let frame_index = dying_enemy.animation_frame.min(enemy_sprites.death.len() - 1);
                        *texture = enemy_sprites.death[frame_index].clone();
                    }
                },
                enemy::EnemyType::Enemy3 => {
                    if let Some(ref enemy3_sprites) = enemy3_sprites {
                        let frame_index = dying_enemy.animation_frame.min(enemy3_sprites.death.len() - 1);
                        *texture = enemy3_sprites.death[frame_index].clone();
                    } else {
                        let frame_index = dying_enemy.animation_frame.min(enemy_sprites.death.len() - 1);
                        *texture = enemy_sprites.death[frame_index].clone();
                    }
                },
                enemy::EnemyType::Enemy5 => {
                    if let Some(ref enemy5_sprites) = enemy5_sprites {
                        let frame_index = dying_enemy.animation_frame.min(enemy5_sprites.death.len() - 1);
                        *texture = enemy5_sprites.death[frame_index].clone();
                    } else {
                        let frame_index = dying_enemy.animation_frame.min(enemy_sprites.death.len() - 1);
                        *texture = enemy_sprites.death[frame_index].clone();
                    }
                }
            }
            
            // When the animation is complete, despawn the entity
            let max_frames = match dying_enemy.enemy_type {
                enemy::EnemyType::Basic => enemy_sprites.death.len(),
                enemy::EnemyType::FireBeam => {
                    if let Some(ref enemy2_sprites) = enemy2_sprites {
                        enemy2_sprites.death.len()
                    } else {
                        enemy_sprites.death.len()
                    }
                },
                enemy::EnemyType::Enemy3 => {
                    if let Some(ref enemy3_sprites) = enemy3_sprites {
                        enemy3_sprites.death.len()
                    } else {
                        enemy_sprites.death.len()
                    }
                },
                enemy::EnemyType::Enemy5 => {
                    if let Some(ref enemy5_sprites) = enemy5_sprites {
                        enemy5_sprites.death.len()
                    } else {
                        enemy_sprites.death.len()
                    }
                }
            };
            
            if dying_enemy.animation_frame >= max_frames {
                commands.entity(entity).despawn();
            }
        }
        
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn();
        }
    }
}