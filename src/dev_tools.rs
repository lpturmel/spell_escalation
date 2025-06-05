//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::prelude::PhysicsGizmos;
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DebugState>();
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        (
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
            toggle_physics_debug.run_if(input_just_pressed(PHYSICS_DEBUG_KEY)),
        ),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;
const PHYSICS_DEBUG_KEY: KeyCode = KeyCode::F1;

#[derive(Resource, Default)]
struct DebugState {
    physics: bool,
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_physics_debug(mut state: ResMut<DebugState>, mut gcs: ResMut<GizmoConfigStore>) {
    let gc = gcs.config_mut::<PhysicsGizmos>().0;
    state.physics = !state.physics;
    gc.enabled = state.physics;
}
