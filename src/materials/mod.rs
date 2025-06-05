use self::orb::OrbMaterial;
use crate::{assets::ImageAssets, player::Element, PausableSystems};
use bevy::{prelude::*, sprite::Material2dPlugin};

pub mod orb;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_orb_infusion_added);
    app.add_plugins(Material2dPlugin::<OrbMaterial>::default());
    app.add_systems(PostUpdate, on_orb_infusion_changed.in_set(PausableSystems));
}

pub const FIRE_COLOR: Color = Color::srgb_u8(207, 87, 60);
pub const LIGHTNING_COLOR: Color = Color::srgb_u8(222, 158, 65);
pub const ICE_COLOR: Color = Color::srgb_u8(104, 222, 240);

#[derive(Component, Default)]
pub struct OrbInfusion(Vec<Element>);

impl OrbInfusion {
    pub fn is_full(&self) -> bool {
        self.0.len() == 2
    }
    pub fn add(&mut self, element: Element) {
        if self.is_full() {
            return;
        }
        self.0.push(element);
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn first(&self) -> Option<Element> {
        self.0.first().copied()
    }
    pub fn last(&self) -> Option<Element> {
        self.0.last().copied()
    }
}

fn on_orb_infusion_added(
    trigger: Trigger<OnAdd, OrbInfusion>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OrbMaterial>>,
    image_assets: Res<ImageAssets>,
    images: Res<Assets<Image>>,
) {
    let target = trigger.target();
    if target == Entity::PLACEHOLDER {
        error!("Orb infusion was added without a target");
        return;
    }
    let Some(orb_image) = images.get(&image_assets.energy_ball) else {
        error!("Couldn't find orb image");
        return;
    };
    let mesh = meshes.add(Mesh::from(Rectangle::new(
        orb_image.size().x as f32,
        orb_image.size().y as f32,
    )));

    commands.entity(target).insert((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(OrbMaterial {
            color_a: Color::WHITE.to_linear(),
            color_b: Color::BLACK.to_linear(),
            texture: image_assets.energy_ball.clone(),
        })),
    ));
}

fn on_orb_infusion_changed(
    query: Query<(&OrbInfusion, &mut MeshMaterial2d<OrbMaterial>), Changed<OrbInfusion>>,
    mut materials: ResMut<Assets<OrbMaterial>>,
) {
    for (infusion, material) in query.iter() {
        if infusion.is_empty() {
            continue;
        }

        let color_a = infusion.first();
        let color_b = infusion.last();

        let Some(mat) = materials.get_mut(&material.0) else {
            error!("Couldn't find orb material");
            continue;
        };

        if let Some(ele_a) = color_a {
            mat.color_a = ele_a.color().to_linear();
        }
        if let Some(ele_b) = color_b {
            mat.color_b = ele_b.color().to_linear();
        }
    }
}
