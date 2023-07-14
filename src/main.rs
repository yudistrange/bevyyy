use bevy::{
    ecs::system::Res,
    prelude::{
        App, Commands, Component, Plugin, PreStartup, Query, ResMut, Resource, Startup, Update,
        With,
    },
    time::{Time, Timer, TimerMode},
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

fn say_hello() {
    println!("Hello, world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, add_people)
            .insert_resource(GreetTimer(Timer::from_seconds(10.0, TimerMode::Repeating)))
            .add_systems(Startup, say_hello)
            .add_systems(Update, greet_people);
    }
}
