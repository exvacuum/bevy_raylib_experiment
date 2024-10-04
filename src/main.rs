use std::mem;

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use player::PlayerPlugin;
use raylib::prelude::*;

mod player;

pub mod schedule {
    use bevy_ecs::schedule::ScheduleLabel;

    #[derive(ScheduleLabel, Clone, PartialEq, Eq, Hash, Debug)]
    pub struct Startup;

    #[derive(ScheduleLabel, Clone, PartialEq, Eq, Hash, Debug)]
    pub struct Update;

    #[derive(ScheduleLabel, Clone, PartialEq, Eq, Hash, Debug)]
    pub struct Draw;
}
use schedule::*;

#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DrawSystemSet {
    Clear,
    Draw,
}

pub trait Plugin {
    fn build(world: &mut World);
}

fn main() {
    let mut world: World = World::new();

    let (rl, thread) = raylib::init().size(640, 480).title("Hello, World!").build();

    world.insert_resource(RaylibContext(rl));

    world.add_schedule(Schedule::new(Startup));
    world.add_schedule(Schedule::new(Update));
    world.add_schedule(Schedule::new(Draw));

    world.schedule_scope(Draw, |_, schedule| {
        schedule.add_systems(
            clear_background
                .in_set(DrawSystemSet::Clear)
                .before(DrawSystemSet::Draw),
        );
    });

    PlayerPlugin::build(&mut world);

    world.run_schedule(Startup);
    while !world
        .get_resource::<RaylibContext>()
        .unwrap()
        .window_should_close()
    {
        world.run_schedule(Update);

        let mut raylib_context = world.remove_resource::<RaylibContext>().unwrap();
        // SAFETY: Uhh I think it's ok...
        unsafe {
            let d = RaylibDrawContext(mem::transmute(
                raylib_context.begin_drawing(&thread.clone()),
            ));
            world.insert_resource(d);
        }
        world.run_schedule(Draw);
        world.remove_resource::<RaylibDrawContext>();
        world.insert_resource(raylib_context);
    }
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct RaylibContext(pub RaylibHandle);

#[derive(Resource, Deref, DerefMut)]
pub struct RaylibDrawContext(pub RaylibDrawHandle<'static>);

#[derive(Component, Default, Debug, Deref, DerefMut)]
struct Transform(pub math::Transform);

fn clear_background(mut d: ResMut<RaylibDrawContext>) {
    d.clear_background(Color::WHITE);
}
