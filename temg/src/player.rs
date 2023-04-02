use crate::loading::TextureAssets;
use crate::GameState;
use crate::{actions::Actions, events::events::*};
use bevy::prelude::*;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, ReceiveEvents};
use temg_common::protocol;

use naia_bevy_client::{transport::webrtc, Client};
use temg_common::messages::Auth;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct Tick;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
        app.add_system(init.in_schedule(OnEnter(GameState::Playing)));

        // Add Naia Client Plugin
        app.add_plugin(ClientPlugin::new(ClientConfig::default(), protocol()));
        app.add_systems(
            (
                connect_events,
                disconnect_events,
                reject_events,
                // events::spawn_entity_events,
                // events::despawn_entity_events,
                // events::insert_component_events,
                // events::update_component_events,
                // events::remove_component_events,
                // events::message_events,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        .configure_set(Tick.after(ReceiveEvents));

        //.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
        //.add_system(move_player.in_set(OnUpdate(GameState::Playing)));
    }
}
pub fn init(
    mut commands: Commands,
    mut client: Client,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Naia Bevy Client Demo started");

    client.auth(Auth::new("charlie", "12345"));
    let socket = webrtc::Socket::new("http://127.0.0.1:14191", client.socket_config());
    client.connect(socket);

    info!("Connected to server");

    // Setup Camera
    //commands.spawn(Camera2dBundle::default());
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
//     commands
//         .spawn(SpriteBundle {
//             texture: textures.texture_bevy.clone(),
//             transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
//             ..Default::default()
//         })
//         .insert(Player);
// }

// fn move_player(
//     time: Res<Time>,
//     actions: Res<Actions>,
//     mut player_query: Query<&mut Transform, With<Player>>,
// ) {
//     if actions.player_movement.is_none() {
//         return;
//     }
//     let speed = 350.;
//     let movement = Vec3::new(
//         actions.player_movement.unwrap().x * speed * time.delta_seconds(),
//         actions.player_movement.unwrap().y * speed * time.delta_seconds(),
//         0.,
//     );
//     for mut player_transform in &mut player_query {
//         player_transform.translation += movement;
//     }
// }
