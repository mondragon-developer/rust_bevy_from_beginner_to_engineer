use bevy::prelude::*;

mod bridge;
mod components;
mod constants;
mod court;
mod feedback;
mod physics;
mod resources;
mod session;
mod shooting;

/// The frame pipeline, named: state flows in, input acts, the world
/// simulates, state flows out. Plugins hang their systems on these.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    SyncIn,
    Input,
    Physics,
    SyncOut,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .configure_sets(
            Update,
            (
                GameSet::SyncIn,
                GameSet::Input,
                GameSet::Physics,
                GameSet::SyncOut,
            )
                .chain(),
        )
        .add_plugins((
            court::CourtPlugin,
            shooting::ShootingPlugin,
            physics::PhysicsPlugin,
            feedback::FeedbackPlugin,
            session::SessionPlugin,
        ))
        .run();
}
