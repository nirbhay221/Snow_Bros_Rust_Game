use bevy::prelude::*;
use crate::Velocity;
use crate::Gravity;
use rand::prelude::*;
use crate::Platform;

use crate::player;

#[derive(Component)]
pub struct Enemy {
    pub facing_right: bool,
    pub is_moving: bool,
    pub is_jumping: bool,
    pub on_ground: bool,
    pub is_dropping: bool,
    pub is_rolling: bool,
    pub movement_timer: Timer,
    pub decision_timer: Timer,
    pub animation_timer: Timer,
    pub animation_frame: usize,
    pub roll_timer: Timer,
    pub roll_frame: usize,
    pub state: EnemyState,
    pub death_timer: Timer,
    pub death_frame: usize,
    pub is_dying: bool,
    pub enemy_type: EnemyType,
    pub is_attacking: bool,
    pub attack_direction: AttackDirection,
    pub attack_timer: Timer,
    pub beam_timer: Timer,
    pub beam_active: bool,
    pub damage: u8, 
}

#[derive(PartialEq, Clone, Debug)]
pub enum EnemyType {
    Basic,
    FireBeam,
    Enemy3,  
    Enemy5,
}

#[derive(PartialEq, Clone, Debug)]
pub enum AttackDirection {
    Horizontal,
    Up,
    Down,
}

#[derive(Resource)]
pub struct Enemy3Sprites {
    pub idle: Handle<Image>,
    pub walk: Vec<Handle<Image>>,  
    pub jump: Handle<Image>,
    pub down: Handle<Image>,
    pub death: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub struct Enemy5Sprites {
    pub idle: Handle<Image>,
    pub walk: Vec<Handle<Image>>,  
    pub jump: Handle<Image>,
    pub attack: Vec<Handle<Image>>, 
    pub death: Vec<Handle<Image>>, 
}
#[derive(PartialEq, Clone, Debug)]
pub enum EnemyState {
    Idle,
    Walking,
    Jumping,
    Rolling,
    Dying,
    AttackingHorizontal,
    AttackingUp,
    AttackingDown,
}
#[derive(Resource)]
pub struct Enemy2Sprites {
    pub idle: Handle<Image>,
    pub walk: Vec<Handle<Image>>,
    pub jump: Handle<Image>,
    pub attack_horizontal: Handle<Image>,
    pub attack_up: Handle<Image>,
    pub attack_down: Handle<Image>,
    pub beam_horizontal: Handle<Image>,
    pub beam_up_first: Handle<Image>,
    pub beam_up_second: Handle<Image>,
    pub death: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub struct EnemySprites {
    pub idle: Handle<Image>,
    pub walk: Vec<Handle<Image>>,
    pub jump: Vec<Handle<Image>>,
    pub roll: Vec<Handle<Image>>,
    pub death: Vec<Handle<Image>>,
}
pub fn setup_enemy3(
    mut commands: Commands, 
    asset_server: &Res<AssetServer>, 
    images: &mut ResMut<Assets<Image>>,
    position: Vec3,
) {
    let idle_handle = asset_server.load("enemy_3_idle.png");
    let walk_handle1 = asset_server.load("enemy_3_walk_first.png");
    let walk_handle2 = asset_server.load("enemy_3_walk_second.png");
    let jump_handle = asset_server.load("enemy_3_jump.png");
    let down_handle = asset_server.load("enemy_3_down.png");
    let death_handle1 = asset_server.load("enemy_3_death_first.png");
    let death_handle2 = asset_server.load("enemy_3_death_second.png");
    let death_handle3 = asset_server.load("enemy_3_death_third.png");
    let death_handle4 = asset_server.load("enemy_3_death_fourth.png");
    let death_handle5 = asset_server.load("enemy_3_death_final.png");
    
    for handle in [
        &idle_handle, &walk_handle1, &walk_handle2, &jump_handle, &down_handle,
        &death_handle1, &death_handle2, &death_handle3, &death_handle4, &death_handle5,
    ].iter() {
        if let Some(image) = images.get_mut(*handle) {
            image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
        }
    }
    
    let enemy3_sprites = Enemy3Sprites {
        idle: idle_handle.clone(),
        walk: vec![
            walk_handle1.clone(),
            walk_handle2.clone(),
        ],
        jump: jump_handle.clone(),
        down: down_handle.clone(),
        death: vec![
            death_handle1.clone(),
            death_handle2.clone(),
            death_handle3.clone(),
            death_handle4.clone(),
            death_handle5.clone(),
        ],
    };
    
    commands.insert_resource(enemy3_sprites);
    
    commands.spawn((
        SpriteBundle {
            texture: idle_handle,
            transform: Transform::from_translation(position),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Velocity { value: Vec2::new(0.0, 0.0) },
        Gravity,
        Enemy { 
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
            state: EnemyState::Idle,
            enemy_type: EnemyType::Enemy3,
            is_attacking: false,
            attack_direction: AttackDirection::Horizontal,
            attack_timer: Timer::from_seconds(0.5, TimerMode::Once),
            beam_timer: Timer::from_seconds(0.8, TimerMode::Once),
            beam_active: false,
            damage: 2,
        },
    ));
}
pub fn setup_enemy5(
    mut commands: Commands, 
    asset_server: &Res<AssetServer>, 
    images: &mut ResMut<Assets<Image>>,
    position: Vec3,
) {
    let idle_handle = asset_server.load("enemy_5_idle.png");
    let walk_handle1 = asset_server.load("enemy_5_walk_first.png");
    let walk_handle2 = asset_server.load("enemy_5_walk_second.png");
    let jump_handle = asset_server.load("enemy_5_jump.png");
    let attack_handle1 = asset_server.load("enemy_5_attack_first.png");
    let attack_handle2 = asset_server.load("enemy_5_attack_second.png");
    let death_handle1 = asset_server.load("enemy_5_death_first.png");
    let death_handle2 = asset_server.load("enemy_5_death_second.png");
    let death_handle3 = asset_server.load("enemy_5_death_third.png");
    let death_handle4 = asset_server.load("enemy_5_death_fourth.png");
    let death_handle5 = asset_server.load("enemy_5_death_final.png");
    
    // Format textures
    for handle in [
        &idle_handle, &walk_handle1, &walk_handle2, &jump_handle,
        &attack_handle1, &attack_handle2,
        &death_handle1, &death_handle2, &death_handle3, &death_handle4, &death_handle5,
    ].iter() {
        if let Some(image) = images.get_mut(*handle) {
            image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
        }
    }
    
    let enemy5_sprites = Enemy5Sprites {
        idle: idle_handle.clone(),
        walk: vec![
            walk_handle1.clone(),
            walk_handle2.clone(),
        ],
        jump: jump_handle.clone(),
        attack: vec![
            attack_handle1.clone(),
            attack_handle2.clone(),
        ],
        death: vec![
            death_handle1.clone(),
            death_handle2.clone(),
            death_handle3.clone(),
            death_handle4.clone(),
            death_handle5.clone(),
        ],
    };
    
    commands.insert_resource(enemy5_sprites);
    
    commands.spawn((
        SpriteBundle {
            texture: idle_handle,
            transform: Transform::from_translation(position),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Velocity { value: Vec2::new(0.0, 0.0) },
        Gravity,
        Enemy { 
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
            state: EnemyState::Idle,
            enemy_type: EnemyType::Enemy5,
            is_attacking: false,
            attack_direction: AttackDirection::Horizontal,
            attack_timer: Timer::from_seconds(0.5, TimerMode::Once),
            beam_timer: Timer::from_seconds(0.8, TimerMode::Once),
            beam_active: false,
            damage: 4, 
        },
    ));
}
pub fn setup_enemy2(
    mut commands: Commands, 
    asset_server: &Res<AssetServer>, 
    images: &mut ResMut<Assets<Image>>,
    position: Vec3,
) {
    let idle_handle = asset_server.load("enemy_2_idle.png");
    let walk_handle1 = asset_server.load("enemy_2_walk_first.png");
    let walk_handle2 = asset_server.load("enemy_2_walk_second.png");
    let jump_handle = asset_server.load("enemy_2_jump.png");
    let attack_horizontal = asset_server.load("enemy_2_fight_first.png");
    let attack_up = asset_server.load("enemy_2_attack_up.png");
    let attack_down = asset_server.load("enemy_2_attack_down.png");
    let beam_horizontal = asset_server.load("enemy_2_beam.png");
    let beam_up_first = asset_server.load("enemy_beam_up_first.png");
    let beam_up_second = asset_server.load("enemy_beam_up_second.png");
    let death_handle1 = asset_server.load("enemy_2_death_first.png");
    let death_handle2 = asset_server.load("enemy_2_death_second.png");
    let death_handle3 = asset_server.load("enemy_2_death_third.png");
    let death_handle4 = asset_server.load("enemy_2_death_fourth.png");
    let death_handle5 = asset_server.load("enemy_2_death_final.png");
    
    // Format textures
    for handle in [
        &idle_handle, &walk_handle1, &walk_handle2, &jump_handle,
        &attack_horizontal, &attack_up, &attack_down,
        &beam_horizontal, &beam_up_first, &beam_up_second,
        &death_handle1, &death_handle2, &death_handle3, &death_handle4, &death_handle5,
    ].iter() {
        if let Some(image) = images.get_mut(*handle) {
            image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
        }
    }
    
    let enemy2_sprites = Enemy2Sprites {
        idle: idle_handle.clone(),
        walk: vec![
            walk_handle1.clone(),
            walk_handle2.clone(),
        ],
        jump: jump_handle.clone(),
        attack_horizontal: attack_horizontal.clone(),
        attack_up: attack_up.clone(),
        attack_down: attack_down.clone(),
        beam_horizontal: beam_horizontal.clone(),
        beam_up_first: beam_up_first.clone(),
        beam_up_second: beam_up_second.clone(),
        death: vec![
            death_handle1.clone(),
            death_handle2.clone(),
            death_handle3.clone(),
            death_handle4.clone(),
            death_handle5.clone(),
        ],
    };
    
    commands.insert_resource(enemy2_sprites);
    
    commands.spawn((
        SpriteBundle {
            texture: idle_handle,
            transform: Transform::from_translation(position),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Velocity { value: Vec2::new(0.0, 0.0) },
        Gravity,
        Enemy { 
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
            state: EnemyState::Idle,
            enemy_type: EnemyType::FireBeam,
            is_attacking: false,
            attack_direction: AttackDirection::Horizontal,
            attack_timer: Timer::from_seconds(0.5, TimerMode::Once),
            beam_timer: Timer::from_seconds(0.8, TimerMode::Once),
            beam_active: false,
            damage: 3, 
        },
    ));
}
pub fn spawn_enemy_beam(
    commands: &mut Commands,
    position: Vec2,
    facing_right: bool,
    attack_direction: AttackDirection,
    beam_sprite: Handle<Image>,
    beam_up_sprite: Option<Handle<Image>>,
    rotation: f32,
    damage: u8, 
) {
    let offset = match attack_direction {
        AttackDirection::Horizontal => Vec2::new(if facing_right { -30.0 } else { 30.0 }, 0.0),
        AttackDirection::Up => Vec2::new(0.0, 25.0),
        AttackDirection::Down => Vec2::new(0.0, -25.0),
    };
    
    let direction = match attack_direction {
        AttackDirection::Horizontal => Vec2::new(if facing_right { -1.0 } else { 1.0 }, 0.0),
        AttackDirection::Up => Vec2::new(0.0, 1.0),
        AttackDirection::Down => Vec2::new(0.0, -1.0),
    };
    
    commands.spawn((
        SpriteBundle {
            texture: beam_sprite,
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 15.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(position.x + offset.x, position.y + offset.y, 2.0),
                rotation: Quat::from_rotation_z(rotation),
                ..default()
            },
            ..default()
        },
        EnemyBeam {
            lifetime: Timer::from_seconds(1.5, TimerMode::Once),
            direction,
            speed: 100.0,
            attack_direction,
            damage, 
        },
    ));
}
pub fn handle_enemy_snowball_collision(
    mut commands: Commands,
    snowball_query: Query<(&Transform, &crate::snowball::Snowball)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<crate::snowball::Snowball>>,
) {
    for (snowball_transform, snowball) in snowball_query.iter() {
        // Only rolling snowballs can damage enemies
        if snowball.state != crate::snowball::SnowballState::Rolling {
            continue;
        }
        
        let snowball_pos = snowball_transform.translation;
        let snowball_size = Vec2::new(50.0, 50.0);
        
        for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            // Skip already dying enemies
            if enemy.state == EnemyState::Dying || enemy.is_dying {
                continue;
            }
            
            let enemy_pos = enemy_transform.translation;
            let enemy_size = Vec2::new(50.0, 50.0);
            
            // Simple collision check
            if (enemy_pos.x - snowball_pos.x).abs() < (enemy_size.x + snowball_size.x) * 0.5 &&
               (enemy_pos.y - snowball_pos.y).abs() < (enemy_size.y + snowball_size.y) * 0.5 {
                // Set enemy to dying state
                enemy.is_dying = true;
                enemy.state = EnemyState::Dying; 
                enemy.death_frame = 0;
                
                println!("Enemy hit by snowball, now dying. Type: {:?}", enemy.enemy_type);
            }
        }
    }
}
pub fn handle_enemy_death(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Enemy)>,
    snowball_sprites: Option<Res<crate::snowball::SnowballSprites>>,
) {
    for (entity, mut enemy) in enemy_query.iter_mut() {
        if enemy.is_dying && enemy.state != EnemyState::Dying {
            enemy.state = EnemyState::Dying;
            enemy.death_frame = 0;
            
            println!("Enemy transitioning to dying state. Type: {:?}", enemy.enemy_type);
        }
    }
}

pub fn handle_enemy2_attacks(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut Enemy, &Transform, &mut Velocity)>,
    player_query: Query<&Transform, (With<player::Player>, Without<Enemy>)>,
    enemy2_sprites: Option<Res<Enemy2Sprites>>,
) {
    let Some(enemy2_sprites) = enemy2_sprites else { return };
    
    let player_pos = if let Ok(player_transform) = player_query.get_single() {
        Some(player_transform.translation)
    } else {
        None
    };
    
    for (mut enemy, transform, mut velocity) in query.iter_mut() {
        // Only process enemies of type FireBeam
        if enemy.enemy_type != EnemyType::FireBeam {
            continue;
        }
        
        if enemy.state == EnemyState::Dying {
            continue;
        }
        
        // Handle attack timer if attacking
        if enemy.is_attacking {
            enemy.attack_timer.tick(time.delta());
            
            if enemy.attack_timer.finished() {
                // If attack is finished, shoot a beam
                if !enemy.beam_active {
                    enemy.beam_active = true;
                    enemy.beam_timer.reset();
                    
                    // Get appropriate sprite and rotation based on attack direction
                    let (beam_sprite, beam_up_sprite, rotation) = match enemy.attack_direction {
                        AttackDirection::Horizontal => {
                            (
                                enemy2_sprites.beam_horizontal.clone(),
                                None,
                                if enemy.facing_right { 0.0 } else { std::f32::consts::PI }
                            )
                        },
                        AttackDirection::Up => {
                            (
                                enemy2_sprites.beam_up_first.clone(),
                                Some(enemy2_sprites.beam_up_second.clone()),
                                0.0
                            )
                        },
                        AttackDirection::Down => {
                            (
                                enemy2_sprites.beam_up_first.clone(),
                                Some(enemy2_sprites.beam_up_second.clone()),
                                std::f32::consts::PI
                            )
                        },
                    };
                    
                    spawn_enemy_beam(
                        &mut commands,
                        transform.translation.truncate(),
                        enemy.facing_right,
                        enemy.attack_direction.clone(),
                        beam_sprite,
                        beam_up_sprite,
                        rotation,
                        enemy.damage,
                    );
                }
                
                // Handle beam timer
                enemy.beam_timer.tick(time.delta());
                
                if enemy.beam_timer.finished() {
                    // Attack sequence is complete
                    enemy.is_attacking = false;
                    enemy.beam_active = false;
                    enemy.state = EnemyState::Idle;
                }
            }
            
            continue;
        }
        
        if let Some(player_pos) = player_pos {
            if enemy.on_ground && !enemy.is_rolling {
                let mut rng = rand::thread_rng();
                let should_attack = rng.gen_bool(0.008); 
                
                if should_attack {
                    // Determine attack direction based on player position
                    let enemy_pos = transform.translation;
                    let x_diff = player_pos.x - enemy_pos.x;
                    let y_diff = player_pos.y - enemy_pos.y;
                    
                    // If player is significantly above, attack up
                    if y_diff > 100.0 && y_diff.abs() > x_diff.abs() {
                        enemy.attack_direction = AttackDirection::Up;
                        enemy.state = EnemyState::AttackingUp;
                    }
                    // If player is significantly below, attack down
                    else if y_diff < -100.0 && y_diff.abs() > x_diff.abs() {
                        enemy.attack_direction = AttackDirection::Down;
                        enemy.state = EnemyState::AttackingDown;
                    }
                    // Otherwise attack horizontally
                    else {
                        enemy.attack_direction = AttackDirection::Horizontal;
                        enemy.state = EnemyState::AttackingHorizontal;
                        
                        // Make enemy face the player
                        enemy.facing_right = x_diff < 0.0;
                    }
                    
                    // Start attack sequence
                    enemy.is_attacking = true;
                    enemy.attack_timer.reset();
                    velocity.value.x = 0.0; // Stop movement during attack
                }
            }
        }
    }
}
#[derive(Component)]
pub struct EnemyBeam {
    pub lifetime: Timer,
    pub direction: Vec2,
    pub speed: f32,
    pub attack_direction: AttackDirection,
    pub damage: u8, 
}

pub fn update_enemy_sprites(
    enemy_sprites: Res<EnemySprites>,
    enemy2_sprites: Option<Res<Enemy2Sprites>>,
    enemy3_sprites: Option<Res<Enemy3Sprites>>,
    enemy5_sprites: Option<Res<Enemy5Sprites>>,
    mut query: Query<(&mut Enemy, &mut Handle<Image>, &mut Sprite)>,
) {
    for (mut enemy, mut texture, mut sprite) in query.iter_mut() {
        match enemy.enemy_type {
            EnemyType::Basic => {
                match enemy.state {
                    EnemyState::Idle => {
                        *texture = enemy_sprites.idle.clone();
                    }
                    EnemyState::Walking => {
                        let walk_index = enemy.animation_frame % enemy_sprites.walk.len();
                        *texture = enemy_sprites.walk[walk_index].clone();
                    }
                    EnemyState::Jumping => {
                        let jump_index = enemy.animation_frame % enemy_sprites.jump.len();
                        *texture = enemy_sprites.jump[jump_index].clone();
                    }
                    EnemyState::Rolling => {
                        let roll_index = enemy.roll_frame % enemy_sprites.roll.len();
                        *texture = enemy_sprites.roll[roll_index].clone();
                    }
                    EnemyState::Dying => {
                        let death_index = enemy.death_frame.min(enemy_sprites.death.len() - 1);
                        *texture = enemy_sprites.death[death_index].clone();
                    }
                    _ => {
                        *texture = enemy_sprites.idle.clone();
                    }
                }
            },
            EnemyType::FireBeam => {
                if let Some(ref enemy2_sprites) = enemy2_sprites {
                    match enemy.state {
                        EnemyState::Idle => {
                            *texture = enemy2_sprites.idle.clone();
                        }
                        EnemyState::Walking => {
                            let walk_index = enemy.animation_frame % enemy2_sprites.walk.len();
                            *texture = enemy2_sprites.walk[walk_index].clone();
                        }
                        EnemyState::Jumping => {
                            *texture = enemy2_sprites.jump.clone();
                        }
                        EnemyState::AttackingHorizontal => {
                            *texture = enemy2_sprites.attack_horizontal.clone();
                        }
                        EnemyState::AttackingUp => {
                            *texture = enemy2_sprites.attack_up.clone();
                        }
                        EnemyState::AttackingDown => {
                            *texture = enemy2_sprites.attack_down.clone();
                        }
                        EnemyState::Dying => {
                            let death_index = enemy.death_frame.min(enemy2_sprites.death.len() - 1);
                            *texture = enemy2_sprites.death[death_index].clone();
                        }
                        EnemyState::Rolling => {
                            *texture = enemy2_sprites.idle.clone();
                            
                            enemy.is_rolling = false;
                            enemy.state = EnemyState::Idle;
                        }
                        _ => {
                            *texture = enemy2_sprites.idle.clone();
                        }
                    }
                } else {
                    match enemy.state {
                        EnemyState::Idle => {
                            *texture = enemy_sprites.idle.clone();
                        }
                        EnemyState::Walking => {
                            let walk_index = enemy.animation_frame % enemy_sprites.walk.len();
                            *texture = enemy_sprites.walk[walk_index].clone();
                        }
                        EnemyState::Jumping => {
                            let jump_index = enemy.animation_frame % enemy_sprites.jump.len();
                            *texture = enemy_sprites.jump[jump_index].clone();
                        }
                        EnemyState::Rolling => {
                            enemy.is_rolling = false;
                            enemy.state = EnemyState::Idle;
                            *texture = enemy_sprites.idle.clone();
                        }
                        EnemyState::Dying => {
                            let death_index = enemy.death_frame.min(enemy_sprites.death.len() - 1);
                            *texture = enemy_sprites.death[death_index].clone();
                        }
                        _ => {
                            *texture = enemy_sprites.idle.clone();
                        }
                    }
                }
            },
            EnemyType::Enemy3 => {
                if let Some(ref enemy3_sprites) = enemy3_sprites {
                    match enemy.state {
                        EnemyState::Idle => {
                            *texture = enemy3_sprites.idle.clone();
                        }
                        EnemyState::Walking => {
                            let walk_index = enemy.animation_frame % enemy3_sprites.walk.len();
                            *texture = enemy3_sprites.walk[walk_index].clone();
                        }
                        EnemyState::Jumping => {
                            *texture = enemy3_sprites.jump.clone();
                        }
                        EnemyState::Rolling => {
                            // Use down sprite for rolling behavior
                            *texture = enemy3_sprites.down.clone();
                        }
                        EnemyState::Dying => {
                            let death_index = enemy.death_frame.min(enemy3_sprites.death.len() - 1);
                            *texture = enemy3_sprites.death[death_index].clone();
                        }
                        _ => {
                            *texture = enemy3_sprites.idle.clone();
                        }
                    }
                } else {
                    // Fallback to basic enemy sprites
                    match enemy.state {
                        EnemyState::Idle => { 
                            *texture = enemy_sprites.idle.clone(); 
                        }
                        EnemyState::Walking => {
                            let walk_index = enemy.animation_frame % enemy_sprites.walk.len();
                            *texture = enemy_sprites.walk[walk_index].clone();
                        }
                        EnemyState::Jumping => {
                            let jump_index = enemy.animation_frame % enemy_sprites.jump.len();
                            *texture = enemy_sprites.jump[jump_index].clone();
                        }
                        EnemyState::Rolling => {
                            let roll_index = enemy.roll_frame % enemy_sprites.roll.len();
                            *texture = enemy_sprites.roll[roll_index].clone();
                        }
                        EnemyState::Dying => {
                            let death_index = enemy.death_frame.min(enemy_sprites.death.len() - 1);
                            *texture = enemy_sprites.death[death_index].clone();
                        }
                        _ => { 
                            *texture = enemy_sprites.idle.clone(); 
                        }
                    }
                }
            },
            EnemyType::Enemy5 => {
                if let Some(ref enemy5_sprites) = enemy5_sprites {
                    match enemy.state {
                        EnemyState::Idle => {
                            *texture = enemy5_sprites.idle.clone();
                        }
                        EnemyState::Walking => {
                            let walk_index = enemy.animation_frame % enemy5_sprites.walk.len();
                            *texture = enemy5_sprites.walk[walk_index].clone();
                        }
                        EnemyState::Jumping => {
                            *texture = enemy5_sprites.jump.clone();
                        }
                        EnemyState::AttackingHorizontal => {
                            let attack_index = enemy.animation_frame % enemy5_sprites.attack.len();
                            *texture = enemy5_sprites.attack[attack_index].clone();
                        }
                        EnemyState::Dying => {
                            let death_index = enemy.death_frame.min(enemy5_sprites.death.len() - 1);
                            *texture = enemy5_sprites.death[death_index].clone();
                        }
                        EnemyState::Rolling => {
                            // Cancel rolling for Enemy5 as it doesn't have rolling sprites
                            enemy.is_rolling = false;
                            enemy.state = EnemyState::Idle;
                            *texture = enemy5_sprites.idle.clone();
                        }
                        _ => {
                            *texture = enemy5_sprites.idle.clone();
                        }
                    }
                } else {
                    // Fallback to basic enemy sprites
                    match enemy.state {
                        EnemyState::Idle => { 
                            *texture = enemy_sprites.idle.clone(); 
                        }
                        EnemyState::Walking => {
                            let walk_index = enemy.animation_frame % enemy_sprites.walk.len();
                            *texture = enemy_sprites.walk[walk_index].clone();
                        }
                        EnemyState::Jumping => {
                            let jump_index = enemy.animation_frame % enemy_sprites.jump.len();
                            *texture = enemy_sprites.jump[jump_index].clone();
                        }
                        EnemyState::Rolling => {
                            let roll_index = enemy.roll_frame % enemy_sprites.roll.len();
                            *texture = enemy_sprites.roll[roll_index].clone();
                        }
                        EnemyState::Dying => {
                            let death_index = enemy.death_frame.min(enemy_sprites.death.len() - 1);
                            *texture = enemy_sprites.death[death_index].clone();
                        }
                        _ => { 
                            *texture = enemy_sprites.idle.clone(); 
                        }
                    }
                }
            }
        }
        
        sprite.flip_x = !enemy.facing_right;
    }
}

pub fn handle_enemy5_attacks(
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &Transform, &mut Velocity)>,
    player_query: Query<&Transform, (With<player::Player>, Without<Enemy>)>,
) {
    // Get player position if there is one
    let player_pos = if let Ok(player_transform) = player_query.get_single() {
        Some(player_transform.translation)
    } else {
        None
    };
    
    for (mut enemy, transform, mut velocity) in query.iter_mut() {
        // Only process enemies of type Enemy5
        if enemy.enemy_type != EnemyType::Enemy5 {
            continue;
        }
        
        // Skip dying enemies
        if enemy.state == EnemyState::Dying {
            continue;
        }
        
        // Handle attack timer if attacking
        if enemy.is_attacking {
            enemy.attack_timer.tick(time.delta());
            
            if enemy.attack_timer.finished() {
                // Attack is finished
                enemy.is_attacking = false;
                enemy.state = EnemyState::Idle;
            }
            
            continue;
        }
        
        // Only attack if the enemy has a player target and is on the ground
        if let Some(player_pos) = player_pos {
            if enemy.on_ground && !enemy.is_rolling {
                let mut rng = rand::thread_rng();
                let should_attack = rng.gen_bool(0.001); 
                let x_diff = (player_pos.x - transform.translation.x).abs();
                
                if should_attack && x_diff < 150.0 {
                    // Make enemy face the player
                    enemy.facing_right = player_pos.x < transform.translation.x;
                    
                    // Start attack sequence
                    enemy.is_attacking = true;
                    enemy.state = EnemyState::AttackingHorizontal;
                    enemy.attack_timer.reset();
                    velocity.value.x = 0.0; 
                }
            }
        }
    }

}



pub fn update_enemy_beams(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut EnemyBeam, &mut Transform, &mut Handle<Image>)>,
    enemy2_sprites: Option<Res<Enemy2Sprites>>,
) {
    let Some(enemy2_sprites) = enemy2_sprites else { return };
    
    for (entity, mut beam, mut transform, mut texture) in query.iter_mut() {
        beam.lifetime.tick(time.delta());
        
        // Move the beam
        transform.translation.x += beam.direction.x * beam.speed * time.delta_seconds();
        transform.translation.y += beam.direction.y * beam.speed * time.delta_seconds();
        
        if (beam.attack_direction == AttackDirection::Up || beam.attack_direction == AttackDirection::Down) {
            let progress = beam.lifetime.elapsed_secs() / beam.lifetime.duration().as_secs_f32();
            
            if progress > 0.5 && *texture == enemy2_sprites.beam_up_first {
                *texture = enemy2_sprites.beam_up_second.clone();
            }
        }
        
        // Despawn when lifetime is over
        if beam.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
pub fn update_enemy2_sprite(
    enemy_sprites: Res<EnemySprites>,
    enemy2_sprites: Option<Res<Enemy2Sprites>>,
    mut query: Query<(&Enemy, &mut Handle<Image>, &mut Sprite)>,
) {
    for (enemy, mut texture, mut sprite) in query.iter_mut() {
        if enemy.enemy_type == EnemyType::Basic {
            // Original enemy sprite logic
            match enemy.state {
                EnemyState::Idle => {
                    *texture = enemy_sprites.idle.clone();
                }
                EnemyState::Walking => {
                    let walk_index = enemy.animation_frame % enemy_sprites.walk.len();
                    *texture = enemy_sprites.walk[walk_index].clone();
                }
                EnemyState::Jumping => {
                    let jump_index = enemy.animation_frame % enemy_sprites.jump.len();
                    *texture = enemy_sprites.jump[jump_index].clone();
                }
                EnemyState::Rolling => {
                    let roll_index = enemy.roll_frame % enemy_sprites.roll.len();
                    *texture = enemy_sprites.roll[roll_index].clone();
                }
                EnemyState::Dying => {
                    let death_index = enemy.death_frame.min(enemy_sprites.death.len() - 1);
                    *texture = enemy_sprites.death[death_index].clone();
                    println!("Basic enemy death frame: {}, using sprite: enemy_1_death", enemy.death_frame);
                }
                _ => {}
            }
        } else if enemy.enemy_type == EnemyType::FireBeam {
            // Type 2 enemy needs enemy2_sprites resource
            if let Some(ref enemy2_sprites) = enemy2_sprites {
                match enemy.state {
                    EnemyState::Idle => {
                        *texture = enemy2_sprites.idle.clone();
                    }
                    EnemyState::Walking => {
                        let walk_index = enemy.animation_frame % enemy2_sprites.walk.len();
                        *texture = enemy2_sprites.walk[walk_index].clone();
                    }
                    EnemyState::Jumping => {
                        *texture = enemy2_sprites.jump.clone();
                    }
                    EnemyState::AttackingHorizontal => {
                        *texture = enemy2_sprites.attack_horizontal.clone();
                    }
                    EnemyState::AttackingUp => {
                        *texture = enemy2_sprites.attack_up.clone();
                    }
                    EnemyState::AttackingDown => {
                        *texture = enemy2_sprites.attack_down.clone();
                    }
                    EnemyState::Dying => {
                        let death_index = enemy.death_frame.min(enemy2_sprites.death.len() - 1);
                        *texture = enemy2_sprites.death[death_index].clone();
                        println!("FireBeam enemy death frame: {}, using sprite: enemy_2_death", enemy.death_frame);
                    }
                    EnemyState::Rolling => {
                        let walk_index = enemy.animation_frame % enemy2_sprites.walk.len();
                        *texture = enemy2_sprites.walk[walk_index].clone();
                    }
                }
            } else {
                println!("Warning: enemy2_sprites resource is not available!");
                
                match enemy.state {
                    EnemyState::Idle => {
                        *texture = enemy_sprites.idle.clone();
                    }
                    EnemyState::Walking => {
                        let walk_index = enemy.animation_frame % enemy_sprites.walk.len();
                        *texture = enemy_sprites.walk[walk_index].clone();
                    }
                    EnemyState::Jumping => {
                        let jump_index = enemy.animation_frame % enemy_sprites.jump.len();
                        *texture = enemy_sprites.jump[jump_index].clone();
                    }
                    EnemyState::Rolling => {
                        let roll_index = enemy.roll_frame % enemy_sprites.roll.len();
                        *texture = enemy_sprites.roll[roll_index].clone();
                    }
                    EnemyState::Dying => {
                        let death_index = enemy.death_frame.min(enemy_sprites.death.len() - 1);
                        *texture = enemy_sprites.death[death_index].clone();
                        println!("FireBeam enemy death frame: {}, using fallback sprite", enemy.death_frame);
                    }
                    _ => {}
                }
            }
        }
        
        sprite.flip_x = !enemy.facing_right;
    }
}
pub fn detect_enemy_beam_player_collision(
    mut commands: Commands,
    beams_query: Query<(Entity, &Transform, &EnemyBeam)>,
    mut player_query: Query<(&mut player::Player, &Transform)>,
) {
    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        // Skip if player is already invincible or dead
        if !player.invincibility_timer.finished() || player.is_dead {
            return;
        }
        
        let player_pos = player_transform.translation;
        let player_size = Vec2::new(40.0, 50.0);
        
        for (beam_entity, beam_transform, beam) in beams_query.iter() {
            let beam_pos = beam_transform.translation;
            let beam_size = Vec2::new(30.0, 15.0);
            
            if (player_pos.x - beam_pos.x).abs() < (player_size.x + beam_size.x) / 2.0 &&
               (player_pos.y - beam_pos.y).abs() < (player_size.y + beam_size.y) / 2.0 {
                
                // Damage player with beam's damage amount
                player.health = player.health.saturating_sub(beam.damage);
                
                // Start invincibility timer
                player.invincibility_timer.reset();
                
                // Check if player died
                if player.health == 0 {
                    player.is_dead = true;
                }
                
                // Despawn the beam
                commands.entity(beam_entity).despawn();
                
                break; // Only process one collision per frame
            }
        }
    }
}
pub fn setup_enemy(
    mut commands: Commands, 
    asset_server: &Res<AssetServer>, 
    images: &mut ResMut<Assets<Image>>,
    position: Vec3,
) {
    let idle_handle = asset_server.load("enemy_1_idle.png");
    let walk_handle1 = asset_server.load("enemy_1_walk_first.png");
    let walk_handle2 = asset_server.load("enemy_1_walk_second.png");
    let jump_handle1 = asset_server.load("enemy_1_jump_first.png");
    let jump_handle2 = asset_server.load("enemy_1_jump_second.png");
    let roll_handle1 = asset_server.load("enemy_1_roll_first.png");
    let roll_handle2 = asset_server.load("enemy_1_roll_second.png");
    let roll_handle3 = asset_server.load("enemy_1_roll_third.png");
    let roll_handle4 = asset_server.load("enemy_1_roll_fourth.png");
    let death_handle1 = asset_server.load("enemy_1_death_first.png");
    let death_handle2 = asset_server.load("enemy_1_death_second.png");
    let death_handle3 = asset_server.load("enemy_1_death_third.png");
    let death_handle4 = asset_server.load("enemy_1_death_fourth.png");
    let death_handle5 = asset_server.load("enemy_1_death_final.png");
    
    for handle in [
        &idle_handle, &walk_handle1, &walk_handle2,
        &jump_handle1, &jump_handle2,
        &roll_handle1, &roll_handle2, &roll_handle3, &roll_handle4,
        &death_handle1, &death_handle2, &death_handle3, &death_handle4, &death_handle5,
    ].iter() {
        if let Some(image) = images.get_mut(*handle) {
            image.texture_descriptor.format = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
        }
    }
    
    let enemy_sprites = EnemySprites {
        idle: idle_handle.clone(),
        walk: vec![
            walk_handle1.clone(),
            walk_handle2.clone(),
        ],
        jump: vec![
            jump_handle1.clone(),
            jump_handle2.clone(),
        ],
        roll: vec![
            roll_handle1.clone(),
            roll_handle2.clone(),
            roll_handle3.clone(),
            roll_handle4.clone(),
        ],
        death: vec![
            death_handle1.clone(),
            death_handle2.clone(),
            death_handle3.clone(),
            death_handle4.clone(),
            death_handle5.clone(),
        ],
    };
    
    commands.insert_resource(enemy_sprites);
    
    commands.spawn((
        SpriteBundle {
            texture: idle_handle,
            transform: Transform::from_translation(position),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Velocity { value: Vec2::new(0.0, 0.0) },
        Gravity,
        Enemy { 
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
            state: EnemyState::Idle,
            enemy_type: EnemyType::Basic,
            is_attacking: false,
            attack_direction: AttackDirection::Horizontal,
            attack_timer: Timer::from_seconds(0.5, TimerMode::Once),
            beam_timer: Timer::from_seconds(0.8, TimerMode::Once),
            beam_active: false,
            damage: 1, 
        },
    ));
}

pub fn enemy_movement(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Enemy, &Transform)>,
    platform_query: Query<(&Transform, &Platform)>,
) {
    let mut rng = rand::thread_rng();
    
    for (mut velocity, mut enemy, transform) in query.iter_mut() {
        // Skip movement for dying enemies
        if enemy.state == EnemyState::Dying {
            continue;
        }
        
        enemy.movement_timer.tick(time.delta());
        enemy.decision_timer.tick(time.delta());
        
        if enemy.is_rolling {
            enemy.roll_timer.tick(time.delta());
            
            let roll_speed = if enemy.facing_right { -220.0 } else { 220.0 };
            velocity.value.x = roll_speed;
            
            if enemy.roll_timer.finished() {
                enemy.is_rolling = false;
                enemy.state = EnemyState::Walking;
            }
            
            continue;
        }
        
        // Check if enemy is near an edge
        let mut near_edge = false;
        let mut on_base_platform = false;
        let enemy_pos = transform.translation;
        let enemy_size = Vec2::new(50.0, 50.0);
        let check_distance = 30.0; 
        if enemy.on_ground && (enemy_pos.y - enemy_size.y / 2.0 - (-250.0)).abs() < 5.0 {
            on_base_platform = true;
        }
        
        // Raycast down from a position ahead of the enemy to check for an edge
        let direction = if enemy.facing_right { -1.0 } else { 1.0 };
        let edge_check_pos = Vec3::new(
            enemy_pos.x + direction * (enemy_size.x / 2.0 + check_distance),
            enemy_pos.y,
            enemy_pos.z
        );
        
        // Check if there's a platform below the edge check position
        let mut has_platform_below = false;
        for (platform_transform, platform) in platform_query.iter() {
            let platform_pos = platform_transform.translation;
            let platform_size = platform.size;
            
            let platform_left = platform_pos.x - platform_size.x / 2.0;
            let platform_right = platform_pos.x + platform_size.x / 2.0;
            let platform_top = platform_pos.y + platform_size.y / 2.0;
            
            if edge_check_pos.x >= platform_left && edge_check_pos.x <= platform_right &&
               edge_check_pos.y - enemy_size.y / 2.0 >= platform_top - 5.0 &&
               edge_check_pos.y - enemy_size.y / 2.0 <= platform_top + 5.0 {
                has_platform_below = true;
                break;
            }
        }
        
        near_edge = enemy.on_ground && !has_platform_below;
        
        if near_edge {
            // If near edge, turn around
            enemy.facing_right = !enemy.facing_right;
            
            // Make sure enemy is moving
            enemy.is_moving = true;
            enemy.state = EnemyState::Walking;
            
            continue;
        }
        
        if enemy.decision_timer.just_finished() {
            let decision = rng.gen_range(0..10);
            
            match decision {
                0..=3 => {
                    enemy.facing_right = !enemy.facing_right;
                    enemy.is_moving = true;
                    enemy.state = EnemyState::Walking;
                }
                4..=5 => {
                    if enemy.on_ground {
                        velocity.value.y = 230.0;
                        enemy.is_jumping = true;
                        enemy.on_ground = false;
                        enemy.animation_frame = 0;
                        enemy.state = EnemyState::Jumping;
                    }
                }
                6..=7 => {
                    if enemy.on_ground && !on_base_platform {
                        enemy.is_dropping = true;
                        enemy.on_ground = false;
                        velocity.value.y = -5.0;
                    }
                }
                8..=9 => {
                    if enemy.on_ground && !enemy.is_rolling {
                        enemy.is_rolling = true;
                        enemy.roll_timer.reset();
                        enemy.roll_frame = 0;
                        enemy.state = EnemyState::Rolling;
                    }
                }
                _ => {}
            }
        }
        if enemy.is_moving && !enemy.is_rolling {
            let direction = if enemy.facing_right { -1.0 } else { 1.0 };
            velocity.value.x = direction * 150.0;
        } else if !enemy.is_rolling {
            velocity.value.x = 0.0;
            enemy.state = EnemyState::Idle;
        }
        
        if transform.translation.x < -380.0 && enemy.facing_right {
            enemy.facing_right = false;
        } else if transform.translation.x > 380.0 && !enemy.facing_right {
            enemy.facing_right = true;
        }
    }
}
pub fn animate_enemy(
    time: Res<Time>,
    mut query: Query<&mut Enemy>,
) {
    for mut enemy in query.iter_mut() {
        if enemy.state == EnemyState::Dying {
            continue;
        }
        
        enemy.animation_timer.tick(time.delta());
        
        if enemy.animation_timer.just_finished() {
            match enemy.state {
                EnemyState::Walking => {
                    enemy.animation_frame = (enemy.animation_frame + 1) % 2;
                }
                EnemyState::Jumping => {
                    if enemy.animation_frame < 1 {
                        enemy.animation_frame += 1;
                    }
                    
                    if enemy.on_ground {
                        enemy.is_jumping = false;
                        enemy.state = if enemy.is_moving { EnemyState::Walking } else { EnemyState::Idle };
                    }
                }
                EnemyState::Rolling => {
                    enemy.roll_frame = (enemy.roll_frame + 1) % 4;
                }
                _ => {
                    enemy.animation_frame = 0;
                }
            }
        }
    }
}
pub fn animate_enemy_death(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Enemy, &mut Transform)>,
    enemy_sprites: Res<EnemySprites>,
    enemy2_sprites: Option<Res<Enemy2Sprites>>,
) {
    for (entity, mut enemy, mut transform) in query.iter_mut() {
        if enemy.state == EnemyState::Dying {
            enemy.death_timer.tick(time.delta());
            
            if enemy.death_timer.just_finished() {
                println!(
                    "Death animation frame: {}, Enemy type: {:?}", 
                    enemy.death_frame, 
                    enemy.enemy_type
                );
                
                enemy.death_frame += 1;
                
                if enemy.death_frame == 1 && enemy.enemy_type == EnemyType::Basic {
                    transform.translation.y += 30.0;
                }
                
                let max_frames = match enemy.enemy_type {
                    EnemyType::Basic => enemy_sprites.death.len(),
                    EnemyType::FireBeam => {
                        if let Some(ref enemy2_sprites) = enemy2_sprites {
                            enemy2_sprites.death.len()
                        } else {
                            enemy_sprites.death.len()
                        }
                    },
                    EnemyType::Enemy3 => enemy_sprites.death.len(),
                    EnemyType::Enemy5 => enemy_sprites.death.len()
                };
                
                if enemy.death_frame >= max_frames {
                    println!("Despawning enemy, animation complete. Type: {:?}", enemy.enemy_type);
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}


#[derive(Resource)]
pub struct WaveSystem {
    pub current_wave: usize,
    pub max_waves: usize,
    pub enemies_remaining: usize,
    pub wave_timer: Timer,        
    pub wave_break_timer: Timer,   
    pub spawning: bool,
    pub spawn_timer: Timer,        
    pub spawn_locations: Vec<Vec2>,
    pub is_major_break: bool,     
}

impl Default for WaveSystem {
    fn default() -> Self {
        Self {
            current_wave: 0,
            max_waves: 100,
            enemies_remaining: 0,
            wave_timer: Timer::from_seconds(45.0, TimerMode::Once),   
            wave_break_timer: Timer::from_seconds(5.0, TimerMode::Once), 
            spawning: false,
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            spawn_locations: Vec::new(),
            is_major_break: false,
        }
    }
}

pub fn setup_wave_system(mut commands: Commands) {
    let spawn_locations = vec![
        Vec2::new(-300.0, 100.0),
        Vec2::new(-200.0, 0.0),
        Vec2::new(-100.0, -50.0),
        Vec2::new(0.0, 100.0),
        Vec2::new(100.0, -50.0),
        Vec2::new(200.0, 0.0),
        Vec2::new(300.0, 100.0),
    ];
    
    let wave_system = WaveSystem {
        spawn_locations,
        ..Default::default()
    };
    
    commands.insert_resource(wave_system);
}


pub fn manage_waves(
    mut commands: Commands,
    time: Res<Time>,
    mut wave_system: ResMut<WaveSystem>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    enemy_query: Query<&Enemy>,
) {
    let current_enemies = enemy_query.iter().count();
    wave_system.enemies_remaining = current_enemies;
    
    if !wave_system.spawning {
        wave_system.wave_break_timer.tick(time.delta());
        
        if wave_system.wave_break_timer.finished() {
            // Break is over, start the next wave
            wave_system.spawning = true;
            wave_system.is_major_break = false;
            
            wave_system.wave_timer.reset();
            
            // Move to the next wave
            wave_system.current_wave = (wave_system.current_wave + 1) % wave_system.max_waves;
            
            println!("Starting Wave {}!", wave_system.current_wave + 1);
            
            let base_enemies = 3 + (wave_system.current_wave / 3) as usize;
            let max_enemies = 15; 
            let enemy_count = base_enemies.min(max_enemies);
            
            println!("Spawning {} enemies for wave {}", enemy_count, wave_system.current_wave + 1);
            
            // Generate spawn positions
            let mut rng = rand::thread_rng();
            let spawn_locations = wave_system.spawn_locations.clone();
            
            // Spawn all enemies at once at the start of the wave
            for i in 0..enemy_count {
                // Choose a spawn location
                let spawn_index = i % spawn_locations.len();
                let pos = spawn_locations[spawn_index];
                
                let enemy_type = if wave_system.current_wave < 5 {
                    if rng.gen_bool(0.8) { 0 } else { 1 }
                } else if wave_system.current_wave < 10 {
                    if rng.gen_bool(0.6) { 0 } else { 1 }
                } else if wave_system.current_wave < 20 {
                    let roll = rng.gen_range(0..10);
                    match roll {
                        0..=5 => 0, 
                        6..=8 => 1,
                        _ => if rng.gen_bool(0.5) { 2 } else { 3 }
                    }
                } else if wave_system.current_wave < 30 {
                    let roll = rng.gen_range(0..10);
                    match roll {
                        0..=3 => 0, 
                        4..=6 => 1,
                        7..=8 => 2,
                        _ => 3,    
                    }
                } else {
                    let roll = rng.gen_range(0..10);
                    match roll {
                        0..=2 => 0, 
                        3..=5 => 1, 
                        6..=7 => 2, 
                        _ => 3,     
                    }
                };
                
                match enemy_type {
                    0 => setup_enemy(commands.reborrow(), &asset_server, &mut images, Vec3::new(pos.x, pos.y, 1.0)),
                    1 => setup_enemy2(commands.reborrow(), &asset_server, &mut images, Vec3::new(pos.x, pos.y, 1.0)),
                    2 => setup_enemy3(commands.reborrow(), &asset_server, &mut images, Vec3::new(pos.x, pos.y, 1.0)),
                    _ => setup_enemy5(commands.reborrow(), &asset_server, &mut images, Vec3::new(pos.x, pos.y, 1.0)),
                }
                
                println!("Spawned enemy type {} at position {:?}", enemy_type, pos);
            }
        }
        return;
    }

    if wave_system.spawning {
        wave_system.wave_timer.tick(time.delta());
        
        if current_enemies == 0 || wave_system.wave_timer.finished() {
            wave_system.spawning = false;
            let is_tenth_wave = wave_system.current_wave > 0 && (wave_system.current_wave + 1) % 10 == 0;
            
            if is_tenth_wave {
                wave_system.wave_break_timer = Timer::from_seconds(10.0, TimerMode::Once);
                wave_system.is_major_break = true;
                println!("Wave {} completed! Taking a longer 10-second break.", wave_system.current_wave + 1);
            } else {
                wave_system.wave_break_timer = Timer::from_seconds(5.0, TimerMode::Once);
                println!("Wave {} completed! Short break before next wave.", wave_system.current_wave + 1);
            }
            
            return;
        }
        
    }
}

fn spawn_wave_enemies(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
    wave: usize,
    snowball_sprites_option: &Option<Res<crate::snowball::SnowballSprites>>,
) {
    match wave {
        1 => {
            setup_enemy(commands.reborrow(), asset_server, images, Vec3::new(-200.0, 150.0, 1.0));
            setup_enemy(commands.reborrow(), asset_server, images, Vec3::new(200.0, 150.0, 1.0));
            
            setup_enemy2(commands.reborrow(), asset_server, images, Vec3::new(0.0, 150.0, 1.0));
        },
        2 => {
            setup_enemy(commands.reborrow(), asset_server, images, Vec3::new(-250.0, 150.0, 1.0));
            setup_enemy(commands.reborrow(), asset_server, images, Vec3::new(0.0, 150.0, 1.0));
            setup_enemy(commands.reborrow(), asset_server, images, Vec3::new(250.0, 150.0, 1.0));
            
            setup_enemy3(commands.reborrow(), asset_server, images, Vec3::new(-150.0, 150.0, 1.0));
            
            setup_enemy2(commands.reborrow(), asset_server, images, Vec3::new(150.0, 150.0, 1.0));
            
            if let Some(snowball_sprites) = snowball_sprites_option {
                crate::snowball::spawn_snowball(
                    commands,
                    Vec2::new(0.0, 200.0),
                    snowball_sprites,
                    2,
                    None 
                );
            }
        },
        3 => {
            setup_enemy(commands.reborrow(), asset_server, images, Vec3::new(-300.0, 150.0, 1.0));
            setup_enemy(commands.reborrow(), asset_server, images, Vec3::new(-150.0, 150.0, 1.0));
            
            setup_enemy3(commands.reborrow(), asset_server, images, Vec3::new(0.0, 150.0, 1.0));
            setup_enemy5(commands.reborrow(), asset_server, images, Vec3::new(150.0, 150.0, 1.0));
            
            setup_enemy2(commands.reborrow(), asset_server, images, Vec3::new(300.0, 150.0, 1.0));
            setup_enemy2(commands.reborrow(), asset_server, images, Vec3::new(-200.0, 200.0, 1.0));
            
            setup_enemy5(commands.reborrow(), asset_server, images, Vec3::new(0.0, 200.0, 1.0));
            setup_enemy5(commands.reborrow(), asset_server, images, Vec3::new(200.0, 200.0, 1.0));
            
            if let Some(snowball_sprites) = snowball_sprites_option {
                crate::snowball::spawn_snowball(
                    commands,
                    Vec2::new(-100.0, 200.0),
                    snowball_sprites,
                    2,
                    None
                );
                crate::snowball::spawn_snowball(
                    commands,
                    Vec2::new(100.0, 200.0),
                    snowball_sprites,
                    3,
                    None  
                );
            }
        },
        _ => {}
    }
}

pub fn debug_enemy_death(
    time: Res<Time>,
    enemy_sprites: Res<EnemySprites>,
    enemy2_sprites: Option<Res<Enemy2Sprites>>,
    mut query: Query<(&Enemy, &Handle<Image>)>,
) {
    for (enemy, texture) in query.iter_mut() {
        if enemy.state == EnemyState::Dying {
            // Check which sprite is being used
            if enemy.enemy_type == EnemyType::FireBeam {
                if let Some(ref enemy2_sprites) = enemy2_sprites {
                    let death_index = enemy.death_frame.min(enemy2_sprites.death.len() - 1);
                    let using_correct_sprite = *texture == enemy2_sprites.death[death_index];
                    
                    println!(
                        "Enemy2 death frame: {}, Correct sprite: {}, Type: {:?}",
                        enemy.death_frame,
                        using_correct_sprite,
                        enemy.enemy_type
                    );
                }
            }
        }
    }
}