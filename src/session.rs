//! Game session handling.
//!
//! A "session" in this context means either a local or networked game session, which for network
//! games will be synced with peers.

use bevy::ecs::system::SystemParam;
use bevy_ggrs::{
    ggrs::{self, SessionBuilder},
    SessionType,
};

use crate::{networking::proto::ClientMatchInfo, player, prelude::*, GgrsConfig};

pub struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameIdx>()
            .extend_rollback_plugin(|plugin| plugin.register_rollback_type::<FrameIdx>())
            .extend_rollback_schedule(|schedule| {
                schedule.add_system_to_stage(
                    RollbackStage::Last,
                    |mut frame_idx: ResMut<FrameIdx>| {
                        frame_idx.0 = frame_idx.0.wrapping_add(1);
                        trace!("End of simulation frame {}", frame_idx.0);
                    },
                );
            });
    }
}

/// The current game logic frame, as distict from a render frame, in the presence of rollback.
///
/// Primarily used for diagnostics.
#[derive(Reflect, Component, Default)]
#[reflect(Default)]
pub struct FrameIdx(pub u32);

#[derive(SystemParam)]
pub struct SessionManager<'w, 's> {
    commands: Commands<'w, 's>,
    client_match_info: Option<Res<'w, ClientMatchInfo>>,
}

impl<'w, 's> SessionManager<'w, 's> {
    /// Setup the game session
    pub fn start_session(&mut self) {
        if let Some(_info) = &self.client_match_info {
            todo!("Network session");
        } else {
            let mut builder = SessionBuilder::<GgrsConfig>::new();

            builder = builder
                .with_num_players(player::MAX_PLAYERS)
                .with_check_distance(7);

            for i in 0..player::MAX_PLAYERS {
                builder = builder.add_player(ggrs::PlayerType::Local, i).unwrap();
            }

            let session = builder.start_synctest_session().unwrap();
            self.commands.insert_resource(session);
            self.commands.insert_resource(SessionType::SyncTestSession);
        }
    }
}
