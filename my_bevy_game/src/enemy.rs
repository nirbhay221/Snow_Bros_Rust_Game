use bevy::prelude::*;
use crate::Velocity;
use crate::Gravity;
use rand::prelude::*;

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
}

#[derive(PartialEq)]
pub enum EnemyState {
    Idle,
    Walking,
    Jumping,
    Rolling,
}

#[derive(Resource)]
pub struct EnemySprites {
    pub idle: Handle<Image>,
    pub walk: Vec<Handle<Image>>,
    pub jump: Vec<Handle<Image>>,
    pub roll: Vec<Handle<Image>>,
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
    
    for handle in [
        &idle_handle, &walk_handle1, &walk_handle2,
        &jump_handle1, &jump_handle2,
        &roll_handle1, &roll_handle2, &roll_handle3, &roll_handle4,
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
            state: EnemyState::Idle,
        },
    ));
}

pub fn enemy_movement(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Enemy, &Transform)>,
) {
    let mut rng = rand::thread_rng();
    
    for (mut velocity, mut enemy, transform) in query.iter_mut() {
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
                    if enemy.on_ground {
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

pub fn update_enemy_sprite(
    enemy_sprites: Res<EnemySprites>,
    mut query: Query<(&Enemy, &mut Handle<Image>, &mut Sprite)>,
) {
    for (enemy, mut texture, mut sprite) in query.iter_mut() {
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
        }
        
        sprite.flip_x = !enemy.facing_right;
    }
}