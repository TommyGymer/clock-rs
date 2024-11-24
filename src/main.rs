use bevy::{
    core_pipeline::tonemapping::DebandDither, prelude::*, sprite::Anchor, window::PresentMode,
};
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

    let time_now = now.time();
    let hour: f32 = time_now.hour12().1 as f32;
    let minute: f32 = time_now.minute() as f32;
    let second: f32 = time_now.second() as f32;
    let nanosecond: f32 = time_now.nanosecond() as f32;

    for (mut transform, time_steps) in query.iter_mut() {
        match time_steps {
            TimeSteps::Hour => {
                transform.rotation = Quat::from_rotation_z(-TAU * (hour + minute / 60.) / 12.)
            }
            TimeSteps::Minute => {
                transform.rotation = Quat::from_rotation_z(-TAU * (minute + second / 60.) / 60.)
            }
            TimeSteps::Second => {
                transform.rotation =
                    Quat::from_rotation_z(-TAU * (second + nanosecond / 1000000000.) / 60.)
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
        deband_dither: DebandDither::Enabled,
        ..default()
    },));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "clock-rs".into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, add_hands))
        .add_systems(Update, update_hands)
        .run();
}
