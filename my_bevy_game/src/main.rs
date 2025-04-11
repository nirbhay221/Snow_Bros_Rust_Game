use bevy::prelude::*;
mod player;
mod enemy;
mod snowball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player::player_movement)
        .add_systems(Update, player::update_sprite)
        .add_systems(Update, player::animate_sprite)
        .add_systems(Update, player::handle_attack)
        .add_systems(Update, player::update_beams)
        .add_systems(Update, player::draw_power_indicator)
        .add_systems(Update, player::handle_snowball_interaction)
        .add_systems(Update, player::detect_player_snowball_collision)
        .add_systems(Update, enemy::enemy_movement)
        .add_systems(Update, enemy::animate_enemy)
        .add_systems(Update, enemy::update_enemy_sprite)
        .add_systems(Update, snowball::update_snowball)
        .add_systems(Update, snowball::animate_snowball)
        .add_systems(Update, snowball::handle_snowball_collision)
        .add_systems(Update, detect_beam_enemy_collision)
        .add_systems(Update, detect_beam_snowball_collision)
        .add_systems(Update, apply_gravity)
        .add_systems(Update, apply_velocity)
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
    
    enemy::setup_enemy(commands.reborrow(), &asset_server, &mut images, Vec3::new(-200.0, 150.0, 1.0));
    
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

fn detect_beam_enemy_collision(
    mut commands: Commands,
    beams_query: Query<(Entity, &Transform, &player::Beam)>,
    enemies_query: Query<(Entity, &Transform), (With<enemy::Enemy>, Without<snowball::Snowball>)>,
    snowball_sprites: Option<Res<snowball::SnowballSprites>>,
) {
    let Some(snowball_sprites) = snowball_sprites else {
        return;
    };
    
    for (beam_entity, beam_transform, beam) in beams_query.iter() {
        let beam_width = 40.0 + (beam.power * 20.0);
        let beam_height = 20.0 + (beam.power * 10.0);
        
        let beam_left = beam_transform.translation.x - beam_width / 2.0;
        let beam_right = beam_transform.translation.x + beam_width / 2.0;
        let beam_top = beam_transform.translation.y + beam_height / 2.0;
        let beam_bottom = beam_transform.translation.y - beam_height / 2.0;
        
        for (enemy_entity, enemy_transform) in enemies_query.iter() {
            let enemy_size = Vec2::new(50.0, 50.0);
            let enemy_left = enemy_transform.translation.x - enemy_size.x / 2.0;
            let enemy_right = enemy_transform.translation.x + enemy_size.x / 2.0;
            let enemy_top = enemy_transform.translation.y + enemy_size.y / 2.0;
            let enemy_bottom = enemy_transform.translation.y - enemy_size.y / 2.0;
            
            if beam_right > enemy_left && beam_left < enemy_right &&
               beam_top > enemy_bottom && beam_bottom < enemy_top {
                commands.entity(beam_entity).despawn();
                commands.entity(enemy_entity).despawn();
                snowball::spawn_snowball(
                    &mut commands,
                    enemy_transform.translation.truncate(),
                    &snowball_sprites,
                    1, 
                );
                
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

fn apply_velocity(
    time: Res<Time>,
    mut entity_query: ParamSet<(
        Query<(&Transform, &Platform, Option<&BlockingPlatform>)>,
        Query<(
            &mut Transform, 
            &mut Velocity, 
            Option<&mut player::Player>, 
            Option<&mut enemy::Enemy>, 
            Option<&mut snowball::Snowball>
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
    
    let mut entity_query = entity_query.p1();
    for (mut transform, mut velocity, player_opt, enemy_opt, snowball_opt) in entity_query.iter_mut() {
        let old_position = transform.translation;
        
        transform.translation.x += velocity.value.x * time.delta_seconds();
        
        transform.translation.y += velocity.value.y * time.delta_seconds();
        if let Some(mut player) = player_opt {
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
        if let Some(mut enemy) = enemy_opt {
            enemy.on_ground = false;
            
            for (platform_pos, platform_size) in &platforms {
                let platform_left = platform_pos.x - platform_size.x / 2.0;
                let platform_right = platform_pos.x + platform_size.x / 2.0;
                let platform_top = platform_pos.y + platform_size.y / 2.0;
                
                let enemy_size = Vec2::new(50.0, 50.0);
                let enemy_left = transform.translation.x - enemy_size.x / 2.0;
                let enemy_right = transform.translation.x + enemy_size.x / 2.0;
                let enemy_bottom = transform.translation.y - enemy_size.y / 2.0;
                
                if enemy_right > platform_left && enemy_left < platform_right {
                    if old_position.y - enemy_size.y / 2.0 >= platform_top && 
                       enemy_bottom < platform_top && 
                       velocity.value.y < 0.0 && 
                       !enemy.is_dropping {
                        transform.translation.y = platform_top + enemy_size.y / 2.0;
                        velocity.value.y = 0.0;
                        enemy.on_ground = true;
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
                            if !enemy.is_dropping {
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