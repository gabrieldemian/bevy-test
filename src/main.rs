use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Gabriel".to_string())));
    commands.spawn((Person, Name("Vicente".to_string())));
}

fn update_name(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        name.0.push_str(" Lombardo");
    }
}

fn hello_person(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for q in &query {
            println!("hello {}", q.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GreetTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, (add_people, update_name).chain())
        .add_systems(Update, hello_person)
        .run();
}
