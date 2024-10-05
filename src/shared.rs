//! This module contains the shared code between the client and the server.
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::utils::Duration;

use client::{ComponentSyncMode, Predicted, Rollback};
use leafwing_input_manager::prelude::{ActionState, InputMap};
use leafwing_input_manager::{Actionlike, InputControlKind};
use lightyear::prelude::*;
use lightyear::shared::config::Mode;

use crate::server::SomeData;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;

pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

/// The [`SharedConfig`] must be shared between the `ClientConfig` and `ServerConfig`
pub fn shared_config() -> SharedConfig {
    SharedConfig {
        // send an update every 100ms
        server_replication_send_interval: SERVER_REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        },
        mode: Mode::Separate,
    }
}

#[derive(Clone)]
pub struct SharedPlugin;

#[derive(Channel)]
pub struct Channel1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Register your protocol, which is shared between client and server
        app.register_message::<Message1>(ChannelDirection::Bidirectional);
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
        app.add_plugins(LeafwingInputPlugin::<TestActions>::default());
        app.register_component::<SomeData>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        if app.is_plugin_added::<RenderPlugin>() {
            app.add_systems(Startup, init);
        }
        app.add_systems(FixedUpdate, check_test_action);
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// These actions are networked using lightyear's leafwing support
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum TestActions {
    Test,
}

impl Actionlike for TestActions {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            TestActions::Test => InputControlKind::Button,
        }
    }
}

impl TestActions {
    pub fn default_input_map() -> InputMap<Self> {
        use TestActions::*;
        let mut input_map = InputMap::default();

        input_map.insert(Test, MouseButton::Left);

        input_map
    }
}

fn check_test_action(
    q_actions: Query<&ActionState<TestActions>, Or<(With<Replicating>, With<Predicted>)>>,
    identity: NetworkIdentity,
    rollback: Option<Res<Rollback>>,
    tick: Res<TickManager>,
) {
    for actions in &q_actions {
        if actions.just_pressed(&TestActions::Test) {
            println!(
                "{}: Action pressed in tick {:?}",
                if identity.is_server() {
                    "SERVER"
                } else {
                    "CLIENT"
                },
                if let Some(rollback) = &rollback {
                    tick.tick_or_rollback_tick(rollback.as_ref())
                } else {
                    tick.tick()
                }
            );
        }
    }
}
