//! The pause menu.
use crate::{assets::ImageAssets, menus::Menu, screens::Screen};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_hui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut html_funcs: HtmlFunctions,
) {
    html_funcs.register("open_settings_menu", open_settings_menu);
    html_funcs.register("close_menu", close_menu);
    html_funcs.register("quit_to_title", quit_to_title);

    commands.spawn((
        Name::new("Pause Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Pause),
        HtmlNode(image_assets.pause_template.clone()),
    ));
}

fn open_settings_menu(In(_): In<Entity>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(In(_): In<Entity>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(In(_): In<Entity>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
