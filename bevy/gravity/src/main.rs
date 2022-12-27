use bevy::prelude::*;
use bevy::time::FixedTimestep;

const OBJECT_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const ARENA_WIDTH: u32 = 200;
const ARENA_HEIGHT: u32 = 200;
const VELOCITY_X: f32 = 5.0;
const VELOCITY_Y: f32 = 10.0;
const ACCEL_Y: f32 = -0.1;

#[derive(Component)]
struct ThrowingObject {
    tick: u32,
    v_x: f32,
    v_y: f32,
    accel_x: f32,
    accel_y: f32,
}

#[derive(PartialEq, Copy, Clone, Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn size_scaling(windows: Res<Windows>, mut q: Query<(&WinSize, &mut Transform)>) {
    if let Some(window) = windows.get_primary() {
        for (sprite_size, mut transform) in q.iter_mut() {
            transform.scale = Vec3::new(
                sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
                sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
                1.0,
            );
        }
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    if let Some(window) = windows.get_primary() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
                0.0,
            );
        }
    }
}

fn spawn_object(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: OBJECT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(ThrowingObject {
            tick: 0,
            v_x: VELOCITY_X,
            v_y: VELOCITY_Y,
            accel_x: 0.0,
            accel_y: ACCEL_Y,
        })
        .insert(Position { x: 0, y: 0 })
        .insert(Size::square(0.8));
}

fn move_object(
    mut objects: Query<(Entity, &mut ThrowingObject)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((entity, mut object)) = objects.iter_mut().next() {
        let mut object_pos = positions.get_mut(entity).unwrap();
        if object_pos.y < 0 {
            object.tick = 0;
            object.v_x = VELOCITY_X;
            object.v_y = VELOCITY_Y;
            object_pos.x = 0;
            object_pos.y = 0;
            return;
        }
        object.tick += 1;
        object.v_x += object.accel_x * object.tick as f32;
        object.v_y += object.accel_y * object.tick as f32;
        object_pos.x += object.v_x as i32;
        object_pos.y += object.v_y as i32;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Gravity!".to_string(),
                width: 1000.0,
                height: 600.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_object)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.15))
                .with_system(move_object),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .run();
}
