use bevy::time::FixedTimestep;
use bevy::{prelude::*, winit::WinitSettings};

const OBJECT_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const ARENA_WIDTH: u32 = 200;
const ARENA_HEIGHT: u32 = 200;
const VELOCITY_X: f32 = 5.0;
const VELOCITY_Y: f32 = 10.0;
const ACCEL_Y: f32 = -0.1;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct ThrowingObject {
    tick: u32,
    v_x: f32,
    v_y: f32,
    accel_x: f32,
    accel_y: f32,
}

#[derive(PartialEq, Copy, Clone, Component)]
struct WinSize {
    width: f32,
    height: f32,
}

impl WinSize {
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
        .insert(WinSize::square(0.8));
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

fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: Default::default(),
                    //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
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
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_object)
        .add_startup_system(setup_button)
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
        .add_system(button_system)
        .run();
}
