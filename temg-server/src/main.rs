use bevy_app::{App, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_log::{info, LogPlugin};
use std::time::Duration;
use temg_common::protocol;

//use naia_bevy_demo_shared::protocol;
use naia_bevy_server::{Plugin as ServerPlugin, ReceiveEvents, ServerConfig};
use std::collections::HashMap;

use bevy_ecs::system::Commands;

use naia_bevy_server::{transport::webrtc, Server};

use crate::events::events::auth_events;

mod events;

//use systems::{events, init};

fn main() {
    info!("Naia Bevy Server Demo starting up");

    // Build App
    App::default()
        // Plugins
        .add_plugin(TaskPoolPlugin::default())
        .add_plugin(TypeRegistrationPlugin::default())
        .add_plugin(FrameCountPlugin::default())
        .insert_resource(
            // this is needed to avoid running the server at uncapped FPS
            ScheduleRunnerSettings::run_loop(Duration::from_millis(3)),
        )
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::new(ServerConfig::default(), protocol()))
        // Startup System
        .add_startup_system(init)
        // Receive Server Events
        .add_systems(
            (
                auth_events,
                // events::connect_events,
                // events::disconnect_events,
                // events::error_events,
                // events::tick_events,
                // events::spawn_entity_events,
                // events::despawn_entity_events,
                // events::insert_component_events,
                // events::update_component_events,
                // events::remove_component_events,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        // Run App
        .run();
}

pub fn init(mut commands: Commands, mut server: Server) {
    info!("Naia Bevy Server Demo is running");

    // Naia Server initialization
    let server_addresses = webrtc::ServerAddrs::new(
        "127.0.0.1:14191"
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        "127.0.0.1:14192"
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        "http://127.0.0.1:14192",
    );
    let socket = webrtc::Socket::new(&server_addresses, server.socket_config());
    server.listen(socket);

    // Create a new, singular room, which will contain Users and Entities that they
    // can receive updates from
    let main_room_key = server.make_room().key();

    // Resources
    // commands.insert_resource(Global {
    //     main_room_key,
    //     user_to_square_map: HashMap::new(),
    //     user_to_cursor_map: HashMap::new(),
    //     client_to_server_cursor_map: HashMap::new(),
    // })
}
