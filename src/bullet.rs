use crate::{Draw, Plugin, RaylibContext, RaylibDrawContext, Transform, Update};
use bevy_ecs::prelude::*;
use raylib::prelude::*;

pub const BULLET_SIZE: u32 = 8;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(world: &mut World) {
        world.schedule_scope(Update, |_, schedule| {
            schedule.add_systems(move_bullets);
        });
        world.schedule_scope(Draw, |_, schedule| {
            schedule.add_systems(draw_bullets);
        });
    }
}

#[derive(Component, Debug)]
pub struct Bullet {
    pub speed: f32,
    pub direction: Vector3,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            speed: 256.0,
            direction: Vector3::default(),
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub transform: Transform,
}

fn move_bullets(
    rl: Res<RaylibContext>,
    mut bullet_query: Query<(Entity, &mut Transform, &Bullet)>,
    mut commands: Commands,
) {
    let delta = rl.get_frame_time();
    for (bullet_entity, mut bullet_transform, bullet) in bullet_query.iter_mut() {
        bullet_transform.translation += bullet.direction * bullet.speed * delta;
        let screen_rect = Rectangle::new(
            -((BULLET_SIZE / 2) as f32),
            -((BULLET_SIZE / 2) as f32),
            (rl.get_screen_width() + BULLET_SIZE as i32) as f32,
            (rl.get_screen_height() + BULLET_SIZE as i32) as f32,
        );
        if !screen_rect.check_collision_point_rec(Vector2::new(
            bullet_transform.translation.x,
            bullet_transform.translation.y,
        )) {
            println!("Despawning {:?}", bullet_entity);
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn draw_bullets(mut d: ResMut<RaylibDrawContext>, bullet_query: Query<&Transform, With<Bullet>>) {
    for bullet_transform in bullet_query.iter() {
        d.draw_rectangle(
            bullet_transform.translation.x as i32 - BULLET_SIZE as i32 / 2,
            bullet_transform.translation.y as i32 - BULLET_SIZE as i32 / 2,
            BULLET_SIZE as i32,
            BULLET_SIZE as i32,
            Color::YELLOW,
        );
    }
}
