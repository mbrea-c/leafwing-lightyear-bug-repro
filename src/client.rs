//! The client plugin.
use crate::server::{SomeData, SERVER_ADDR};
use crate::shared::{self, TestActions};
use crate::shared::{shared_config, SharedPlugin};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
pub use lightyear::prelude::client::*;
use lightyear::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct ExampleClientPlugin;

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);

/// Here we create the lightyear [`ClientPlugins`]
fn build_client_plugin() -> ClientPlugins {
    // Authentication is where you specify how the client should connect to the server
    // This is where you provide the server address.
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 0,
        private_key: Key::default(),
        protocol_id: 0,
    };
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the client_address, because we open a UDP socket on the client
        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        auth,
        io,
        config: NetcodeConfig::default(),
    };
    let config = ClientConfig {
        // part of the config needs to be shared between the client and server
        shared: shared_config(),
        net: net_config,
        ..default()
    };
    ClientPlugins::new(config)
}

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        // add lightyear plugins
        app.add_plugins(build_client_plugin());
        // add our shared plugin containing the protocol + other shared behaviour
        app.add_plugins(SharedPlugin);

        // add our client-specific logic. Here we will just connect to the server
        app.add_systems(Startup, connect_client);
        // add our client-specific logic. Here we will just connect to the server
        app.add_systems(FixedUpdate, spawn_missing_action_states);
    }
}

/// Connect to the server
fn connect_client(mut commands: Commands) {
    commands.connect_client();
}

pub fn spawn_missing_action_states(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            Without<ActionState<TestActions>>,
            With<SomeData>,
            With<Predicted>,
        ),
    >,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert(ActionState::<TestActions>::default());
        commands
            .entity(entity)
            .insert(TestActions::default_input_map());
    }
}
