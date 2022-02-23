use bevy::prelude::*;
use input::InputPlugin;
use simulation::SimulationPlugin;
use ui::MainMenuPlugin;

const GRID_SIZE: i32 = 100;

mod ui;
mod input;
mod simulation;

pub fn game_of_life_app() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1024.0,
            height: 720.0,
            position: Default::default(),
            title: String::from("Game of Life"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(SimulationPlugin)
        .run();
}
