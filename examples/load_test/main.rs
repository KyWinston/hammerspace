use bevy::{
    color::palettes::css::WHITE,
    image::{ImageAddressMode, ImageSamplerDescriptor},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraPlugin, Zoom};
use hammerspace::{
    HammerspacePlugin, assembler::events::PrepareLevelEvent, resources::HammerspaceConfig,
};

///test of the level and prefab load functionality
fn main() {
    let rpt = ImageAddressMode::Repeat;
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        address_mode_u: rpt,
                        address_mode_v: rpt,
                        address_mode_w: rpt,
                        ..default()
                    },
                })
                .build(),
        )
        .add_plugins((
            HammerspacePlugin {
                config: HammerspaceConfig::new("levels".to_string()),
            },
            WorldInspectorPlugin::default(),
            ThirdPersonCameraPlugin,
        ))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera3d::default(),
                ThirdPersonCamera {
                    zoom: Zoom::new(15.0, 60.0),
                    ..default()
                },
            ));

            commands.spawn((
                DirectionalLight {
                    illuminance: light_consts::lux::OVERCAST_DAY,
                    ..default()
                },
                CascadeShadowConfigBuilder {
                    first_cascade_far_bound: 4.0,
                    maximum_distance: 10.0,
                    ..default()
                }
                .build(),
                Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
            commands.insert_resource(AmbientLight {
                brightness: 100.0,
                color: WHITE.into(),
            });
        })
        .add_systems(Startup, |mut level: EventWriter<PrepareLevelEvent>| {
            level.send(PrepareLevelEvent("test_load".to_string()));
        })
        .run();
}
