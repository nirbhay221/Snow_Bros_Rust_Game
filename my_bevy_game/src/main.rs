use bevy::prelude::*;
mod player;
mod enemy;
use rand::Rng;
mod snowball;
mod ui;
mod lives;

mod drinks;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, |mut commands: Commands, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>| {
            enemy::setup_enemy2(commands.reborrow(), &asset_server, &mut images, Vec3::new(150.0, 100.0, 1.0));
        })
        .add_systems(Update, enemy::handle_enemy2_attacks)
        .add_systems(Update, enemy::update_enemy_beams)
        .add_systems(Update, enemy::handle_enemy5_attacks)
        .add_systems(Startup, |mut commands: Commands, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>| {
            enemy::setup_enemy3(commands.reborrow(), &asset_server, &mut images, Vec3::new(-100.0, 100.0, 1.0));
            enemy::setup_enemy5(commands.reborrow(), &asset_server, &mut images, Vec3::new(100.0, 100.0, 1.0));
        })
        
        // Add lives storage system
        .add_systems(Startup, lives::setup_lives_storage)
        
        .add_systems(Update, enemy::detect_enemy_beam_player_collision)
        .add_systems(Startup, setup)
        .add_systems(Startup, enemy::setup_wave_system)
        .add_systems(Startup, |mut commands: Commands, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>| {
            drinks::setup_drink_resource(commands.reborrow(), &asset_server, &mut images);
        })
        // Player systems
        .add_systems(Update, player::player_movement)
        .add_systems(Update, player::update_sprite)
        .add_systems(Update, player::animate_sprite)
        .add_systems(Update, player::handle_attack)
        .add_systems(Update, player::update_beams)
        .add_systems(Update, player::draw_power_indicator)
        .add_systems(Update, player::handle_snowball_interaction)
        .add_systems(Update, player::detect_player_snowball_collision)
        
        // Enemy systems
        .add_systems(Update, enemy::enemy_movement)
        .add_systems(Update, enemy::animate_enemy)
        .add_systems(Update, enemy::update_enemy_sprites) 
        .add_systems(Update, enemy::animate_enemy_death)
        .add_systems(Update, enemy::manage_waves)
        .add_systems(Update, enemy::handle_enemy2_attacks)
        .add_systems(Update, enemy::update_enemy_beams)
        
        // Core physics systems with proper ordering
        .add_systems(Update, apply_gravity.before(apply_velocity))
        .add_systems(Update, apply_velocity.before(snowball::update_snowball))
        .add_systems(Update, drinks::handle_drink_lifetime)
        .add_systems(Update, drinks::handle_player_drink_collection.after(apply_velocity))
        .add_systems(Update, drinks::handle_power_up_effects)
        .add_systems(Update, drinks::apply_speed_boost.after(player::player_movement))
        .add_systems(Update, drinks::update_red_power_sprite.after(player::update_sprite))

        // Snowball systems with ordering
        .add_systems(Update, snowball::update_snowball.before(snowball::animate_snowball))
        .add_systems(Update, snowball::animate_snowball.after(snowball::update_snowball))
        .add_systems(Update, snowball::handle_snowball_collision.after(snowball::update_snowball))
        .add_systems(Update, snowball::animate_dying_enemies)
        .add_systems(Update, snowball::handle_snowball_melting)
        .add_systems(Update, snowball::animate_reviving_enemies)
        
        // Drink systems with ordering
        .add_systems(Update, drinks::animate_drinks)
        .add_systems(Update, drinks::update_drink_sprites)
        .add_systems(Update, drinks::check_drink_landing.after(apply_velocity))
        .add_systems(Update, drinks::handle_drink_lifetime)
        .add_systems(Update, drinks::handle_player_drink_collection.after(apply_velocity))
        .add_systems(Update, drinks::handle_power_up_effects)
        .add_systems(Update, drinks::apply_speed_boost.after(player::player_movement))
        .add_systems(Update, drinks::update_red_power_sprite.after(player::update_sprite))
        
        // Collision systems with ordering
        .add_systems(Update, detect_beam_enemy_collision)
        .add_systems(Update, detect_beam_snowball_collision)
        
        // Player status systems - Use the updated player death handler
        .add_systems(Update, player::handle_player_enemy_collision)
        .add_systems(Update, player::update_health_display)
        .add_systems(Update, lives::handle_player_death) 
        .add_systems(Startup, player::setup_health_bar)
        .add_systems(Update, player::update_health_bar)
        
        // UI systems
        .add_systems(Startup, ui::setup_ui)
        .add_systems(Update, ui::update_wave_display)
        .add_systems(Update, ui::update_lives_display)
        .add_systems(Update, ui::update_break_timer_display) 
        .add_systems(Update, ui::display_wave_announcement)
        .add_systems(Update, debug_entity_counts)
        
        .run();
}
#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}

#[derive(Component)]
pub struct Gravity;

#[derive(Component)]
struct Platform {
    size: Vec2,
}

#[derive(Component)]
struct BlockingPlatform;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());
    
    snowball::setup_snowball_resource(commands.reborrow(), &asset_server, &mut images);
    
    player::setup_player(commands.reborrow(), &asset_server, &mut images);
    
    let enemy_positions = [
        Vec3::new(200.0, 50.0, 1.0),
        Vec3::new(-100.0, -50.0, 1.0),
    ];
    
    for position in enemy_positions.iter() {
        enemy::setup_enemy(commands.reborrow(), &asset_server, &mut images, *position);
    }
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..default()
        },
        ..default()
    });
    
    // Setup platforms 
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

    let mut wave_system = enemy::WaveSystem::default();
    wave_system.spawning = true; 
    commands.insert_resource(wave_system);
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

fn detect_beam_enemy_collision(
    mut commands: Commands,
    beams_query: Query<(Entity, &Transform, &player::Beam)>,
    enemies_query: Query<(Entity, &Transform, &enemy::Enemy), Without<snowball::Snowball>>,
    snowball_sprites: Option<Res<snowball::SnowballSprites>>,
    drink_sprites: Option<Res<drinks::DrinkSprites>>,
) {
    let (Some(snowball_sprites), Some(drink_sprites)) = (snowball_sprites, drink_sprites) else {
        println!("Warning: Missing required resources in detect_beam_enemy_collision");
        return;
    };
    
    for (beam_entity, beam_transform, beam) in beams_query.iter() {
        let beam_width = 40.0 + (beam.power * 20.0);
        let beam_height = 20.0 + (beam.power * 10.0);
        
        let beam_left = beam_transform.translation.x - beam_width / 2.0;
        let beam_right = beam_transform.translation.x + beam_width / 2.0;
        let beam_top = beam_transform.translation.y + beam_height / 2.0;
        let beam_bottom = beam_transform.translation.y - beam_height / 2.0;
        
        for (enemy_entity, enemy_transform, enemy) in enemies_query.iter() {
            let enemy_size = Vec2::new(50.0, 50.0);
            let enemy_left = enemy_transform.translation.x - enemy_size.x / 2.0;
            let enemy_right = enemy_transform.translation.x + enemy_size.x / 2.0;
            let enemy_top = enemy_transform.translation.y + enemy_size.y / 2.0;
            let enemy_bottom = enemy_transform.translation.y - enemy_size.y / 2.0;
            
            if beam_right > enemy_left && beam_left < enemy_right &&
               beam_top > enemy_bottom && beam_bottom < enemy_top {
                println!("Beam hit enemy at position: {:?}", enemy_transform.translation);
                
                // Despawn beam and enemy
                commands.entity(beam_entity).despawn();
                commands.entity(enemy_entity).despawn();
                
                // Spawn a snowball at the enemy position with the enemy type
                snowball::spawn_snowball(
                    &mut commands,
                    enemy_transform.translation.truncate(),
                    &snowball_sprites,
                    1,
                    Some(enemy.enemy_type.clone()),
                );
                
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.3) { 
                    drinks::spawn_drink(
                        &mut commands,
                        enemy_transform.translation.truncate(),
                        &drink_sprites,
                    );
                    println!("Drink spawned from enemy!");
                }
                
                println!("Enemy turned into snowball");
                break;
            }
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

fn debug_entity_counts(
    time: Res<Time>,
    drinks: Query<Entity, With<drinks::Drink>>,
    snowballs: Query<Entity, With<snowball::Snowball>>,
    enemies: Query<Entity, With<enemy::Enemy>>,
    players: Query<Entity, With<player::Player>>,
    mut timer: Local<Option<Timer>>,
) {
    if timer.is_none() {
        *timer = Some(Timer::from_seconds(5.0, TimerMode::Repeating));
    }
    
    if let Some(t) = timer.as_mut() {
        t.tick(time.delta());
        
        if t.just_finished() {
            println!("Entity counts - Drinks: {}, Snowballs: {}, Enemies: {}, Players: {}",
                drinks.iter().count(),
                snowballs.iter().count(),
                enemies.iter().count(),
                players.iter().count()
            );
        }
    }
}
fn apply_velocity(
    time: Res<Time>,
    mut entity_query: ParamSet<(
        Query<(&Transform, &Platform, Option<&BlockingPlatform>)>,
        Query<(
            &mut Transform, 
            &mut Velocity, 
            Option<&mut player::Player>, 
            Option<&mut enemy::Enemy>, 
            Option<&mut snowball::Snowball>,
            Option<&mut drinks::Drink>
        )>
    )>,
) {
    let mut platforms = Vec::new();
    let mut blocking_platforms = Vec::new();
    
    for (transform, platform, blocking) in entity_query.p0().iter() {
        if blocking.is_some() {
            blocking_platforms.push((transform.translation, platform.size));
        } else {
            platforms.push((transform.translation, platform.size));
        }
    }
    
    let base_platform_y = -250.0;
    
    let mut entity_query = entity_query.p1();
    for (mut transform, mut velocity, player_opt, enemy_opt, snowball_opt, drink_opt) in entity_query.iter_mut() {
        let old_position = transform.translation;
        
        transform.translation.x += velocity.value.x * time.delta_seconds();
        transform.translation.y += velocity.value.y * time.delta_seconds();
        
        if let Some(mut player) = player_opt {
            let player_size = Vec2::new(50.0, 50.0);
            let player_bottom = transform.translation.y - player_size.y / 2.0;
            
            if player_bottom < base_platform_y {
                continue;
            }
            
            player.on_ground = false;
            
            for (platform_pos, platform_size) in &platforms {
                let platform_left = platform_pos.x - platform_size.x / 2.0;
                let platform_right = platform_pos.x + platform_size.x / 2.0;
                let platform_top = platform_pos.y + platform_size.y / 2.0;
                let is_base_platform = (platform_pos.y - base_platform_y).abs() < 1.0;
                let player_left = transform.translation.x - player_size.x / 2.0;
                let player_right = transform.translation.x + player_size.x / 2.0;
                let player_bottom = transform.translation.y - player_size.y / 2.0;
                
                if player_right > platform_left && player_left < platform_right {
                    if old_position.y - player_size.y / 2.0 >= platform_top && 
                       player_bottom < platform_top && 
                       velocity.value.y < 0.0 && 
                       (!player.is_dropping || is_base_platform) { 
                        transform.translation.y = platform_top + player_size.y / 2.0;
                        velocity.value.y = 0.0;
                        player.on_ground = true;
                        if player.is_dropping && is_base_platform {
                            player.is_dropping = false;
                            player.drop_timer.reset();
                        }
                    }
                }
            }
            
            for (platform_pos, platform_size) in &blocking_platforms {
                let platform_left = platform_pos.x - platform_size.x / 2.0;
                let platform_right = platform_pos.x + platform_size.x / 2.0;
                let platform_top = platform_pos.y + platform_size.y / 2.0;
                let platform_bottom = platform_pos.y - platform_size.y / 2.0;
                
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
        
        if let Some(mut enemy) = enemy_opt {
            enemy.on_ground = false;
            
            for (platform_pos, platform_size) in &platforms {
                let platform_left = platform_pos.x - platform_size.x / 2.0;
                let platform_right = platform_pos.x + platform_size.x / 2.0;
                let platform_top = platform_pos.y + platform_size.y / 2.0;
                let is_base_platform = (platform_pos.y - (-250.0)).abs() < 1.0;
                
                let enemy_size = Vec2::new(50.0, 50.0);
                let enemy_left = transform.translation.x - enemy_size.x / 2.0;
                let enemy_right = transform.translation.x + enemy_size.x / 2.0;
                let enemy_bottom = transform.translation.y - enemy_size.y / 2.0;
                
                if enemy_right > platform_left && enemy_left < platform_right {
                    if old_position.y - enemy_size.y / 2.0 >= platform_top && 
                       enemy_bottom < platform_top && 
                       velocity.value.y < 0.0 { 
                        if !enemy.is_dropping || is_base_platform {
                            transform.translation.y = platform_top + enemy_size.y / 2.0;
                            velocity.value.y = 0.0;
                            enemy.on_ground = true;
                            
                            if enemy.is_dropping && is_base_platform {
                                enemy.is_dropping = false;
                            }
                        }
                    }
                }
            }
            
            for (platform_pos, platform_size) in &blocking_platforms {
                let platform_left = platform_pos.x - platform_size.x / 2.0;
                let platform_right = platform_pos.x + platform_size.x / 2.0;
                let platform_top = platform_pos.y + platform_size.y / 2.0;
                let platform_bottom = platform_pos.y - platform_size.y / 2.0;
                
                let enemy_size = Vec2::new(50.0, 50.0);
                let enemy_left = transform.translation.x - enemy_size.x / 2.0;
                let enemy_right = transform.translation.x + enemy_size.x / 2.0;
                let enemy_top = transform.translation.y + enemy_size.y / 2.0;
                let enemy_bottom = transform.translation.y - enemy_size.y / 2.0;
                
                if enemy_right > platform_left && enemy_left < platform_right &&
                   enemy_bottom < platform_top && enemy_top > platform_bottom {
                    
                    let left_overlap = enemy_right - platform_left;
                    let right_overlap = platform_right - enemy_left;
                    let top_overlap = enemy_bottom - platform_top;
                    let bottom_overlap = platform_bottom - enemy_top;
                    
                    let min_x_overlap = left_overlap.min(right_overlap);
                    let min_y_overlap = top_overlap.abs().min(bottom_overlap.abs());
                    
                    if min_x_overlap < min_y_overlap {
                        if left_overlap < right_overlap {
                            transform.translation.x = platform_left - enemy_size.x / 2.0;
                        } else {
                            transform.translation.x = platform_right + enemy_size.x / 2.0;
                        }
                        velocity.value.x = 0.0;
                    } else {
                        if top_overlap.abs() < bottom_overlap.abs() {
                            if !enemy.is_dropping || platform_pos.y < -240.0 {
                                transform.translation.y = platform_top + enemy_size.y / 2.0;
                                velocity.value.y = 0.0;
                                enemy.on_ground = true;
                            }
                        } else {
                            transform.translation.y = platform_bottom - enemy_size.y / 2.0;
                            velocity.value.y = 0.0;
                        }
                    }
                }
            }
            
            // Prevent enemies from going off screen edges
            if transform.translation.x < -380.0 {
                transform.translation.x = -380.0;
                velocity.value.x = 0.0;
                if let enemy::Enemy { facing_right: true, .. } = *enemy {
                    enemy.facing_right = false;
                }
            } else if transform.translation.x > 380.0 {
                transform.translation.x = 380.0;
                velocity.value.x = 0.0;
                if let enemy::Enemy { facing_right: false, .. } = *enemy {
                    enemy.facing_right = true;
                }
            }
        }
        
        if let Some(mut snowball) = snowball_opt {
            snowball.on_ground = false;
            
            for (platform_pos, platform_size) in &platforms {
                let platform_left = platform_pos.x - platform_size.x / 2.0;
                let platform_right = platform_pos.x + platform_size.x / 2.0;
                let platform_top = platform_pos.y + platform_size.y / 2.0;
                
                let snowball_size = Vec2::new(50.0, 50.0);
                let snowball_left = transform.translation.x - snowball_size.x / 2.0;
                let snowball_right = transform.translation.x + snowball_size.x / 2.0;
                let snowball_bottom = transform.translation.y - snowball_size.y / 2.0;
                
                if snowball_right > platform_left && snowball_left < platform_right {
                    if old_position.y - snowball_size.y / 2.0 >= platform_top && 
                       snowball_bottom < platform_top && 
                       velocity.value.y < 0.0 {
                        transform.translation.y = platform_top + snowball_size.y / 2.0;
                        velocity.value.y = 0.0;
                        snowball.on_ground = true;
                        
                        if snowball.state == snowball::SnowballState::Rolling {
                            velocity.value.x *= 0.98;
                            
                            if velocity.value.x.abs() < 5.0 {
                                velocity.value.x = 0.0;
                                snowball.state = snowball::SnowballState::Idle;
                            }
                        }
                    }
                }
            }
            
            for (platform_pos, platform_size) in &blocking_platforms {
                let platform_left = platform_pos.x - platform_size.x / 2.0;
                let platform_right = platform_pos.x + platform_size.x / 2.0;
                let platform_top = platform_pos.y + platform_size.y / 2.0;
                let platform_bottom = platform_pos.y - platform_size.y / 2.0;
                
                let snowball_size = Vec2::new(50.0, 50.0);
                let snowball_left = transform.translation.x - snowball_size.x / 2.0;
                let snowball_right = transform.translation.x + snowball_size.x / 2.0;
                let snowball_top = transform.translation.y + snowball_size.y / 2.0;
                let snowball_bottom = transform.translation.y - snowball_size.y / 2.0;
                
                if snowball_right > platform_left && snowball_left < platform_right &&
                   snowball_bottom < platform_top && snowball_top > platform_bottom {
                    
                    let left_overlap = snowball_right - platform_left;
                    let right_overlap = platform_right - snowball_left;
                    let top_overlap = snowball_bottom - platform_top;
                    let bottom_overlap = platform_bottom - snowball_top;
                    
                    let min_x_overlap = left_overlap.min(right_overlap);
                    let min_y_overlap = top_overlap.abs().min(bottom_overlap.abs());
                    
                    if min_x_overlap < min_y_overlap {
                        if left_overlap < right_overlap {
                            transform.translation.x = platform_left - snowball_size.x / 2.0;
                            
                            if snowball.state == snowball::SnowballState::Rolling {
                                velocity.value.x = -velocity.value.x * 0.7; 
                            } else {
                                velocity.value.x = 0.0;
                            }
                        } else {
                            transform.translation.x = platform_right + snowball_size.x / 2.0;
                            
                            if snowball.state == snowball::SnowballState::Rolling {
                                velocity.value.x = -velocity.value.x * 0.7; 
                            } else {
                                velocity.value.x = 0.0;
                            }
                        }
                    } else {
                        if top_overlap.abs() < bottom_overlap.abs() {
                            transform.translation.y = platform_top + snowball_size.y / 2.0;
                            velocity.value.y = 0.0;
                            snowball.on_ground = true;
                        } else {
                            transform.translation.y = platform_bottom - snowball_size.y / 2.0;
                            velocity.value.y = 0.0;
                        }
                    }
                }
            }
            
            if transform.translation.x < -380.0 {
                transform.translation.x = -380.0;
                if snowball.state == snowball::SnowballState::Rolling {
                    velocity.value.x = -velocity.value.x * 0.7; 
                } else {
                    velocity.value.x = 0.0;
                }
            } else if transform.translation.x > 380.0 {
                transform.translation.x = 380.0;
                if snowball.state == snowball::SnowballState::Rolling {
                    velocity.value.x = -velocity.value.x * 0.7; 
                } else {
                    velocity.value.x = 0.0;
                }
            }
        }
        
        // Handle drink collision with platforms
        if let Some(mut drink) = drink_opt {
            if drink.landed {
                velocity.value = Vec2::ZERO;
                continue;
            }
            
            // Check collisions with regular platforms
            for (platform_pos, platform_size) in &platforms {
                let platform_left = platform_pos.x - platform_size.x / 2.0;
                let platform_right = platform_pos.x + platform_size.x / 2.0;
                let platform_top = platform_pos.y + platform_size.y / 2.0;
                
                let drink_size = Vec2::new(40.0, 40.0);
                let drink_left = transform.translation.x - drink_size.x / 2.0;
                let drink_right = transform.translation.x + drink_size.x / 2.0;
                let drink_bottom = transform.translation.y - drink_size.y / 2.0;
                
                if drink_right > platform_left && drink_left < platform_right {
                    if old_position.y - drink_size.y / 2.0 >= platform_top && 
                       drink_bottom < platform_top && 
                       velocity.value.y < 0.0 {
                        transform.translation.y = platform_top + drink_size.y / 2.0;
                        velocity.value.x = 0.0;
                        velocity.value.y = 0.0;
                        drink.landed = true;
                        break;
                    }
                }
            }
            
            // Check collisions with blocking platforms
            for (platform_pos, platform_size) in &blocking_platforms {
                let platform_left = platform_pos.x - platform_size.x / 2.0;
                let platform_right = platform_pos.x + platform_size.x / 2.0;
                let platform_top = platform_pos.y + platform_size.y / 2.0;
                let platform_bottom = platform_pos.y - platform_size.y / 2.0;
                
                let drink_size = Vec2::new(40.0, 40.0);
                let drink_left = transform.translation.x - drink_size.x / 2.0;
                let drink_right = transform.translation.x + drink_size.x / 2.0;
                let drink_top = transform.translation.y + drink_size.y / 2.0;
                let drink_bottom = transform.translation.y - drink_size.y / 2.0;
                
                if drink_right > platform_left && drink_left < platform_right &&
                   drink_bottom < platform_top && drink_top > platform_bottom {
                    
                    let left_overlap = drink_right - platform_left;
                    let right_overlap = platform_right - drink_left;
                    let top_overlap = drink_bottom - platform_top;
                    let bottom_overlap = platform_bottom - drink_top;
                    
                    let min_x_overlap = left_overlap.min(right_overlap);
                    let min_y_overlap = top_overlap.abs().min(bottom_overlap.abs());
                    
                    if min_x_overlap < min_y_overlap {
                        if left_overlap < right_overlap {
                            transform.translation.x = platform_left - drink_size.x / 2.0;
                        } else {
                            transform.translation.x = platform_right + drink_size.x / 2.0;
                        }
                        velocity.value.x = 0.0;
                    } else {
                        if top_overlap.abs() < bottom_overlap.abs() {
                            transform.translation.y = platform_top + drink_size.y / 2.0;
                            velocity.value.y = 0.0;
                            velocity.value.x = 0.0;
                            drink.landed = true;
                        } else {
                            transform.translation.y = platform_bottom - drink_size.y / 2.0;
                            velocity.value.y = 0.0;
                        }
                    }
                }
            }
            
            // Prevent the drink from going off-screen
            if transform.translation.x < -380.0 {
                transform.translation.x = -380.0;
                velocity.value.x = 0.0;
            } else if transform.translation.x > 380.0 {
                transform.translation.x = 380.0;
                velocity.value.x = 0.0;
            }
        }
    }
}
fn detect_beam_snowball_collision(
    mut commands: Commands,
    beams_query: Query<(Entity, &Transform, &player::Beam)>,
    mut snowballs_query: Query<(Entity, &Transform, &mut snowball::Snowball)>,
) {
    for (beam_entity, beam_transform, beam) in beams_query.iter() {
        let beam_width = 40.0 + (beam.power * 20.0);
        let beam_height = 20.0 + (beam.power * 10.0);
        
        let beam_left = beam_transform.translation.x - beam_width / 2.0;
        let beam_right = beam_transform.translation.x + beam_width / 2.0;
        let beam_top = beam_transform.translation.y + beam_height / 2.0;
        let beam_bottom = beam_transform.translation.y - beam_height / 2.0;
        
        for (_, snowball_transform, mut snowball) in snowballs_query.iter_mut() {
            let snowball_size = Vec2::new(50.0, 50.0);
            let snowball_left = snowball_transform.translation.x - snowball_size.x / 2.0;
            let snowball_right = snowball_transform.translation.x + snowball_size.x / 2.0;
            let snowball_top = snowball_transform.translation.y + snowball_size.y / 2.0;
            let snowball_bottom = snowball_transform.translation.y - snowball_size.y / 2.0;
            
            if beam_right > snowball_left && beam_left < snowball_right &&
               beam_top > snowball_bottom && beam_bottom < snowball_top {
                commands.entity(beam_entity).despawn();
                
                if snowball.snow_level < 3 {
                    snowball.snow_level += 1;
                }
                
                break;
            }
        }
    }
}