use bevy::{
    color::palettes::{css::WHITE, tailwind::BLUE_100},
    image::{ImageAddressMode, ImageSamplerDescriptor},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
};

use bevy_third_person_camera::{
    ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom,
};
use hammerspace::{
    interact::components::{Actor, Agent, Interactable},
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
                    Agent::new(),
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
            |mut player_q: Query<(&Transform, &Interactable, &mut Agent)>,
             int_q: Query<(&Transform, &Interactable), Without<Agent>>,
             mut gizmos: Gizmos| {
                if let Ok((t, _i, mut agent)) = player_q.get_single_mut() {
                    let list = Actor::list_valid_interacts(t.translation, 30.0, int_q, true);
                    let mut focus_ent = Vec3::ZERO;

                    if list.len() > 0 && agent.focused_idx.is_none() {
                        agent.focused_idx = Some(0);
                    } else if agent.focused_idx.is_some()
                        && list.len() > agent.focused_idx.unwrap()
                    {
                        focus_ent = list[agent.focused_idx.unwrap()].translation;
                    } else {
                        agent.focused_idx = None;
                    }
                    gizmos.arrow(
                        Vec3::new(focus_ent.x, focus_ent.y + 5.0, focus_ent.z),
                        Vec3::new(focus_ent.x, focus_ent.y + 2.5, focus_ent.z),
                        BLUE_100,
                    );
                }
            },
        )
        .run();
}
