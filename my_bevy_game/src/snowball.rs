use bevy::prelude::*;
use crate::Velocity;
use crate::Gravity;

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
}

#[derive(PartialEq)]
pub enum SnowballState {
    Idle,
    Pushed,
    Rolling,
}

#[derive(Resource)]
pub struct SnowballSprites {
    pub snow_first: Handle<Image>,
    pub snow_second: Handle<Image>,
    pub snow_third: Handle<Image>,
    pub snow_roll: Vec<Handle<Image>>,
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
}

pub fn spawn_snowball(
    commands: &mut Commands,
    position: Vec2,
    sprites: &Res<SnowballSprites>,
    initial_snow_level: u8,
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
        },
    ));
}

pub fn update_snowball(
    time: Res<Time>,
    mut query: Query<(&mut Snowball, &mut Velocity)>,
) {
    for (mut snowball, mut velocity) in query.iter_mut() {
        snowball.animation_timer.tick(time.delta());
        
        if snowball.state == SnowballState::Rolling {
            snowball.roll_timer.tick(time.delta());
            
            if snowball.roll_timer.just_finished() {
                snowball.roll_frame = (snowball.roll_frame + 1) % 4;
            }
            
            if snowball.on_ground {
                velocity.value.x *= 0.995;
                
                if velocity.value.x.abs() < 20.0 {
                    snowball.state = SnowballState::Idle;
                    velocity.value.x = 0.0;
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
        if snowball.state == SnowballState::Rolling {
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

pub fn handle_snowball_collision(
    _commands: Commands,
    mut snowball_query: Query<(Entity, &Transform, &mut Snowball, &mut Velocity)>,
) {

    let mut snowball_data = Vec::new();
    for (entity, transform, snowball, _) in snowball_query.iter() {
        snowball_data.push((
            entity,
            transform.translation,
            snowball.state == SnowballState::Rolling,
        ));
    }
    
    let mut collisions = Vec::new();
    for i in 0..snowball_data.len() {
        for j in i+1..snowball_data.len() {
            let (entity_a, pos_a, is_rolling_a) = snowball_data[i];
            let (entity_b, pos_b, is_rolling_b) = snowball_data[j];
            
            let distance = (pos_a.truncate() - pos_b.truncate()).length();
            
            if distance < 45.0 {
                collisions.push((entity_a, entity_b, is_rolling_a, is_rolling_b));
            }
        }
    }
    
    for (entity_a, entity_b, is_rolling_a, is_rolling_b) in collisions {
        if is_rolling_a && is_rolling_b {
            if let Ok([(_, _, _, mut vel_a), (_, _, _, mut vel_b)]) = snowball_query.get_many_mut([entity_a, entity_b]) {
                let temp = vel_a.value;
                vel_a.value = vel_b.value * 0.8;
                vel_b.value = temp * 0.8;
            }
        } else if is_rolling_a && !is_rolling_b {
            if let Ok([(_, transform_a, _, mut vel_a), (_, transform_b, mut snowball_b, mut vel_b)]) = 
                snowball_query.get_many_mut([entity_a, entity_b]) {
                let direction = (transform_b.translation - transform_a.translation).normalize().truncate();
                let speed = vel_a.value.length() * 0.7;
                
                snowball_b.state = SnowballState::Rolling;
                vel_b.value = direction * speed;
                vel_a.value *= 0.5;
            }
        } else if !is_rolling_a && is_rolling_b {
            if let Ok([(_, transform_a, mut snowball_a, mut vel_a), (_, transform_b, _, mut vel_b)]) = 
                snowball_query.get_many_mut([entity_a, entity_b]) {
                let direction = (transform_a.translation - transform_b.translation).normalize().truncate();
                let speed = vel_b.value.length() * 0.7;
                
                snowball_a.state = SnowballState::Rolling;
                vel_a.value = direction * speed;
                vel_b.value *= 0.5;
            }
        }
    }
}