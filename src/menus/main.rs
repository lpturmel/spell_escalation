//! The main menu (seen on the title screen).

use crate::{assets::ImageAssets, menus::Menu, screens::Screen};
use bevy::prelude::*;
use bevy_hui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(
    mut commands: Commands,
    mut html_comps: HtmlComponents,
    mut html_funcs: HtmlFunctions,
    image_assets: Res<ImageAssets>,
) {
    html_comps.register("menu_btn", image_assets.button_component.clone());
    html_funcs.register("play", play);
    html_funcs.register("settings", open_settings_menu);
    #[cfg(not(target_family = "wasm"))]
    html_funcs.register("quit", exit_app);

    commands.spawn((
        Name::new("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        HtmlNode(image_assets.main_menu.clone()),
        #[cfg(target_family = "wasm")]
        HtmlNode(image_assets.main_wasm_menu.clone()),
    ));
}

fn play(In(_entity): In<Entity>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn open_settings_menu(In(_entity): In<Entity>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(In(_entity): In<Entity>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
