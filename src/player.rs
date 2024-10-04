use crate::{
    bullet::{Bullet, BulletBundle},
    Draw, DrawSystemSet, Plugin, RaylibContext, RaylibDrawContext, Startup, Transform, Update,
};
use bevy_ecs::prelude::*;
use raylib::{color::Color, consts::KeyboardKey, prelude::*};

pub const PLAYER_SIZE: u32 = 32;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(world: &mut bevy_ecs::world::World) {
        world.schedule_scope(Startup, |_, schedule| {
            schedule.add_systems(setup_player);
        });
        world.schedule_scope(Update, |_, schedule| {
            schedule.add_systems((move_player, fire_projectiles));
        });
        world.schedule_scope(Draw, |_, schedule| {
            schedule.add_systems(draw_player.in_set(DrawSystemSet::Draw));
        });
    }
}

#[derive(Component, Debug)]
pub struct Player {
    speed: f32,
}

fn setup_player(mut commands: Commands) {
    commands.spawn((Player { speed: 32.0 }, Transform::default()));
}

fn move_player(rl: Res<RaylibContext>, mut player_query: Query<(&mut Transform, &Player)>) {
    let input_x = (rl.is_key_down(KeyboardKey::KEY_D) as i32
        - rl.is_key_down(KeyboardKey::KEY_A) as i32) as f32;
    let input_y = (rl.is_key_down(KeyboardKey::KEY_S) as i32
        - rl.is_key_down(KeyboardKey::KEY_W) as i32) as f32;
    let delta = rl.get_frame_time();

    let (mut player_transform, player) = player_query.single_mut();
    player_transform.translation += Vector3::new(input_x, input_y, 0.0) * player.speed * delta;
}

fn draw_player(mut d: ResMut<RaylibDrawContext>, player_query: Query<&Transform, With<Player>>) {
    let player_transform = player_query.single();
    d.draw_rectangle(
        player_transform.translation.x as i32 - PLAYER_SIZE as i32 / 2,
        player_transform.translation.y as i32 - PLAYER_SIZE as i32 / 2,
        PLAYER_SIZE as i32,
        PLAYER_SIZE as i32,
        Color::RED,
    );
}

fn fire_projectiles(
    rl: Res<RaylibContext>,
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
) {
    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        let player_transform = player_query.single();
        let direction = (Vector3::new(rl.get_mouse_x() as f32, rl.get_mouse_y() as f32, 0.0)
            - player_transform.translation)
            .normalized();
        commands.spawn(BulletBundle {
            bullet: Bullet {
                direction,
                ..Default::default()
            },
            transform: player_transform.clone(),
        });
    }
}
