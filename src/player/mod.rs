use std::time::Duration;

use crate::{
    assets::ImageAssets,
    gameplay::DespawnTimer,
    materials::{OrbInfusion, FIRE_COLOR, ICE_COLOR, LIGHTNING_COLOR},
    screens::Screen,
    AppSystems, PausableSystems,
};
use avian2d::prelude::*;
use bevy::{platform::collections::HashSet, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ChargeTimer>();
    app.insert_resource(ChargeTimer(Timer::from_seconds(0.0, TimerMode::Once)))
        .add_systems(OnEnter(Screen::Gameplay), spawn_player);

    app.add_systems(
        Update,
        (launch_orb.in_set(AppSystems::RecordInput))
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        RigidBody::Static,
        Player,
    ));
}

const SPAWN_ORB_KEY: MouseButton = MouseButton::Right;

#[derive(Component, Default)]
struct Orb;

#[derive(Component, Default)]
struct Charging;

#[derive(Component, Default)]
struct Player;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Element {
    Fire,
    Ice,
    Lightning,
}

impl Element {
    pub fn color(&self) -> Color {
        match self {
            Element::Fire => FIRE_COLOR,
            Element::Ice => ICE_COLOR,
            Element::Lightning => LIGHTNING_COLOR,
        }
    }
}

#[derive(Resource, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
struct ChargeTimer(pub Timer);

impl ChargeTimer {
    fn tier_scale(&self) -> f32 {
        let percent = self.0.elapsed_secs() / self.0.duration().as_secs_f32();

        match percent {
            p if p < 0.25 => 1.0,
            p if p < 0.5 => 1.25,
            p if p < 0.75 => 1.5,
            _ => 2.0,
        }
    }
}

fn launch_orb(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    keyborad: Res<ButtonInput<KeyCode>>,
    mut charging_orbs: Query<
        (
            Entity,
            &mut LinearVelocity,
            &mut OrbInfusion,
            &mut Transform,
        ),
        (With<Charging>, With<Orb>),
    >,
    mut charge_timer: ResMut<ChargeTimer>,
    time: Res<Time>,
) {
    let any_charging_orbs = !charging_orbs.is_empty();

    if any_charging_orbs {
        let Ok((orb_ent, mut orb_velocity, mut infusion, mut orb_tf)) = charging_orbs.single_mut()
        else {
            return;
        };
        if keyborad.just_pressed(KeyCode::Digit1) {
            // orb_sprite.image = assets.fire_orb.clone();
            infusion.add(Element::Fire);
        }
        if keyborad.just_pressed(KeyCode::Digit2) {
            // orb_sprite.image = assets.ice_orb.clone();
            infusion.add(Element::Ice);
        }
        if keyborad.just_pressed(KeyCode::Digit3) {
            // orb_sprite.image = assets.lightning_orb.clone();
            infusion.add(Element::Lightning);
        }

        if mouse.pressed(SPAWN_ORB_KEY) {
            charge_timer.tick(time.delta());

            let new_scale = charge_timer.tier_scale();
            if (orb_tf.scale.x - new_scale).abs() > f32::EPSILON {
                orb_tf.scale = Vec3::splat(new_scale);
            }
        }
        if mouse.just_released(SPAWN_ORB_KEY) {
            if infusion.is_empty() {
                commands.entity(orb_ent).despawn();
                charge_timer.reset();
                return;
            }

            let scale = charge_timer.tier_scale();
            info!("Scale modifier because of timer: {:?}", scale);

            let base_velocity = Vec2::new(0., 1.) * 300.;

            **orb_velocity = base_velocity / scale;

            commands.entity(orb_ent).remove::<Charging>();
            commands
                .entity(orb_ent)
                .insert(DespawnTimer(Timer::from_seconds(5., TimerMode::Once)));

            charge_timer.reset();
        }

        return;
    }

    if mouse.just_pressed(SPAWN_ORB_KEY) {
        charge_timer.set_duration(Duration::from_secs(2));
        commands.spawn((
            OrbInfusion::default(),
            Name::new("Orb"),
            Transform::default(),
            Visibility::default(),
            // Sprite {
            //     image: assets.energy_ball.clone(),
            //     ..default()
            // },
            Orb,
            LinearVelocity::default(),
            Collider::circle(16.0),
            Charging,
            RigidBody::Kinematic,
            StateScoped(Screen::Gameplay),
        ));
    }
}
