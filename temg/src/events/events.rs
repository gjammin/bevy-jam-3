use bevy::prelude::{info, Commands, EventReader};
use naia_bevy_client::{
    events::{
        ClientTickEvent, ConnectEvent, DespawnEntityEvent, DisconnectEvent, InsertComponentEvents,
        MessageEvents, RejectEvent, RemoveComponentEvents, SpawnEntityEvent, UpdateComponentEvents,
    },
    sequence_greater_than, Client, CommandsExt, Random, Replicate, Tick,
};

use temg_common::{
    //components::{Color, ColorValue, Position, Shape, ShapeValue},
    //messages::{EntityAssignment, KeyCommand},
    EntityAssignmentChannel,
    PlayerCommandChannel,
};

pub fn connect_events(
    mut commands: Commands,
    mut client: Client,
    mut event_reader: EventReader<ConnectEvent>,
) {
    for _ in event_reader.iter() {
        let Ok(server_address) = client.server_address() else {
            panic!("Shouldn't happen");
        };
        info!("Client connected to: {}", server_address);

        // Create entity for Client-authoritative Cursor

        // // Position component
        // let position = {
        //     let x = 16 * ((Random::gen_range_u32(0, 40) as i16) - 20);
        //     let y = 16 * ((Random::gen_range_u32(0, 30) as i16) - 15);
        //     Position::new(x, y)
        // };

        // // Spawn Cursor Entity
        // let entity = commands
        //     // Spawn new Square Entity
        //     .spawn_empty()
        //     // MUST call this to begin replication
        //     .enable_replication(&mut client)
        //     // Insert Position component
        //     .insert(position)
        //     // Insert Cursor marker component
        //     .insert(LocalCursor)
        //     // return Entity id
        //     .id();

        // // Insert SpriteBundle locally only
        // commands.entity(entity).insert(MaterialMesh2dBundle {
        //     mesh: global.circle.clone().into(),
        //     material: global.white.clone(),
        //     transform: Transform::from_xyz(0.0, 0.0, 1.0),
        //     ..Default::default()
        // });

        // global.cursor_entity = Some(entity);
    }
}

pub fn reject_events(mut event_reader: EventReader<RejectEvent>) {
    for _ in event_reader.iter() {
        info!("Client rejected from connecting to Server");
    }
}

pub fn disconnect_events(mut event_reader: EventReader<DisconnectEvent>) {
    for _ in event_reader.iter() {
        info!("Client disconnected from Server");
    }
}
