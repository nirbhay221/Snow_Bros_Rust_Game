use bevy::{prelude::*, transform};
use crate::snowball::{Snowball, SnowballState};
use crate::enemy;
use crate::drinks;
use crate::enemy::Enemy;
use crate::enemy::EnemyState;

#[derive(Component)]
pub struct Player {
    pub facing_right: bool,
    pub is_moving: bool,
    pub is_jumping: bool,
    pub on_ground: bool,
    pub is_dropping: bool,
    pub is_attacking: bool,
    pub attack_timer: Timer,
    pub attack_frame: usize,
    pub drop_timer: Timer,
    pub animation_timer: Timer,
    pub animation_frame: usize,
    pub x_button_held: bool,
    pub x_button_hold_time: f32,
    pub max_hold_time: f32,
    pub pushing_snowball: bool,
    pub kicking_snowball: bool,
    pub push_timer: Timer,
    pub push_frame: usize,
    pub snowball_collision: bool,
    pub collision_direction: Option<Vec2>, 
    pub health: u8,
    pub max_health: u8,
    pub invincibility_timer: Timer, 
    pub is_dead: bool,
    pub lives: u8,
    pub max_lives: u8,
}

#[derive(Component)]
pub struct PowerIndicator;

#[derive(Component)]
pub struct HealthBar {
    pub is_background: bool,
}

#[derive(Component)]
pub struct Beam {
    pub lifetime: Timer,
    pub direction: f32,
    pub speed: f32,
    pub phase: BeamPhase,
    pub phase_timer: Timer,
    pub power: f32,
    pub transition_timer: Timer,
}

#[derive(PartialEq)]
pub enum BeamPhase {
    Straight,
    Down,
}

#[derive(Resource)]
pub struct PlayerSprites {
    pub idle: Handle<Image>,
    pub left: Vec<Handle<Image>>,
    pub jump: Vec<Handle<Image>>,
    pub attack: Vec<Handle<Image>>,
    pub beam_first: Handle<Image>,
    pub beam_second: Handle<Image>,
    pub push_snow: Vec<Handle<Image>>,
    pub kick_snowball: Handle<Image>,
}
pub fn setup_health_bar(mut commands: Commands) {
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.8, 0.0),
                custom_size: Some(Vec2::new(50.0, 5.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 35.0, 1.6),
            ..default()
        },
        HealthBar { is_background: false },
    ));
}
pub fn update_health_bar(
    mut param_set: ParamSet<(
        Query<(&Player, &Transform)>,
        Query<(&mut Transform, &mut Sprite, &HealthBar)>
    )>,
) {
    let mut player_data: Option<(u8, u8, Vec3)> = None;
    for (player, transform) in param_set.p0().iter() {
        player_data = Some((player.health, player.max_health, transform.translation));
        break;
    }
    
    // update the health bars if player exists
    if let Some((health, max_health, player_pos)) = player_data {
        let health_ratio = health as f32 / max_health as f32;
        
        for (mut transform, mut sprite, health_bar) in param_set.p1().iter_mut() {
            // Position the health bar above the player
            transform.translation.x = player_pos.x;
            transform.translation.y = player_pos.y + 35.0;
            
            // Update the green foreground bar width based on health
            if !health_bar.is_background {
                let base_width = 50.0;
                sprite.anchor = bevy::sprite::Anchor::CenterLeft;
                sprite.custom_size = Some(Vec2::new(base_width * health_ratio, 5.0));
            }
        }
    }
}
pub fn setup_player(
    mut commands: Commands, 
    asset_server: &Res<AssetServer>, 
    images: &mut ResMut<Assets<Image>>
) {
    let idle_handle = asset_server.load("tom.png");
    let left_handle1 = asset_server.load("tom_left.png");
    let left_handle2 = asset_server.load("tom_left_middle.png");
    let left_handle3 = asset_server.load("tom_left_final.png");
    let jump_handle1 = asset_server.load("jump_first.png");
    let jump_handle2 = asset_server.load("jump_second.png");
    let jump_handle3 = asset_server.load("jump_third.png");
    let jump_handle4 = asset_server.load("jump_fourth.png");
    
    let attack_handle1 = asset_server.load("fight_first.png");
    let attack_handle2 = asset_server.load("fight_second.png");
    
    let beam_first_handle = asset_server.load("beam_second.png");
    let beam_second_handle = asset_server.load("beam_first.png");
    
    let push_snow_one = asset_server.load("push_snow_one.png");
    let push_snow_second = asset_server.load("push_snow_second.png");
    let kick_snowball = asset_server.load("kick_snowball.png");
    
    for handle in [
        &idle_handle, &left_handle1, &left_handle2, &left_handle3,
        &jump_handle1, &jump_handle2, &jump_handle3, &jump_handle4,
        &attack_handle1, &attack_handle2, &beam_first_handle, &beam_second_handle,
        &push_snow_one, &push_snow_second, &kick_snowball,
    ].iter() {
        if let Some(image) = images.get_mut(*handle) {
            image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
        }
    }
    
    let player_sprites = PlayerSprites {
        idle: idle_handle.clone(),
        left: vec![
            left_handle1.clone(),
            left_handle2.clone(),
            left_handle3.clone(),
        ],
        jump: vec![
            jump_handle1.clone(),
            jump_handle2.clone(),
            jump_handle3.clone(),
            jump_handle4.clone(),
        ],
        attack: vec![
            attack_handle1.clone(),
            attack_handle2.clone(),
        ],
        beam_first: beam_first_handle.clone(),
        beam_second: beam_second_handle.clone(),
        push_snow: vec![
            push_snow_one.clone(),
            push_snow_second.clone(),
        ],
        kick_snowball: kick_snowball.clone(),
    };
    
    commands.insert_resource(player_sprites);
    
    commands.spawn((
        SpriteBundle {
            texture: idle_handle,
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
            invincibility_timer: Timer::from_seconds(1.0, TimerMode::Once),
            is_dead: false,
            lives: 3,
            max_lives: 3,
        },
    ));
}


pub fn spawn_beam(
    commands: &mut Commands, 
    position: Vec2, 
    facing_right: bool, 
    power: f32, 
    player_sprites: &Res<PlayerSprites>
) {
    let direction = if facing_right { -1.0 } else { 1.0 };
    let offset = Vec2::new(direction * 30.0, 5.0);
    
    let base_speed = 180.0;
    let speed = base_speed * (0.5 + power * 1.5);
    
    let base_lifetime = 1.2;
    let lifetime = base_lifetime * (0.5 + power * 0.8);
    
    let width = 40.0 + (power * 20.0);
    let height = 20.0 + (power * 10.0);
    
    commands.spawn((
        SpriteBundle {
            texture: player_sprites.beam_second.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(position.x + offset.x, position.y + offset.y, 2.0),
                rotation: if facing_right { 
                    Quat::from_rotation_z(0.0) 
                } else { 
                    Quat::from_rotation_z(std::f32::consts::PI) 
                },
                ..default()
            },
            ..default()
        },
        Beam {
            lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
            direction,
            speed,
            phase: BeamPhase::Straight,
            phase_timer: Timer::from_seconds(0.5, TimerMode::Once),
            power,
            transition_timer: Timer::from_seconds(0.3, TimerMode::Once),
        },
    ));
}

pub fn draw_power_indicator(
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<(&Player, &Transform)>,
        Query<(Entity, &mut Transform, &mut Sprite), With<PowerIndicator>>
    )>,
) {
    let mut player_data = Vec::new();
    for (player, transform) in param_set.p0().iter() {
        player_data.push((player.x_button_held, player.x_button_hold_time, player.max_hold_time, transform.translation));
    }
    
    for (held, hold_time, max_hold_time, player_pos) in player_data.iter() {
        if *held {
            let power = (hold_time / max_hold_time).clamp(0.0, 1.0);
            
            let r = power;
            let g = 1.0 - power * 0.8;
            let b = 0.2;
            
            let mut found = false;
            for (_entity, mut ind_transform, mut sprite) in param_set.p1().iter_mut() {
                ind_transform.translation = Vec3::new(player_pos.x, player_pos.y + 35.0, 2.0);
                sprite.custom_size = Some(Vec2::new(40.0 * power, 5.0));
                sprite.color = Color::rgb(r, g, b);
                found = true;
                break;
            }
            
            if !found {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(r, g, b),
                            custom_size: Some(Vec2::new(40.0 * power, 5.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            player_pos.x,
                            player_pos.y + 35.0,
                            2.0
                        ),
                        ..default()
                    },
                    PowerIndicator,
                ));
            }
        } else {
            for (entity, _, _) in param_set.p1().iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}
pub fn handle_player_enemy_collision(
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &Transform, &mut Sprite)>,
    enemy_query: Query<(&Enemy, &Transform)>,
) {
    if let Ok((mut player, player_transform, mut player_sprite)) = player_query.get_single_mut() {
        // Skip collision checks if player is dead
        if player.is_dead {
            return;
        }
        
        player.invincibility_timer.tick(time.delta());
        
        if !player.invincibility_timer.finished() {
            // Make player flash when invincible
            let flash_speed = 10.0;
            let alpha = (time.elapsed_seconds() * flash_speed).sin() * 0.5 + 0.5;
            player_sprite.color.set_a(alpha);
            return;
        }
        
        // Reset alpha if not invincible
        player_sprite.color.set_a(1.0);
        
        // Check for collisions with enemies
        let player_size = Vec2::new(40.0, 50.0);
        let player_left = player_transform.translation.x - player_size.x / 2.0;
        let player_right = player_transform.translation.x + player_size.x / 2.0;
        let player_top = player_transform.translation.y + player_size.y / 2.0;
        let player_bottom = player_transform.translation.y - player_size.y / 2.0;
        
        for (enemy, enemy_transform) in enemy_query.iter() {
            // Skip dying enemies
            if enemy.state == EnemyState::Dying || enemy.is_dying {
                continue;
            }
            
            let enemy_size = Vec2::new(50.0, 50.0);
            let enemy_left = enemy_transform.translation.x - enemy_size.x / 2.0;
            let enemy_right = enemy_transform.translation.x + enemy_size.x / 2.0;
            let enemy_top = enemy_transform.translation.y + enemy_size.y / 2.0;
            let enemy_bottom = enemy_transform.translation.y - enemy_size.y / 2.0;
            
            if player_right > enemy_left && player_left < enemy_right &&
               player_top > enemy_bottom && player_bottom < enemy_top {
                // Collision detected, reduce health by enemy damage amount
                player.health = player.health.saturating_sub(enemy.damage);
                
                // Start invincibility timer
                player.invincibility_timer.reset();
                
                // Check if player died
                if player.health == 0 {
                    player.is_dead = true;
                }
                
                break; 
            }
        }
    }
}
pub fn handle_attack(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Player)>,
    player_sprites: Res<PlayerSprites>,
    snowball_query: Query<Entity, With<Snowball>>,
) {
    if let Ok((transform, mut player)) = query.get_single_mut() {
        if player.pushing_snowball || player.kicking_snowball {
            player.push_timer.tick(time.delta());
            
            if player.push_timer.finished() {
                player.pushing_snowball = false;
                player.kicking_snowball = false;
                player.push_timer.reset();
                player.push_frame = 0;
            }
            
            if player.pushing_snowball {
                let elapsed_fraction = player.push_timer.elapsed_secs() / player.push_timer.duration().as_secs_f32();
                if elapsed_fraction > 0.5 && player.push_frame == 0 {
                    player.push_frame = 1;
                }
            }
            
            return;
        }
        
        if player.is_attacking {
            player.attack_timer.tick(time.delta());
            
            let elapsed_fraction = player.attack_timer.elapsed_secs() / player.attack_timer.duration().as_secs_f32();
            
            if elapsed_fraction > 0.5 && player.attack_frame == 0 {
                player.attack_frame = 1;
            }
            
            if player.attack_timer.finished() {
                player.is_attacking = false;
                player.attack_timer.reset();
                player.attack_frame = 0;
            }
            
            return;
        }
        
        if keyboard.just_pressed(KeyCode::KeyX) && !snowball_query.is_empty() {
            return;
        }
        
        if keyboard.pressed(KeyCode::KeyX) {
            if !player.x_button_held {
                player.x_button_held = true;
                player.x_button_hold_time = 0.0;
            } else {
                player.x_button_hold_time += time.delta_seconds();
                
                if player.x_button_hold_time > player.max_hold_time {
                    player.x_button_hold_time = player.max_hold_time;
                }
            }
        } else if player.x_button_held { 
            player.x_button_held = false;
            
            let power = (player.x_button_hold_time / player.max_hold_time).clamp(0.0, 1.0);
            
            player.is_attacking = true;
            player.attack_frame = 0;
            player.attack_timer.reset();
            
            spawn_beam(&mut commands, transform.translation.truncate(), player.facing_right, power, &player_sprites);
            
            player.x_button_hold_time = 0.0;
        }
    }
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut crate::Velocity, &mut Player, &mut Transform)>,
) {
    if let Ok((mut velocity, mut player, mut transform)) = query.get_single_mut() {
        let mut direction = 0.0;
        player.is_moving = false;
        
        if player.pushing_snowball || player.kicking_snowball {
            return;
        }
        
        if player.snowball_collision {
            if let Some(collision_dir) = player.collision_direction {
                if (collision_dir.x < 0.0 && keyboard.pressed(KeyCode::KeyD)) ||
                   (collision_dir.x > 0.0 && keyboard.pressed(KeyCode::KeyA)) {
                    velocity.value.x = 0.0;
                } else {
                    player.snowball_collision = false;
                }
            } else {
                velocity.value.x = 0.0;
            }
        }
        
        if player.is_dropping {
            player.drop_timer.tick(time.delta());
            if player.drop_timer.finished() {
                player.is_dropping = false;
                player.drop_timer.reset();
            }
        }
        
        // Base platform y position
        let base_platform_y = -250.0;
        
        // Check if player is currently on or below the base platform
        let on_or_below_base = transform.translation.y - 25.0 <= base_platform_y + 5.0;
        
        // Only allow dropping if NOT on or below the base platform
        if (keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown)) && 
           player.on_ground && !on_or_below_base {
            player.is_dropping = true;
            player.on_ground = false;
            velocity.value.y = -5.0;
        }
        
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
            player.facing_right = true;
            player.is_moving = true;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
            player.facing_right = false;
            player.is_moving = true;
        }
        
        if !player.snowball_collision {
            velocity.value.x = direction * 200.0;
        }
        
        if (keyboard.just_pressed(KeyCode::Space) || 
            keyboard.just_pressed(KeyCode::KeyW) || 
            keyboard.just_pressed(KeyCode::ArrowUp)) && player.on_ground {
            velocity.value.y = 250.0;
            player.is_jumping = true;
            player.on_ground = false;
            player.animation_frame = 0;
        }
        
        let screen_bounds = Vec2::new(380.0, 280.0);
        
        if transform.translation.x < -screen_bounds.x {
            transform.translation.x = -screen_bounds.x;
            velocity.value.x = 0.0;
        } else if transform.translation.x > screen_bounds.x {
            transform.translation.x = screen_bounds.x;
            velocity.value.x = 0.0;
        }
        
        if transform.translation.y < -screen_bounds.y || transform.translation.y - 25.0 < base_platform_y {
            let respawn_x = if transform.translation.x < 0.0 { -200.0 } else { 200.0 };
            
            // Respawn above the base platform
            transform.translation = Vec3::new(respawn_x, 100.0, 1.0);
            velocity.value = Vec2::ZERO;
            player.on_ground = false;
            player.is_dropping = false;
            
            // Give temporary invincibility to prevent immediate collisions
            player.invincibility_timer.reset();
        } else if transform.translation.y > screen_bounds.y {
            transform.translation.y = screen_bounds.y;
            velocity.value.y = 0.0;
        }
    }
}


pub fn update_beams(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Beam, &mut Transform, &mut Handle<Image>)>,
    player_sprites: Res<PlayerSprites>,
) {
    for (entity, mut beam, mut transform, mut texture) in query.iter_mut() {
        beam.lifetime.tick(time.delta());
        beam.phase_timer.tick(time.delta());
        beam.transition_timer.tick(time.delta());
        
        if beam.transition_timer.finished() && *texture == player_sprites.beam_second {
            *texture = player_sprites.beam_first.clone();
        }
        
        if beam.phase == BeamPhase::Straight && beam.phase_timer.finished() {
            beam.phase = BeamPhase::Down;
        }
        
        match beam.phase {
            BeamPhase::Straight => {
                transform.translation.x += beam.direction * beam.speed * time.delta_seconds();
                
                if beam.power > 0.6 {
                    let phase_progress = beam.phase_timer.elapsed_secs() / 
                                        beam.phase_timer.duration().as_secs_f32();
                    let upward_force = 50.0 * beam.power * (1.0 - phase_progress);
                    transform.translation.y += upward_force * time.delta_seconds();
                }
            },
            BeamPhase::Down => {
                transform.translation.x += beam.direction * (beam.speed * 0.5) * time.delta_seconds();
                transform.translation.y -= (200.0 + 100.0 * beam.power) * time.delta_seconds();
            }
        }
        
        let wave_intensity = 0.1;
        let wave_frequency = 8.0;
        transform.rotation = Quat::from_rotation_z(
            transform.rotation.to_euler(EulerRot::XYZ).2 + 
            wave_intensity * (beam.lifetime.elapsed_secs() * wave_frequency).sin() * time.delta_seconds()
        );
        
        if beam.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Player, Option<&drinks::PowerUpEffect>)>,
) {
    if let Ok((mut player, power_up)) = query.get_single_mut() {
        if player.is_attacking || player.pushing_snowball || player.kicking_snowball {
            return;
        }
        
        // Check if the player has a speed boost effect
        let animation_speed_multiplier = if let Some(power_up) = power_up {
            if power_up.effect_type == drinks::PowerUpType::SpeedBoost && power_up.active {
                // Animate faster when speed boost is active
                2.0
            } else {
                1.0
            }
        } else {
            1.0
        };
        
        player.animation_timer.tick(time.delta().mul_f32(animation_speed_multiplier));
        
        if player.is_jumping {
            if player.animation_timer.just_finished() {
                player.animation_frame = (player.animation_frame + 1) % 4;
                
                if player.animation_frame == 0 && player.on_ground {
                    player.is_jumping = false;
                }
            }
        } else if player.is_moving {
            if player.animation_timer.just_finished() {
                player.animation_frame = (player.animation_frame + 1) % 3;
            }
        } else {
            // For idle animation during speed boost, cycle between frames
            if power_up.is_some() && power_up.unwrap().effect_type == drinks::PowerUpType::SpeedBoost {
                if player.animation_timer.just_finished() {
                    player.animation_frame = (player.animation_frame + 1) % 2;
                }
            } else {
                player.animation_frame = 0;
            }
        }
    }
}

pub fn update_sprite(
    player_sprites: Res<PlayerSprites>,
    red_power_sprites: Option<Res<drinks::RedPowerSprites>>,
    mut query: Query<(&Player, &mut Handle<Image>, &mut Sprite, Option<&drinks::PowerUpEffect>)>,
) {
    if let Ok((player, mut texture, mut sprite, power_up)) = query.get_single_mut() {
        // Check if the player has a speed boost effect
        if let Some(power_up) = power_up {
            if power_up.effect_type == drinks::PowerUpType::SpeedBoost && power_up.active {
                // Get red power sprites resource
                if let Some(red_sprites) = &red_power_sprites {
                    // Use red power-up sprites based on player state
                    if player.pushing_snowball {
                        let push_index = player.push_frame % player_sprites.push_snow.len();
                        *texture = player_sprites.push_snow[push_index].clone();
                    } else if player.kicking_snowball {
                        *texture = player_sprites.kick_snowball.clone();
                    } else if player.is_attacking {
                        let attack_index = player.attack_frame % player_sprites.attack.len();
                        *texture = player_sprites.attack[attack_index].clone();
                    } else if player.is_jumping || player.is_dropping {
                        let jump_index = player.animation_frame % player_sprites.jump.len();
                        *texture = player_sprites.jump[jump_index].clone();
                    } else if player.is_moving {
                        // Use red walk sprites
                        let walk_index = player.animation_frame % red_sprites.walk.len();
                        *texture = red_sprites.walk[walk_index].clone();
                    } else {
                        // Use red idle sprites
                        let idle_index = player.animation_frame % red_sprites.idle.len();
                        *texture = red_sprites.idle[idle_index].clone();
                    }
                    
                    sprite.flip_x = !player.facing_right;
                    sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
                    return;
                }
            }
        }
        
        if player.pushing_snowball {
            let push_index = player.push_frame % player_sprites.push_snow.len();
            *texture = player_sprites.push_snow[push_index].clone();
        } else if player.kicking_snowball {
            *texture = player_sprites.kick_snowball.clone();
        } else if player.is_attacking {
            let attack_index = player.attack_frame % player_sprites.attack.len();
            *texture = player_sprites.attack[attack_index].clone();
        } else if player.is_jumping || player.is_dropping {
            let jump_index = player.animation_frame;
            *texture = player_sprites.jump[jump_index].clone();
        } else if player.is_moving {
            let walk_index = player.animation_frame % player_sprites.left.len();
            *texture = player_sprites.left[walk_index].clone();
        } else {
            *texture = player_sprites.idle.clone();
        }
        
        sprite.flip_x = !player.facing_right;
        sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    }
}
pub fn handle_snowball_interaction(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    _commands: Commands,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut snowball_query: Query<(&mut Snowball, &Transform, &mut crate::Velocity), Without<Player>>,
) {
    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        if player.pushing_snowball || player.kicking_snowball {
            player.push_timer.tick(time.delta());
            
            if player.push_timer.finished() {
                player.pushing_snowball = false;
                player.kicking_snowball = false;
                player.push_timer.reset();
                player.push_frame = 0;
            }
            
            if player.pushing_snowball {
                let elapsed_fraction = player.push_timer.elapsed_secs() / player.push_timer.duration().as_secs_f32();
                if elapsed_fraction > 0.5 && player.push_frame == 0 {
                    player.push_frame = 1;
                }
            }
            
            if player.pushing_snowball && keyboard.just_pressed(KeyCode::KeyX) {
                for (mut snowball, snowball_transform, mut snowball_velocity) in snowball_query.iter_mut() {
                    let distance = (player_transform.translation.truncate() - snowball_transform.translation.truncate()).length();
                    
                    if distance < 45.0 {
                        player.pushing_snowball = false;
                        player.kicking_snowball = true;
                        player.push_timer.reset();
                        
                        let kick_direction = if player.facing_right { -1.0 } else { 1.0 };
                        
                        snowball.state = SnowballState::Kicked;
                        snowball.kicked = true;
                        
                        let kick_speed = 350.0 + (50.0 * snowball.snow_level as f32);
                        snowball_velocity.value.x = kick_direction * kick_speed;
                        snowball_velocity.value.y = 100.0; 
                        
                        break;
                    }
                }
            }
            return;
        }
        if player.is_attacking {
            return;
        }
        
        player.snowball_collision = false;
        player.collision_direction = None;
        
        for (mut snowball, snowball_transform, mut snowball_velocity) in snowball_query.iter_mut() {
            let distance = (player_transform.translation.truncate() - snowball_transform.translation.truncate()).length();
            
            if distance < 45.0 {
                player.snowball_collision = true;
                
                let direction = (player_transform.translation - snowball_transform.translation).normalize();
                player.collision_direction = Some(Vec2::new(direction.x, direction.y));
                
                if keyboard.just_pressed(KeyCode::KeyX) {
                    player.kicking_snowball = true;
                    player.push_timer.reset();
                    
                    let kick_direction = if snowball_transform.translation.x > player_transform.translation.x { 1.0 } else { -1.0 };
                    
                    snowball.state = SnowballState::Kicked;
                    snowball.kicked = true;
                    snowball.roll_timer = Timer::from_seconds(0.05, TimerMode::Repeating); 
                    
                    // Calculate if the ball should go left or right based on kick direction
                    let kick_direction = if player.facing_right { -1.0 } else { 1.0 };
                    
                    // Give a strong horizontal kick with minimal upward momentum
                    let kick_speed = 400.0 + (50.0 * snowball.snow_level as f32);
                    snowball_velocity.value.x = kick_direction * kick_speed;
                    snowball_velocity.value.y = 50.0; 
                    
                    return;
                }
                
                if player.on_ground && player.is_moving {
                    let player_facing_right = player.facing_right;
                    let player_going_left = player_facing_right;
                    
                    let is_player_moving_towards_snowball = 
                        (player_going_left && player_transform.translation.x > snowball_transform.translation.x) ||
                        (!player_going_left && player_transform.translation.x < snowball_transform.translation.x);
                    
                    if is_player_moving_towards_snowball {
                        player.pushing_snowball = true;
                        player.push_timer.reset();
                        player.push_frame = 0;
                        
                        let push_direction = if player_going_left { -1.0 } else { 1.0 };
                        snowball_velocity.value.x = push_direction * 100.0;
                        
                        if snowball.snow_level < 3 {
                            snowball.snow_level += 1;
                        }
                        
                        player.snowball_collision = false;
                        player.collision_direction = None;
                        
                        return;
                    }
                }
            }
        }
    }
}
pub fn detect_player_snowball_collision(
    mut query_set: ParamSet<(
        Query<(&mut Player, &mut Transform, &mut crate::Velocity)>,
        Query<&Transform, With<Snowball>>
    )>,
) {
    let mut snowball_positions = Vec::new();
    for snowball_transform in query_set.p1().iter() {
        snowball_positions.push(snowball_transform.translation);
    }

    if let Ok((mut player, mut player_transform, mut player_velocity)) = query_set.p0().get_single_mut() {
        if player.pushing_snowball || player.kicking_snowball {
            return;
        }
        
        player.snowball_collision = false;
        player.collision_direction = None;
        let player_size = Vec2::new(40.0, 50.0);
        let player_left = player_transform.translation.x - player_size.x / 2.0;
        let player_right = player_transform.translation.x + player_size.x / 2.0;
        let player_top = player_transform.translation.y + player_size.y / 2.0;
        let player_bottom = player_transform.translation.y - player_size.y / 2.0;
        
        for snowball_pos in snowball_positions.iter() {
            let snowball_size = Vec2::new(40.0, 40.0);
            let snowball_left = snowball_pos.x - snowball_size.x / 2.0;
            let snowball_right = snowball_pos.x + snowball_size.x / 2.0;
            let snowball_top = snowball_pos.y + snowball_size.y / 2.0;
            let snowball_bottom = snowball_pos.y - snowball_size.y / 2.0;
            
            if player_right > snowball_left && player_left < snowball_right &&
               player_top > snowball_bottom && player_bottom < snowball_top {
                
                player.snowball_collision = true;
                
                let left_overlap = player_right - snowball_left;
                let right_overlap = snowball_right - player_left;
                let top_overlap = player_bottom - snowball_top;
                let bottom_overlap = snowball_bottom - player_top;
                
                let min_x_overlap = left_overlap.min(right_overlap);
                let min_y_overlap = top_overlap.abs().min(bottom_overlap.abs());
                
                if min_x_overlap < min_y_overlap {
                    if left_overlap < right_overlap {
                        player_transform.translation.x = snowball_left - player_size.x / 2.0;
                        player_velocity.value.x = 0.0;
                        player.collision_direction = Some(Vec2::new(1.0, 0.0));
                    } else {
                        player_transform.translation.x = snowball_right + player_size.x / 2.0;
                        player_velocity.value.x = 0.0;
                        player.collision_direction = Some(Vec2::new(-1.0, 0.0));
                    }
                } else {
                    if top_overlap.abs() < bottom_overlap.abs() {
                        player_transform.translation.y = snowball_top + player_size.y / 2.0;
                        player_velocity.value.y = 0.0;
                        player.on_ground = true;
                        player.collision_direction = Some(Vec2::new(0.0, -1.0));
                    } else {
                        player_transform.translation.y = snowball_bottom - player_size.y / 2.0;
                        player_velocity.value.y = 0.0;
                        player.collision_direction = Some(Vec2::new(0.0, 1.0));
                    }
                }
                
                break;
            }
        }
    }
}

#[derive(Component)]
pub struct HealthHeart {
    pub index: u8,
}

pub fn setup_health_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create heart sprites for each health point
    for i in 0..10 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("heart_full.png"), // You'll need this asset
                transform: Transform::from_xyz(-350.0 + (i as f32 * 25.0), 280.0, 10.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                ..default()
            },
            HealthHeart { index: i },
        ));
    }
}

pub fn update_health_display(
    player_query: Query<&Player>,
    mut heart_query: Query<(&mut Sprite, &HealthHeart)>,
    _asset_server: Res<AssetServer>,
) {
    if let Ok(player) = player_query.get_single() {
        for (mut sprite, heart) in heart_query.iter_mut() {
            if heart.index < player.health {
                sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0); // Visible
            } else {
                sprite.color = Color::rgba(1.0, 1.0, 1.0, 0.3); // Faded
            }
        }
    }
}

#[derive(Component)]
pub struct GameOverText;


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
) {
    // Initialize timers if needed
    if death_timer.is_none() {
        *death_timer = Some(Timer::from_seconds(0.2, TimerMode::Repeating));
    }
    
    if respawn_timer.is_none() {
        *respawn_timer = Some(Timer::from_seconds(2.0, TimerMode::Once));
    }
    
    // Handle existing player
    let player_exists = !player_query.is_empty();
    
    if player_exists {
        // Get the player entity - we'll loop through to handle potential multiple players
        for (entity, mut player, _transform, mut texture) in player_query.iter_mut() {
            if player.is_dead {
                // Death animation sequence
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
                            // Decrease lives 
                            println!("Player died! Lives before: {}", player.lives);
                            player.lives = player.lives.saturating_sub(1);
                            println!("Lives after: {}", player.lives);
                            
                            // Despawn player
                            commands.entity(entity).despawn();
                            
                            // Set respawn timer based on lives
                            let respawn_time = if player.lives > 0 { 2.0 } else { 4.0 };
                            *respawn_timer = Some(Timer::from_seconds(respawn_time, TimerMode::Once));
                            *animation_frame = 0;
                            
                            // If game over, spawn game over text
                            if player.lives == 0 {
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
                                
                                // Only reset wave when player is completely out of lives
                                wave_system.current_wave = 0;
                                wave_system.spawn_timer.reset();
                                wave_system.spawning = true;
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    } else {
        // Player doesn't exist, check if it's time to respawn
        if let Some(timer) = respawn_timer.as_mut() {
            timer.tick(time.delta());
            
            if timer.finished() {
                // Check if we need to reset after game over
                let mut game_over = false;
                
                for entity in game_over_query.iter() {
                    // Game over text exists
                    commands.entity(entity).despawn();
                    game_over = true;
                }
                
                let new_lives = if game_over {
                    3 // Full reset after game over
                } else {
                    for (_entity, player, _, _) in player_query.iter() {
                        if player.lives == 0 {
                            return;
                        }
                    }
                    1 
                };
                
                println!("Respawning player with {} lives", new_lives);
                
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
                        lives: new_lives,
                        max_lives: 3,
                    },
                ));
                
                // Reset the respawn timer and animation frame
                *timer = Timer::from_seconds(0.2, TimerMode::Repeating);
                *animation_frame = 1; // Start the appear animation
            }
        }
    }
    
    // Handle respawn animation for existing player
    if player_exists {
        let mut found_alive_player = false;
        
        for (_entity, player, _, mut texture) in player_query.iter_mut() {
            if !player.is_dead {
                found_alive_player = true;
                
                // Handle respawn animation if needed
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
        
        // Clean up any "Game Over" text if the player is alive
        if found_alive_player {
            for entity in game_over_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}