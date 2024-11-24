use bevy::prelude::*;
use bevy::sprite::Anchor;
use chrono::{Local, Timelike};
use std::f32::consts::TAU;

#[derive(Component, Debug)]
struct Hand;

#[derive(Component, Debug)]
enum TimeSteps {
    Hour,
    Minute,
    Second,
}

fn add_hands(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Hand,
        TimeSteps::Hour,
        SpriteBundle {
            texture: asset_server.load("hand_hour.png"),
            transform: Transform::from_scale(Vec3 {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            })
            .with_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            }),
            sprite: Sprite {
                anchor: Anchor::BottomCenter,
                ..default()
            },
            ..default()
        },
    ));
    commands.spawn((
        Hand,
        TimeSteps::Minute,
        SpriteBundle {
            texture: asset_server.load("hand_minute.png"),
            transform: Transform::from_scale(Vec3 {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            })
            .with_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: 2.,
            }),
            sprite: Sprite {
                anchor: Anchor::BottomCenter,
                ..default()
            },
            ..default()
        },
    ));
    commands.spawn((
        Hand,
        TimeSteps::Second,
        SpriteBundle {
            texture: asset_server.load("hand_second.png"),
            transform: Transform::from_scale(Vec3 {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            })
            .with_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: 3.,
            }),
            sprite: Sprite {
                anchor: Anchor::BottomCenter,
                ..default()
            },
            ..default()
        },
    ));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("dial.png"),
        transform: Transform::from_scale(Vec3 {
            x: 0.25,
            y: 0.25,
            z: 0.25,
        })
        .with_translation(Vec3 {
            x: 0.,
            y: 0.,
            z: -1.,
        }),
        ..default()
    });
}

fn update_hands(mut query: Query<(&mut Transform, &TimeSteps), With<Hand>>) {
    let now = Local::now();
    for (mut transform, time_steps) in query.iter_mut() {
        match time_steps {
            TimeSteps::Hour => {
                transform.rotation = Quat::from_rotation_z(
                    -TAU * (now.time().hour12().1 as f32 + now.time().minute() as f32 / 60.) / 12.,
                )
            }
            TimeSteps::Minute => {
                transform.rotation = Quat::from_rotation_z(
                    -TAU * (now.time().minute() as f32 + now.time().second() as f32 / 60.) / 60.,
                )
            }
            TimeSteps::Second => {
                transform.rotation = Quat::from_rotation_z(
                    -TAU * (now.time().second() as f32
                        + now.time().nanosecond() as f32 / 1000000000.)
                        / 60.,
                )
            }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    },));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "clock-rs".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, add_hands))
        .add_systems(Update, update_hands)
        .run();
}
