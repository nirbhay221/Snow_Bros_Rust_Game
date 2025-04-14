use bevy::prelude::*;
use crate::player::Player;
use crate::enemy::WaveSystem;

#[derive(Component)]
pub struct WaveDisplay;

#[derive(Component)]
pub struct LivesDisplay;
#[derive(Component)]
pub struct Announcement {
    pub timer: Timer,
}

#[derive(Component)]
pub struct BreakTimerDisplay;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Wave: 1/100",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(300.0, 250.0, 10.0),
            ..default()
        },
        WaveDisplay,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Lives: 3",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(-300.0, 250.0, 10.0),
            ..default()
        },
        LivesDisplay,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font_size: 32.0,
                    color: Color::rgba(1.0, 1.0, 0.0, 0.0), 
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, 50.0, 10.0),
            ..default()
        },
        BreakTimerDisplay,
    ));
}

pub fn update_wave_display(
    wave_system: Res<WaveSystem>,
    mut query: Query<&mut Text, With<WaveDisplay>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Wave: {}/{}", wave_system.current_wave + 1, wave_system.max_waves);
        
        let wave_progress = wave_system.current_wave as f32 / wave_system.max_waves as f32;
        
        if wave_progress > 0.8 {
            text.sections[0].style.color = Color::RED; 
        } else if wave_progress > 0.6 {
            text.sections[0].style.color = Color::rgb(1.0, 0.5, 0.0); 
        } else if wave_progress > 0.4 {
            text.sections[0].style.color = Color::YELLOW; 
        } else if wave_progress > 0.2 {
            text.sections[0].style.color = Color::GREEN; 
        } else {
            text.sections[0].style.color = Color::WHITE; 
        }
    }
}

pub fn update_lives_display(
    player_query: Query<&Player>,
    mut query: Query<&mut Text, With<LivesDisplay>>,
) {
    for player in player_query.iter() {
        if let Ok(mut text) = query.get_single_mut() {
            text.sections[0].value = format!("Lives: {}", player.lives);
            
            // Change color based on lives remaining
            if player.lives == 1 {
                text.sections[0].style.color = Color::RED; // Critical
            } else if player.lives == 2 {
                text.sections[0].style.color = Color::YELLOW; // Warning
            } else {
                text.sections[0].style.color = Color::WHITE; // Normal
            }
        }
    }
    
    if player_query.is_empty() {
        if let Ok(mut text) = query.get_single_mut() {
            let seconds = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f32();
            
            let flash = (seconds * 3.0).sin() * 0.5 + 0.5;
            text.sections[0].style.color = Color::rgb(1.0, flash * 0.5, flash * 0.5);
        }
    }
}

pub fn update_break_timer_display(
    wave_system: Res<WaveSystem>,
    time: Res<Time>,
    mut query: Query<&mut Text, With<BreakTimerDisplay>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = String::new();
        text.sections[0].style.color = Color::rgba(1.0, 1.0, 1.0, 0.0); 
    }
}
// Wave announcement system with improved visuals
pub fn display_wave_announcement(
    mut commands: Commands,
    wave_system: Res<WaveSystem>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Text, &mut Announcement)>,
) {
    let mut announcement_exists = false;
    
    for (entity, mut text, mut announcement) in query.iter_mut() {
        announcement_exists = true;
        announcement.timer.tick(time.delta());
        let alpha = 1.0 - (announcement.timer.elapsed_secs() / announcement.timer.duration().as_secs_f32());
        let mut color = text.sections[0].style.color;
        color.set_a(alpha);
        text.sections[0].style.color = color;
        
        let scale_factor = 1.0 + (alpha * 0.5);
        text.sections[0].style.font_size = 48.0 + (20.0 * alpha);
        
        if announcement.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
    
    // Create new announcement when a wave starts
    if !announcement_exists && wave_system.spawning && 
       wave_system.wave_timer.elapsed_secs() < 1.0 {
        let wave_progress = wave_system.current_wave as f32 / wave_system.max_waves as f32;
        let wave_color = if wave_progress > 0.8 {
            Color::rgba(1.0, 0.2, 0.2, 1.0) // Red for late waves
        } else if wave_progress > 0.5 {
            Color::rgba(1.0, 0.6, 0.0, 1.0) // Orange for mid waves
        } else {
            Color::rgba(0.2, 1.0, 0.2, 1.0) // Green for early waves
        };
        
        let is_tenth_wave = (wave_system.current_wave + 1) % 10 == 0;
        let display_text = if is_tenth_wave {
            format!("!!! MAJOR WAVE {} !!!", wave_system.current_wave + 1)
        } else {
            format!("WAVE {}", wave_system.current_wave + 1)
        };
        
        let font_size = if is_tenth_wave { 72.0 } else { 64.0 };
        
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    display_text,
                    TextStyle {
                        font_size,
                        color: wave_color,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            },
            Announcement {
                timer: Timer::from_seconds(3.0, TimerMode::Once),
            },
        ));
    }
}