use crate::{Draw, DrawSystemSet, Plugin, RaylibContext, RaylibDrawContext, Startup, Transform, Update};
use bevy_ecs::prelude::*;
use raylib::{color::Color, consts::KeyboardKey, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(world: &mut bevy_ecs::world::World) {
        world.schedule_scope(Startup, |_, schedule| {
            schedule.add_systems(setup_player);
        });
        world.schedule_scope(Update, |_, schedule| {
            schedule.add_systems(move_player);
        });
        world.schedule_scope(Draw, |_, schedule| {
            schedule.add_systems(draw_player.in_set(DrawSystemSet::Draw));
        });
    }
}

#[derive(Component, Debug)]
pub struct Player;

fn setup_player(mut commands: Commands) {
    commands.spawn((Player, Transform::default()));
}

fn move_player(rl: Res<RaylibContext>, mut player_query: Query<&mut Transform, With<Player>>) {
    let input_x = (rl.is_key_down(KeyboardKey::KEY_D) as i32
        - rl.is_key_down(KeyboardKey::KEY_A) as i32) as f32;
    let delta = rl.get_frame_time();

    let mut player_transform = player_query.single_mut();
    player_transform.translation.x += input_x * 32.0 * delta;
}

fn draw_player(mut d: ResMut<RaylibDrawContext>, player_query: Query<&Transform, With<Player>>) {
    let player_transform = player_query.single();
    d.draw_rectangle(
        player_transform.translation.x as i32,
        player_transform.translation.y as i32,
        32,
        32,
        Color::RED,
    );
}
