use bevy::prelude::*;

use crate::{screens::Screen, AppSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        despawn_if_timer_finished
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::TickTimers),
    );
}

#[derive(Component, Deref, DerefMut)]
pub struct DespawnTimer(pub Timer);

fn despawn_if_timer_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut despawn_timer: Query<(Entity, &mut DespawnTimer)>,
) {
    for (ent, mut timer) in despawn_timer.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            commands.entity(ent).despawn();
        }
    }
}
