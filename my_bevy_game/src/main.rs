use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, apply_gravity)
        .add_systems(Update, apply_velocity)
        .add_systems(Update, update_sprite)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, handle_attack)
        .add_systems(Update, update_snowballs)
        .add_systems(Update, draw_power_indicator)
        .run();
}

#[derive(Component)]
struct Velocity {
    value: Vec2,
}

#[derive(Component)]
struct Gravity;

#[derive(Component)]
struct Platform {
    size: Vec2,
}

#[derive(Component)]
struct BlockingPlatform;

#[derive(Component)]
struct Player {
    facing_right: bool,
    is_moving: bool,
    is_jumping: bool,
    on_ground: bool,
    is_dropping: bool,
    is_attacking: bool,
    attack_timer: Timer,
    attack_frame: usize,
    drop_timer: Timer,
    animation_timer: Timer,
    animation_frame: usize,
    x_button_held: bool,
    x_button_hold_time: f32,
    max_hold_time: f32,
}

#[derive(Component)]
struct PowerIndicator;

#[derive(Component)]
struct Snowball {
    lifetime: Timer,
    direction: f32,
    speed: f32,
    phase: SnowballPhase,
    phase_timer: Timer,
    power: f32,
}

#[derive(PartialEq)]
enum SnowballPhase {
    Straight,
    Down,
}

#[derive(Resource)]
struct PlayerSprites {
    idle: Handle<Image>,
    left: Vec<Handle<Image>>,
    jump: Vec<Handle<Image>>,
    attack: Vec<Handle<Image>>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());
    
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
    
    for handle in [
        &idle_handle, &left_handle1, &left_handle2, &left_handle3,
        &jump_handle1, &jump_handle2, &jump_handle3, &jump_handle4,
        &attack_handle1, &attack_handle2,
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
    };
    commands.insert_resource(player_sprites);
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..default()
        },
        ..default()
    });
    
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
        Velocity { value: Vec2::new(0.0, 0.0) },
        Gravity,
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
        },
    ));
    
    spawn_platform(&mut commands, Vec2::new(0.0, -250.0), Vec2::new(800.0, 55.0));
    
    spawn_platform(&mut commands, Vec2::new(0.0, 250.0), Vec2::new(800.0, 25.0));
    
    spawn_platform(&mut commands, Vec2::new(-300.0, -150.0), Vec2::new(200.0, 49.0));
    spawn_platform(&mut commands, Vec2::new(5.0, -168.0), Vec2::new(200.0, 45.0));
    spawn_platform(&mut commands, Vec2::new(300.0, -155.0), Vec2::new(200.0, 40.0));
    
    spawn_platform(&mut commands, Vec2::new(0.0, -90.0), Vec2::new(500.0, 40.0));
    
    spawn_platform(&mut commands, Vec2::new(-225.0, 5.0), Vec2::new(350.0, 40.0));
    spawn_platform(&mut commands, Vec2::new(225.0, 5.0), Vec2::new(350.0, 40.0));
    
    spawn_platform(&mut commands, Vec2::new(0.0, 80.0), Vec2::new(600.0, 40.0));
    
    spawn_platform(&mut commands, Vec2::new(0.0, 100.0), Vec2::new(400.0, 60.0));
    
    spawn_blocking_platform(&mut commands, Vec2::new(0.0, 120.0), Vec2::new(400.0, 40.0));
}

fn spawn_platform(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.9, 0.5, 0.1, 0.0),
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        },
        Platform { size },
    ));
}

fn spawn_blocking_platform(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.9, 0.5, 0.1, 0.0),
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        },
        Platform { size },
        BlockingPlatform,
    ));
}

fn spawn_snowball(commands: &mut Commands, position: Vec2, facing_right: bool, power: f32) {
    let direction = if facing_right { -1.0 } else { 1.0 };
    let offset = Vec2::new(direction * 30.0, 5.0);
    
    let base_speed = 180.0;
    let speed = base_speed * (0.5 + power * 1.5);
    
    let base_lifetime = 1.2;
    let lifetime = base_lifetime * (0.5 + power * 0.8);
    
    let size = 15.0 + (power * 5.0);
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.9, 0.9, 1.0),
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(position.x + offset.x, position.y + offset.y, 2.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Snowball {
            lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
            direction,
            speed,
            phase: SnowballPhase::Straight,
            phase_timer: Timer::from_seconds(0.5, TimerMode::Once),
            power,
        },
    ));
}

fn draw_power_indicator(
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

fn handle_attack(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Player)>,
) {
    if let Ok((transform, mut player)) = query.get_single_mut() {
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
            
            spawn_snowball(&mut commands, transform.translation.truncate(), player.facing_right, power);
            
            player.x_button_hold_time = 0.0;
        }
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Player)>,
) {
    if let Ok((mut velocity, mut player)) = query.get_single_mut() {
        let mut direction = 0.0;
        player.is_moving = false;
        
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
        
        velocity.value.x = direction * 200.0;
        
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

fn apply_gravity(
    mut query: Query<&mut Velocity, With<Gravity>>,
    time: Res<Time>,
) {
    let gravity = Vec2::new(0.0, -9.8);
    
    for mut velocity in &mut query {
        velocity.value += gravity * time.delta_seconds() * 30.0;
    }
}

fn update_snowballs(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Snowball, &mut Transform)>,
) {
    for (entity, mut snowball, mut transform) in query.iter_mut() {
        snowball.lifetime.tick(time.delta());
        snowball.phase_timer.tick(time.delta());
        
        if snowball.phase == SnowballPhase::Straight && snowball.phase_timer.finished() {
            snowball.phase = SnowballPhase::Down;
        }
        
        match snowball.phase {
            SnowballPhase::Straight => {
                transform.translation.x += snowball.direction * snowball.speed * time.delta_seconds();
                
                if snowball.power > 0.6 {
                    let phase_progress = snowball.phase_timer.elapsed_secs() / 
                                        snowball.phase_timer.duration().as_secs_f32();
                    let upward_force = 50.0 * snowball.power * (1.0 - phase_progress);
                    transform.translation.y += upward_force * time.delta_seconds();
                }
            },
            SnowballPhase::Down => {
                transform.translation.x += snowball.direction * (snowball.speed * 0.5) * time.delta_seconds();
                transform.translation.y -= (200.0 + 100.0 * snowball.power) * time.delta_seconds();
            }
        }
        
        transform.rotation = Quat::from_rotation_z(
            snowball.lifetime.elapsed_secs() * 6.28 * (1.0 + snowball.power)
        );
        
        if snowball.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut player_query: ParamSet<(
        Query<(&Transform, &Platform, Option<&BlockingPlatform>)>,
        Query<(&mut Transform, &mut Velocity, &mut Player)>
    )>,
) {
    let mut platforms = Vec::new();
    let mut blocking_platforms = Vec::new();
    
    for (transform, platform, blocking) in player_query.p0().iter() {
        if blocking.is_some() {
            blocking_platforms.push((transform.translation, platform.size));
        } else {
            platforms.push((transform.translation, platform.size));
        }
    }
    
    let mut player_query = player_query.p1();
    for (mut transform, mut velocity, mut player) in player_query.iter_mut() {
        let old_position = transform.translation;
        
        transform.translation.x += velocity.value.x * time.delta_seconds();
        
        transform.translation.y += velocity.value.y * time.delta_seconds();
        
        player.on_ground = false;
        
        for (platform_pos, platform_size) in &platforms {
            let platform_left = platform_pos.x - platform_size.x / 2.0;
            let platform_right = platform_pos.x + platform_size.x / 2.0;
            let platform_top = platform_pos.y + platform_size.y / 2.0;
            
            let player_size = Vec2::new(50.0, 50.0);
            let player_left = transform.translation.x - player_size.x / 2.0;
            let player_right = transform.translation.x + player_size.x / 2.0;
            let player_bottom = transform.translation.y - player_size.y / 2.0;
            
            if player_right > platform_left && player_left < platform_right {
                if old_position.y - player_size.y / 2.0 >= platform_top && 
                   player_bottom < platform_top && 
                   velocity.value.y < 0.0 && 
                   !player.is_dropping {
                    transform.translation.y = platform_top + player_size.y / 2.0;
                    velocity.value.y = 0.0;
                    player.on_ground = true;
                }
            }
        }
        
        for (platform_pos, platform_size) in &blocking_platforms {
            let platform_left = platform_pos.x - platform_size.x / 2.0;
            let platform_right = platform_pos.x + platform_size.x / 2.0;
            let platform_top = platform_pos.y + platform_size.y / 2.0;
            let platform_bottom = platform_pos.y - platform_size.y / 2.0;
            
            let player_size = Vec2::new(50.0, 50.0);
            let player_left = transform.translation.x - player_size.x / 2.0;
            let player_right = transform.translation.x + player_size.x / 2.0;
            let player_top = transform.translation.y + player_size.y / 2.0;
            let player_bottom = transform.translation.y - player_size.y / 2.0;
            
            if player_right > platform_left && player_left < platform_right &&
               player_bottom < platform_top && player_top > platform_bottom {
                
                let left_overlap = player_right - platform_left;
                let right_overlap = platform_right - player_left;
                let top_overlap = player_bottom - platform_top;
                let bottom_overlap = platform_bottom - player_top;
                
                let min_x_overlap = left_overlap.min(right_overlap);
                let min_y_overlap = top_overlap.abs().min(bottom_overlap.abs());
                
                if min_x_overlap < min_y_overlap {
                    if left_overlap < right_overlap {
                        transform.translation.x = platform_left - player_size.x / 2.0;
                    } else {
                        transform.translation.x = platform_right + player_size.x / 2.0;
                    }
                    velocity.value.x = 0.0;
                } else {
                    if top_overlap.abs() < bottom_overlap.abs() {
                        if !player.is_dropping {
                            transform.translation.y = platform_top + player_size.y / 2.0;
                            velocity.value.y = 0.0;
                            player.on_ground = true;
                        }
                    } else {
                        transform.translation.y = platform_bottom - player_size.y / 2.0;
                        velocity.value.y = 0.0;
                    }
                }
            }
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<&mut Player>,
) {
    if let Ok(mut player) = query.get_single_mut() {
        if player.is_attacking {
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

fn update_sprite(
    player_sprites: Res<PlayerSprites>,
    mut query: Query<(&Player, &mut Handle<Image>, &mut Sprite)>,
) {
    if let Ok((player, mut texture, mut sprite)) = query.get_single_mut() {
        if player.is_attacking {
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