use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name;

fn hello_world() {
    println!("Hello bevy world");
}

fn main() {
    App::new().
        add_system(hello_world).
        run();
}
