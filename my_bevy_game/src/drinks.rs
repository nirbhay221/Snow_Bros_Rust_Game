use bevy::prelude::*;
use crate::Gravity;
use crate::Velocity;
use crate::player;
use rand::prelude::*;

// Power-up system for gameplay: drops drinks that give the player special abilities

#[derive(Component)]
pub struct Drink {
    pub color: DrinkColor,
    pub animation_timer: Timer,
    pub animation_frame: usize,
    pub lifetime_timer: Timer,
    pub landed: bool,
}

#[derive(PartialEq, Clone, Copy)]
pub enum DrinkColor {
    Red,     // Speed boost
    Green,   // Extra life
    Blue,    // Health restore
}

#[derive(Resource)]
pub struct DrinkSprites {
    pub red: Vec<Handle<Image>>,
    pub green: Vec<Handle<Image>>,
    pub blue: Vec<Handle<Image>>,
}

#[derive(Component)]
pub struct PowerUpEffect {
    pub effect_type: PowerUpType,
    pub timer: Timer,
    pub active: bool,
}

#[derive(PartialEq, Clone, Copy)]
pub enum PowerUpType {
    SpeedBoost,     // Red drink effect
    ExtraLife,      // Green drink effect
    HealthRestore,  // Blue drink effect
}

#[derive(Resource)]
pub struct RedPowerSprites {
    pub idle: Vec<Handle<Image>>,
    pub walk: Vec<Handle<Image>>,
}

// Initialize all drink and power-up related resources
pub fn setup_drink_resource(
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
) {
    let red_drink_first = asset_server.load("red_drink_first.png");
    let red_drink_second = asset_server.load("red_drink_second.png");
    let red_drink_third = asset_server.load("red_drink_third.png");
    
    let green_drink_first = asset_server.load("green_drink_first.png");
    let green_drink_second = asset_server.load("green_drink_second.png");
    let green_drink_third = asset_server.load("green_drink_third.png");
    
    let blue_drink_first = asset_server.load("blue_drink_first.png");
    let blue_drink_second = asset_server.load("blue_drink_second.png");
    let blue_drink_third = asset_server.load("blue_drink_third.png");
    
    for handle in [
        &red_drink_first, &red_drink_second, &red_drink_third,
        &green_drink_first, &green_drink_second, &green_drink_third,
        &blue_drink_first, &blue_drink_second, &blue_drink_third,
    ].iter() {
        if let Some(image) = images.get_mut(*handle) {
            image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
        }
    }
    
    let drink_sprites = DrinkSprites {
        red: vec![
            red_drink_first.clone(),
            red_drink_second.clone(),
            red_drink_third.clone(),
        ],
        green: vec![
            green_drink_first.clone(),
            green_drink_second.clone(),
            green_drink_third.clone(),
        ],
        blue: vec![
            blue_drink_first.clone(),
            blue_drink_second.clone(),
            blue_drink_third.clone(),
        ],
    };
    
    commands.insert_resource(drink_sprites);
    
    let player_red_idle_first = asset_server.load("player_red_idle_first.png");
    let player_red_idle_second = asset_server.load("player_red_idle_second.png");
    let player_red_walk_first = asset_server.load("player_red_walk_first.png");
    let player_red_walk_second = asset_server.load("player_red_walk_second.png");
    
    for handle in [
        &player_red_idle_first, &player_red_idle_second,
        &player_red_walk_first, &player_red_walk_second,
    ].iter() {
        if let Some(image) = images.get_mut(*handle) {
            image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
        }
    }
    
    let red_power_sprites = RedPowerSprites {
        idle: vec![
            player_red_idle_first.clone(),
            player_red_idle_second.clone(),
        ],
        walk: vec![
            player_red_walk_first.clone(),
            player_red_walk_second.clone(),
        ],
    };
    
    commands.insert_resource(red_power_sprites);
}

// Spawns a random drink power-up at the given position
// Used when enemies are defeated to drop a power-up reward
pub fn spawn_drink(
    commands: &mut Commands,
    position: Vec2,
    drink_sprites: &Res<DrinkSprites>,
) {
    let mut rng = rand::thread_rng();
    
    let drink_color = match rng.gen_range(0..3) {
        0 => DrinkColor::Red,
        1 => DrinkColor::Green,
        _ => DrinkColor::Blue,
    };
    
    let initial_texture = match drink_color {
        DrinkColor::Red => drink_sprites.red[0].clone(),
        DrinkColor::Green => drink_sprites.green[0].clone(),
        DrinkColor::Blue => drink_sprites.blue[0].clone(),
    };
    
    // Add a small bounce effect when dropped
    let initial_velocity = Vec2::new(
        rng.gen_range(-20.0..20.0),
        rng.gen_range(50.0..100.0)
    );
    
    commands.spawn((
        SpriteBundle {
            texture: initial_texture,
            transform: Transform::from_xyz(position.x, position.y, 1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            ..default()
        },
        Velocity { value: initial_velocity },
        Gravity,
        Drink {
            color: drink_color,
            animation_timer: Timer::from_seconds(0.3, TimerMode::Repeating),
            animation_frame: 0,
            lifetime_timer: Timer::from_seconds(30.0, TimerMode::Once),
            landed: false,
        },
    ));
}

pub fn animate_drinks(
    time: Res<Time>,
    mut drink_query: Query<&mut Drink>,
) {
    for mut drink in drink_query.iter_mut() {
        drink.animation_timer.tick(time.delta());
        
        if drink.animation_timer.just_finished() {
            drink.animation_frame = (drink.animation_frame + 1) % 3;
        }
    }
}

pub fn update_drink_sprites(
    drink_sprites: Res<DrinkSprites>,
    mut query: Query<(&Drink, &mut Handle<Image>)>,
) {
    for (drink, mut texture) in query.iter_mut() {
        let frame_index = drink.animation_frame;
        
        *texture = match drink.color {
            DrinkColor::Red => drink_sprites.red[frame_index].clone(),
            DrinkColor::Green => drink_sprites.green[frame_index].clone(),
            DrinkColor::Blue => drink_sprites.blue[frame_index].clone(),
        };
    }
}

pub fn check_drink_landing(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Drink, &Velocity)>,
) {
    for (entity, mut drink, velocity) in query.iter_mut() {
        if !drink.landed && velocity.value.y.abs() < 0.1 {
            drink.landed = true;
            
            commands.entity(entity).remove::<Gravity>();
        }
    }
}

pub fn handle_drink_lifetime(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Drink)>,
) {
    for (entity, mut drink) in query.iter_mut() {
        drink.lifetime_timer.tick(time.delta());
        
        if drink.lifetime_timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// Detects when player collects a drink and applies the appropriate power-up effect
// Power-ups are exclusive - collecting a new one replaces any active effect
pub fn handle_player_drink_collection(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform, &player::Player)>,
    drink_query: Query<(Entity, &Transform, &Drink)>,
    mut lives_storage: Option<ResMut<crate::lives::LivesStorage>>,
) {
    if let Ok((player_entity, player_transform, player)) = player_query.get_single() {
        let player_pos = player_transform.translation.truncate();
        let player_size = Vec2::new(40.0, 50.0);
        
        for (drink_entity, drink_transform, drink) in drink_query.iter() {
            let drink_pos = drink_transform.translation.truncate();
            let drink_size = Vec2::new(40.0, 40.0);
            
            if (player_pos.x - drink_pos.x).abs() < (player_size.x + drink_size.x) / 2.0 &&
               (player_pos.y - drink_pos.y).abs() < (player_size.y + drink_size.y) / 2.0 {
                
                commands.entity(player_entity).remove::<PowerUpEffect>();
                
                match drink.color {
                    DrinkColor::Red => {
                        // Speed boost lasts for 60 seconds
                        commands.entity(player_entity).insert(PowerUpEffect {
                            effect_type: PowerUpType::SpeedBoost,
                            timer: Timer::from_seconds(60.0, TimerMode::Once),
                            active: true,
                        });
                        println!("Player collected RED drink: Speed boost activated!");
                    },
                    DrinkColor::Green => {
                        // Extra life is a one-time immediate effect
                        commands.entity(player_entity).insert(PowerUpEffect {
                            effect_type: PowerUpType::ExtraLife,
                            timer: Timer::from_seconds(0.1, TimerMode::Once),
                            active: true,
                        });
                        println!("Player collected GREEN drink: Extra life added!");
                    },
                    DrinkColor::Blue => {
                        // Health restore is a one-time immediate effect
                        commands.entity(player_entity).insert(PowerUpEffect {
                            effect_type: PowerUpType::HealthRestore,
                            timer: Timer::from_seconds(0.1, TimerMode::Once),
                            active: true,
                        });
                        println!("Player collected BLUE drink: Health restored!");
                    },
                }
                
                // Remove the drink from the world
                commands.entity(drink_entity).despawn();
            }
        }
    }
}

pub fn handle_power_up_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut player::Player, Option<&mut PowerUpEffect>)>,
    mut lives_storage: Option<ResMut<crate::lives::LivesStorage>>,
) {
    for (entity, mut player, power_up) in player_query.iter_mut() {
        if let Some(mut power_up) = power_up {
            power_up.timer.tick(time.delta());
            
            match power_up.effect_type {
                PowerUpType::SpeedBoost => {
                    if power_up.timer.finished() {
                        commands.entity(entity).remove::<PowerUpEffect>();
                    }
                },
                PowerUpType::ExtraLife => {
                    if power_up.active {
                        player.lives += 1;
                        player.max_lives += 1;
                        
                        if let Some(mut lives_storage) = lives_storage.as_mut() {
                            lives_storage.current_lives += 1;
                            lives_storage.max_lives += 1;
                        }
                        
                        power_up.active = false;
                        
                        if power_up.timer.finished() {
                            commands.entity(entity).remove::<PowerUpEffect>();
                        }
                    }
                },
                PowerUpType::HealthRestore => {
                    if power_up.active {
                        player.health = player.max_health;
                        power_up.active = false;
                        
                        if power_up.timer.finished() {
                            commands.entity(entity).remove::<PowerUpEffect>();
                        }
                    }
                },
            }
        }
    }
}

pub fn update_red_power_sprite(
    mut player_query: Query<(&player::Player, &mut Handle<Image>, Option<&PowerUpEffect>)>,
    player_sprites: Res<player::PlayerSprites>,
    red_power_sprites: Res<RedPowerSprites>,
) {
    if let Ok((player, mut texture, power_up)) = player_query.get_single_mut() {
        if let Some(power_up) = power_up {
            if power_up.effect_type == PowerUpType::SpeedBoost && power_up.active {
                if player.is_jumping || player.is_dropping {
                    let jump_index = player.animation_frame % player_sprites.jump.len();
                    *texture = player_sprites.jump[jump_index].clone();
                } else if player.is_moving {
                    let walk_index = player.animation_frame % red_power_sprites.walk.len();
                    *texture = red_power_sprites.walk[walk_index].clone();
                } else {
                    let idle_index = player.animation_frame % red_power_sprites.idle.len();
                    *texture = red_power_sprites.idle[idle_index].clone();
                }
                return;
            }
        }
    }
}

pub fn apply_speed_boost(
    mut player_query: Query<(&mut Velocity, Option<&PowerUpEffect>), With<player::Player>>,
) {
    if let Ok((mut velocity, power_up)) = player_query.get_single_mut() {
        if let Some(power_up) = power_up {
            if power_up.effect_type == PowerUpType::SpeedBoost && power_up.active {
                velocity.value.x *= 1.2;
                
                if velocity.value.y > 0.0 {
                    velocity.value.y *= 1.1;
                }
                
                velocity.value.x = velocity.value.x.clamp(-300.0, 300.0);
                velocity.value.y = velocity.value.y.clamp(-400.0, 400.0);
            }
        }
    }
}