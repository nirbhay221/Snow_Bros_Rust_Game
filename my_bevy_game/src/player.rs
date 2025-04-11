use bevy::prelude::*;
use crate::snowball::{Snowball, SnowballState};

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
}

#[derive(Component)]
pub struct PowerIndicator;

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
    mut query: Query<(&mut crate::Velocity, &mut Player)>,
) {
    if let Ok((mut velocity, mut player)) = query.get_single_mut() {
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
        
        if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
            if player.on_ground {
                player.is_dropping = true;
                player.on_ground = false;
                velocity.value.y = -5.0;
            }
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
    mut query: Query<&mut Player>,
) {
    if let Ok(mut player) = query.get_single_mut() {
        if player.is_attacking || player.pushing_snowball || player.kicking_snowball {
            return;
        }
        
        player.animation_timer.tick(time.delta());
        
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
            player.animation_frame = 0;
        }
    }
}

pub fn update_sprite(
    player_sprites: Res<PlayerSprites>,
    mut query: Query<(&Player, &mut Handle<Image>, &mut Sprite)>,
) {
    if let Ok((player, mut texture, mut sprite)) = query.get_single_mut() {
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
                        
                        snowball.state = SnowballState::Rolling;
                        
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
                    
                    snowball.state = SnowballState::Rolling;
                    
                    let kick_speed = 350.0 + (50.0 * snowball.snow_level as f32);
                    snowball_velocity.value.x = kick_direction * kick_speed;
                    snowball_velocity.value.y = 100.0; 
                    
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