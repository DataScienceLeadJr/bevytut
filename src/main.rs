mod player;

use bevy::prelude::*;
use player::PlayerPlugin;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const TIME_STEP: f32 = 1.0 / 60.0;

// region:      Resources
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser: Handle<ColorMaterial>,
}

struct WinSize {
    #[allow(unused)]
    width: f32,
    height: f32,
}
// endregion:   Resources

// region:      Components
struct Player;
struct PlayerReadyFire(bool);
struct Laser;
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500.0)
    }
}
// endregion:   Components

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(PlayerPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create the main resources
    commands.insert_resource(WinSize {
        width: window.width(),
        height: window.height(),
    });
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser: materials.add(asset_server.load(LASER_SPRITE).into())
    });

    // position window
    window.set_position(IVec2::new(2140,420));

}
