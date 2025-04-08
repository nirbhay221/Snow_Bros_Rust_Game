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
struct Player {
    facing_right: bool,
    is_moving: bool,
    is_jumping: bool,
    on_ground: bool,
    animation_timer: Timer,
    animation_frame: usize,
}

#[derive(Resource)]
struct PlayerSprites {
    idle: Handle<Image>,
    left: Vec<Handle<Image>>,
    jump: Vec<Handle<Image>>,
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
    
    if let Some(image) = images.get_mut(&idle_handle) {
        image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
    }
    if let Some(image) = images.get_mut(&left_handle1) {
        image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
    }
    if let Some(image) = images.get_mut(&left_handle2) {
        image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
    }
    if let Some(image) = images.get_mut(&left_handle3) {
        image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
    }
    if let Some(image) = images.get_mut(&jump_handle1) {
        image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
    }
    if let Some(image) = images.get_mut(&jump_handle2) {
        image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
    }
    if let Some(image) = images.get_mut(&jump_handle3) {
        image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
    }
    if let Some(image) = images.get_mut(&jump_handle4) {
        image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
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
            animation_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            animation_frame: 0,
        },
    ));
    
    spawn_platform(&mut commands, Vec2::new(0.0, -235.0), Vec2::new(800.0, 25.0));
    
    spawn_platform(&mut commands, Vec2::new(0.0, -150.0), Vec2::new(100.0, 25.0));
    spawn_platform(&mut commands, Vec2::new(200.0, -150.0), Vec2::new(200.0, 25.0));
    spawn_platform(&mut commands, Vec2::new(0.0, -100.0), Vec2::new(300.0, 25.0));
    
    spawn_platform(&mut commands, Vec2::new(-200.0, 0.0), Vec2::new(400.0, 25.0));
    spawn_platform(&mut commands, Vec2::new(200.0, 0.0), Vec2::new(400.0, 25.0));
    spawn_platform(&mut commands, Vec2::new(0.0, 100.0), Vec2::new(200.0, 25.0));
    
    spawn_platform(&mut commands, Vec2::new(-350.0, -50.0), Vec2::new(100.0, 25.0));
    spawn_platform(&mut commands, Vec2::new(350.0, -50.0), Vec2::new(100.0, 25.0));
}

fn spawn_platform(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.9, 0.5, 0.1),
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        },
        Platform { size },
    ));
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player)>,
) {
    if let Ok((mut velocity, mut player)) = query.get_single_mut() {
        let mut direction = 0.0;
        player.is_moving = false;
        
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

fn apply_velocity(
    time: Res<Time>,
    mut player_query: ParamSet<(
        Query<(&Transform, &Platform)>,
        Query<(&mut Transform, &mut Velocity, &mut Player)>
    )>,
) {
    let mut platforms = Vec::new();
    for (transform, platform) in player_query.p0().iter() {
        platforms.push((transform.translation, platform.size));
    }
    
    let mut player_query = player_query.p1();
    for (mut transform, mut velocity, mut player) in player_query.iter_mut() {
        let old_position = transform.translation;
        
        transform.translation.x += velocity.value.x * time.delta_seconds();
        
        for (platform_pos, platform_size) in &platforms {
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
                if old_position.x + player_size.x / 2.0 <= platform_left && player_right > platform_left {
                    transform.translation.x = platform_left - player_size.x / 2.0;
                }
                else if old_position.x - player_size.x / 2.0 >= platform_right && player_left < platform_right {
                    transform.translation.x = platform_right + player_size.x / 2.0;
                }
            }
        }
        
        transform.translation.y += velocity.value.y * time.delta_seconds();
        
        player.on_ground = false;
        
        for (platform_pos, platform_size) in &platforms {
            let platform_left = platform_pos.x - platform_size.x / 2.0;
            let platform_right = platform_pos.x + platform_size.x / 2.0;
            let platform_top = platform_pos.y + platform_size.y / 2.0;
            let platform_bottom = platform_pos.y - platform_size.y / 2.0;
            
            let player_size = Vec2::new(50.0, 50.0);
            let player_left = transform.translation.x - player_size.x / 2.0;
            let player_right = transform.translation.x + player_size.x / 2.0;
            let player_top = transform.translation.y + player_size.y / 2.0;
            let player_bottom = transform.translation.y - player_size.y / 2.0;
            
            if player_right > platform_left && player_left < platform_right {
                if old_position.y - player_size.y / 2.0 >= platform_top && player_bottom < platform_top {
                    transform.translation.y = platform_top + player_size.y / 2.0;
                    velocity.value.y = 0.0;
                    player.on_ground = true;
                }
                else if old_position.y + player_size.y / 2.0 <= platform_bottom && player_top > platform_bottom {
                    transform.translation.y = platform_bottom - player_size.y / 2.0;
                    velocity.value.y = 0.0;
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
        if player.is_jumping {
            let jump_index = player.animation_frame;
            *texture = player_sprites.jump[jump_index].clone();
        } else if player.is_moving {
            let walk_index = player.animation_frame % player_sprites.left.len();
            *texture = player_sprites.left[walk_index].clone();
        } else {
            *texture = player_sprites.idle.clone();
        }
        
        sprite.flip_x = !player.facing_right;
    }
}