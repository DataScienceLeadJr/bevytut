use std::f32::consts::PI;

use bevy::{prelude::{Plugin, SystemSet, Commands, ResMut, Res, IntoSystem, SpriteBundle, Transform, Query, With, Entity}, core::{FixedTimestep, Time}, math::Vec3};
use rand::{thread_rng, Rng};

use crate::{ActiveEnemies, WinSize, Materials, Enemy, FromEnemy, Speed, Laser, SCALE, TIME_STEP, MAX_ENEMIES, MAX_FORMATION_MEMBERS};

// region:      Formation
// Component
#[derive(Default, Clone)]
struct Formation {
    start: (f32, f32),
    radius: (f32, f32),
    offset: (f32, f32),
    angle: f32,
    group_id: u32,
}

// Resource
#[derive(Default, Clone)]
struct FormationMaker {
    group_seq: u32,
    current_formation: Option<Formation>,
    current_formation_members: u32,
}

impl FormationMaker {
    fn make(&mut self, win_size: &WinSize) -> Formation {
        match (
            &self.current_formation,
            self.current_formation_members >= MAX_FORMATION_MEMBERS
        ) {
            // if first formation or previous formation full
            (None, _) | (_, true) => {
                // compute the start x,y
                let mut rng = thread_rng();
                let h_span = win_size.height / 2.0 - 100.0;
                let w_span = win_size.width / 4.0;
                let x = if rng.gen::<bool>() {
                    win_size.width
                } else {
                    -win_size.width
                };
                let y = rng.gen_range(-h_span..h_span) as f32;
                let start = (x, y);

                // compute offset and radius
                let offset = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));
                let radius = (rng.gen_range(80.0..150.0), 100.0);
                let angle: f32 = (y - offset.0).atan2(x - offset.1);

                // create new formation
                self.group_seq += 1;
                let group_id = self.group_seq;
                let formation = Formation {
                    start,
                    radius,
                    offset,
                    angle,
                    group_id,
                };

                // close, set, and return
                self.current_formation = Some(formation.clone());
                self.current_formation_members = 1;
                formation
            }
            // if still within the formation count
            (Some(formation_template), false) => {
                self.current_formation_members += 1;
                formation_template.clone()
            }
        }
    }
}
// endregion:   Formation

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .insert_resource(FormationMaker::default())
            .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(enemy_spawn.system()),
            )
            .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.85))
                .with_system(enemy_fire.system()),
            )
            .add_system(laser_movement.system())
            .add_system(enemy_movement.system());
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    mut formation_maker: ResMut<FormationMaker>,
    win_size: Res<WinSize>,
    materials: Res<Materials>,
) {
    if active_enemies.0 < MAX_ENEMIES {
        // get the formation and start x,y
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;

        // spawn enemy
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    scale: Vec3::new(SCALE, SCALE, SCALE),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Speed::default())
            .insert(formation);
        
        active_enemies.0 += 1;
    }
}

fn enemy_fire(
    mut commands: Commands,
    materials: Res<Materials>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &tf in enemy_query.iter() {
        let x = tf.translation.x;
        let y = tf.translation.y;

        // spawn enemy laser sprite
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15.0, 0.0),
                    scale: Vec3::new(SCALE, -SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(FromEnemy)
            .insert(Speed::default());
    }
}


fn laser_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform), (With<Laser>, With<FromEnemy>)>
) {
    for (laser_entity, speed, mut laser_tf) in query.iter_mut() {
        let translation = &mut laser_tf.translation;
        translation.y -= speed.0 * TIME_STEP;
        if translation.y < -win_size.height / 2.0 - 50.0 {
            commands.entity(laser_entity).despawn();
        }
    }
}

fn enemy_movement(
    mut query: Query<(&mut Transform, &Speed, &mut Formation), With<Enemy>>
) {
    for (mut transform, speed, mut formation) in query.iter_mut() {
        let max_distance = TIME_STEP * speed.0;
        let x_org = transform.translation.x;
        let y_org = transform.translation.y;

        // Get the ellipse shape
        let (x_offset, y_offset) = formation.offset;
        let (x_radius, y_radius) = formation.radius;

        // Compute the next angle
        let dir = if formation.start.0 > 0.0 { 1.0 } else { -1.0 };
        let angle = formation.angle + dir * speed.0 * TIME_STEP / (x_radius.min(y_radius) * PI / 2.0);

        // Calculate the destination
        let x_dst = x_radius * angle.cos() + x_offset;
        let y_dst = y_radius * angle.sin() + y_offset;


        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance == 0.0 {
            0.0
        } else {
            max_distance / distance
        };

        // Calculate the final x,y (make sure to not overshoot the intended ellipse path)
        let x = x_org - dx * distance_ratio;
        let x = if dx > 0.0 {
            x.max(x_dst)
        } else {
            x.min(x_dst)
        };

        let y = y_org - dy * distance_ratio;
        let y = if dy > 0.0 {
            y.max(y_dst)
        } else {
            y.min(y_dst)
        };

        // start rotating the formation anlge only when the sprite is on or close to the destination
        if distance < max_distance * speed.0 / 20.0 {
            formation.angle = angle;
        }

        // Apply the translation
        transform.translation.x = x;
        transform.translation.y = y;
    }
}