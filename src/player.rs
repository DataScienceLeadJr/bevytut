use bevy::{prelude::*, core::FixedTimestep};

use crate::{Player, PlayerReadyFire, Speed, TIME_STEP, Materials, Laser, WinSize, FromPlayer, SCALE, PlayerState, PLAYER_RESPAWN_DELAY};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(PlayerState::default())
        .add_startup_stage("game_setup_actors", SystemStage::single(player_spawn.system()))
        .add_system(player_movement.system())
        .add_system(player_fire.system())
        .add_system(laser_movement.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(player_spawn.system())
        );
    }
}

fn player_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    win_size: Res<WinSize>,
    time: Res<Time>,
    mut player_state: ResMut<PlayerState>
) {
    let now = time.seconds_since_startup();
    let last_shot = player_state. last_shot;

    // spawn a sprite
    if !player_state.on && (last_shot == 0.0 || now < last_shot + PLAYER_RESPAWN_DELAY) {
        let bottom = -win_size.height / 2.0;
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.player_materials.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, bottom + 75.0 / 4.0 + 5.0, 10.0),
                    scale: Vec3::new(SCALE, SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
        })
        .insert(Player)
        .insert(PlayerReadyFire(true))
        .insert(Speed::default());

        player_state.spawned();
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>
) {
    if let Ok((speed, mut transform, _player)) = query.single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };
        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(&Transform, &mut PlayerReadyFire), With<Player>>
) {
    if let Ok((player_tf, mut ready_fire)) = query.single_mut() {
        if ready_fire.0 && keyboard_input.pressed(KeyCode::Space) {
            let x = player_tf.translation.x;
            let y = player_tf.translation.y + 19.0; // manually defined "claw height" offset
            let mut spawn_laser = |x_offset: f32| {
                commands.spawn_bundle(SpriteBundle {
                    material: materials.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x + x_offset, y, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(FromPlayer)
                .insert(Speed::default());
                ready_fire.0 = false;
            };
            let x_offset = 144.0 / 4.0 - 5.0; // manually defined "claw width"
            spawn_laser(x_offset);
            spawn_laser(-x_offset);
        }
            

        if keyboard_input.just_released(KeyCode::Space) {
            ready_fire.0 = true;
        }
    }
}

fn laser_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform), (With<Laser>, With<FromPlayer>)>
) {
    for (laser_entity, speed, mut laser_tf) in query.iter_mut() {
        let translation = &mut laser_tf.translation;
        translation.y += speed.0 * TIME_STEP;
        if translation.y > win_size.height {
            commands.entity(laser_entity).despawn();
        }
    }
}