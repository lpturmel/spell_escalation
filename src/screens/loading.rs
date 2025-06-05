//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use crate::screens::Screen;
use bevy::prelude::*;
use bevy_hui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);
}

fn spawn_loading_screen(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        HtmlNode(server.load("templates/loading.html")),
        StateScoped(Screen::Loading),
    ));
}
