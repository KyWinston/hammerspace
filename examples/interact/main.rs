use bevy::{
    color::palettes::css::WHITE,
    image::{ImageAddressMode, ImageSamplerDescriptor},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
};

use bevy_third_person_camera::{
    ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom,
};
use hammerspace::{
    components::{Actor, Interactable, Player},
    resources::HammerspaceConfig,
    HammerspacePlugin,
};

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
            ThirdPersonCameraPlugin,
        ))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera3d::default(),
                ThirdPersonCamera {
                    zoom: Zoom::new(30.0, 60.0),
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
        .add_systems(
            Startup,
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut mats: ResMut<Assets<StandardMaterial>>| {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::from_length(2.0))),
                    MeshMaterial3d(mats.add(StandardMaterial::default())),
                    ThirdPersonCameraTarget,
                    Actor,
                    Player,
                    Transform::from_xyz(0.0, 5.0, 0.0),
                ));
                for loc in 1..15 {
                    commands.spawn((
                        Mesh3d(meshes.add(Cuboid::from_length(2.0))),
                        MeshMaterial3d(mats.add(StandardMaterial::default())),
                        Actor,
                        Interactable::new(format!("test-{}", loc)),
                        Transform::from_xyz(loc as f32 * 5.0, 5.0, loc as f32 * 5.0),
                    ));
                    commands.spawn((
                        Mesh3d(meshes.add(Cuboid::from_length(2.0))),
                        MeshMaterial3d(mats.add(StandardMaterial::default())),
                        Actor,
                        Interactable::new(format!("test-{}-mirror", loc)),
                        Transform::from_xyz(-loc as f32 * 5.0, 5.0, loc as f32 * 5.0),
                    ));
                }
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(150.0, 1.0, 150.0)))),
                    MeshMaterial3d(mats.add(StandardMaterial::default())),
                ));
            },
        )
        .add_systems(
            Update,
            |player_q: Query<(&Transform, &Interactable), With<Player>>,
             int_q: Query<(&Transform, &Interactable), Without<Player>>| {
                if let Ok((t, _i)) = player_q.get_single() {
                    println!(
                        "{:?}",
                        Actor::list_valid_interacts(t.translation, 30.0, int_q,true).len()
                    );
                }
            },
        )
        .run();
}
