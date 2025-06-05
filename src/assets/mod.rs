use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_hui::prelude::*;

use crate::screens::Screen;

mod tracking;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Title)
            .load_collection::<ImageAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/energy-ball.png")]
    pub energy_ball: Handle<Image>,
    #[asset(path = "images/fire-orb.png")]
    pub fire_orb: Handle<Image>,
    #[asset(path = "images/ice-orb.png")]
    pub ice_orb: Handle<Image>,
    #[asset(path = "images/lightning-orb.png")]
    pub lightning_orb: Handle<Image>,

    #[asset(path = "images/water-icon.png")]
    pub water_icon: Handle<Image>,

    #[asset(path = "templates/menu/pause.html")]
    pub pause_template: Handle<HtmlTemplate>,

    #[asset(path = "templates/menu/main.html")]
    pub main_menu: Handle<HtmlTemplate>,

    #[asset(path = "templates/menu/main_wasm.html")]
    pub main_wasm_menu: Handle<HtmlTemplate>,

    #[asset(path = "templates/components/button.html")]
    pub button_component: Handle<HtmlTemplate>,

    #[asset(path = "templates/settings.html")]
    pub settings: Handle<HtmlTemplate>,
}
