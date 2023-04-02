use naia_bevy_shared::{
    Channel, ChannelDirection, ChannelMode, Protocol, ProtocolPlugin, ReliableSettings,
    TickBufferSettings,
};
use std::time::Duration;

use naia_bevy_shared::LinkConditionerConfig;

use crate::messages::MessagesPlugin;

pub mod messages;

// Protocol Build
pub fn protocol() -> Protocol {
    Protocol::builder()
        // Config
        .tick_interval(Duration::from_millis(40))
        .link_condition(LinkConditionerConfig::good_condition())
        .enable_client_authoritative_entities()
        // Channels
        .add_plugin(ChannelsPlugin)
        // Messages
        .add_plugin(MessagesPlugin)
        // Components
        //.add_plugin(ComponentsPlugin)
        // Build Protocol
        .build()
}

#[derive(Channel)]
pub struct PlayerCommandChannel;

#[derive(Channel)]
pub struct EntityAssignmentChannel;

// Plugin
pub struct ChannelsPlugin;

impl ProtocolPlugin for ChannelsPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol
            .add_channel::<PlayerCommandChannel>(
                ChannelDirection::ClientToServer,
                ChannelMode::TickBuffered(TickBufferSettings::default()),
            )
            .add_channel::<EntityAssignmentChannel>(
                ChannelDirection::ServerToClient,
                ChannelMode::UnorderedReliable(ReliableSettings::default()),
            );
    }
}
