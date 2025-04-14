use bevy::prelude::*;
use rand::prelude::*;
use crate::enemy;
use crate::snowball;

// Wave system resource to track the current wave and its state
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
            wave_timer: Timer::from_seconds(60.0, TimerMode::Once),   
            wave_break_timer: Timer::from_seconds(5.0, TimerMode::Once),
            spawning: false,
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            spawn_locations: Vec::new(),
            is_major_break: false,
        }
    }
}

// Get positions for spawning enemies based on the current wave
pub fn get_enemy_spawn_positions(wave: u32) -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    let mut positions = Vec::new();
    
    let base_count = 2 + wave as usize;
    let count = base_count.min(10); 
    let spawn_points = [
        (Vec2::new(-300.0, -120.0), 5),
        (Vec2::new(0.0, -60.0), 7),
        (Vec2::new(300.0, -125.0), 5),
        (Vec2::new(-225.0, 35.0), 6),
        (Vec2::new(225.0, 35.0), 6),
        (Vec2::new(0.0, 110.0), 8),
        (Vec2::new(-150.0, 150.0), 9),
        (Vec2::new(150.0, 150.0), 9),
    ];
    
    for _ in 0..count {
        let (base_pos, height_mod) = spawn_points[rng.gen_range(0..spawn_points.len())];
        
        let x_offset = rng.gen_range(-50.0..50.0);
        let position = Vec3::new(
            base_pos.x + x_offset,
            base_pos.y + height_mod as f32 * 10.0,
            1.0
        );
        
        positions.push(position);
    }
    
    positions
}

// System to manage wave progression
pub fn manage_wave_system(
    mut commands: Commands,
    time: Res<Time>,
    mut wave_system: ResMut<WaveSystem>,
    enemy_query: Query<Entity, With<enemy::Enemy>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let current_enemy_count = enemy_query.iter().count() as u32;
    wave_system.enemies_remaining = current_enemy_count;
    
    if wave_system.wave_in_progress {
        if current_enemy_count == 0 {
            wave_system.wave_in_progress = false;
            wave_system.between_waves_timer.reset();
            wave_system.current_wave += 1;
        }
    } else {
        wave_system.between_waves_timer.tick(time.delta());
        
        if wave_system.between_waves_timer.finished() {
            wave_system.wave_in_progress = true;
        
            let spawn_positions = get_enemy_spawn_positions(wave_system.current_wave);
            
            for position in spawn_positions {
                enemy::setup_enemy(commands.reborrow(), &asset_server, &mut images, position);
            }
        }
    }
}

#[derive(Component)]
pub struct WaveAnnouncement {
    pub timer: Timer,
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
        Vec2::new(-150.0, 150.0),
        Vec2::new(150.0, 150.0),
        Vec2::new(-250.0, -100.0),
        Vec2::new(250.0, -100.0),
        Vec2::new(0.0, -150.0),
    ];
    
    let wave_system = WaveSystem {
        spawn_locations,
        ..Default::default()
    };
    
    commands.insert_resource(wave_system);
}
// System to display wave announcements
pub fn display_wave_announcement(
    mut commands: Commands,
    wave_system: Res<WaveSystem>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Text, &mut WaveAnnouncement)>,
    asset_server: Res<AssetServer>,
) {
    let mut announcement_exists = false;
    
    for (entity, mut text, mut announcement) in query.iter_mut() {
        announcement_exists = true;
        announcement.timer.tick(time.delta());
        
        let alpha = 1.0 - (announcement.timer.elapsed_secs() / announcement.timer.duration().as_secs_f32());
        let mut color = text.sections[0].style.color;
        color.set_a(alpha);
        text.sections[0].style.color = color;
        
        if announcement.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
    
    if !announcement_exists && wave_system.wave_in_progress && wave_system.enemies_remaining > 0 {
        let font = asset_server.load("font.ttf");
        
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("Wave {}", wave_system.current_wave + 1),
                    TextStyle {
                        font,
                        font_size: 40.0,
                        color: Color::rgba(1.0, 0.5, 0.0, 1.0),
                    },
                ),
                transform: Transform::from_xyz(0.0, 200.0, 10.0),
                ..default()
            },
            WaveAnnouncement {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}

// Modified enemy setup to support different enemy types based on wave number
pub fn spawn_enemy_for_wave(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
    position: Vec3,
    wave: u32,
) {
    enemy::setup_enemy(commands.reborrow(), asset_server, images, position);
    
}