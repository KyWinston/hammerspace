use bevy::prelude::*;
use bevy_quill_obsidian_inspect::{Precision, ValueRange};

use crate::TestEnum;

#[derive(Resource)]
pub struct PanelWidth(pub f32);

#[derive(Resource, Default)]
pub struct SelectedShape(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct ClickLog(pub Vec<String>);

#[derive(Resource)]
pub struct PreviewEntities {
    pub camera: Entity,
    pub _overlay: Entity,
}

#[derive(Resource, Debug, Reflect, Clone, Default)]
pub struct TestStruct {
    pub selected: bool,

    #[reflect(@ValueRange::<f32>(0.0..1.0))]
    pub scale: f32,

    pub color: Srgba,
    pub position: Vec3,
    pub unlit: Option<bool>,

    #[reflect(@ValueRange::<f32>(0.0..10.0))]
    pub roughness: Option<f32>,

    #[reflect(@Precision(2))]
    pub metalness: Option<f32>,

    #[reflect(@ValueRange::<f32>(0.0..1000.0))]
    pub factors: Vec<f32>,
}

#[derive(Resource, Debug, Reflect, Clone, Default)]
pub struct TestStruct2 {
    pub nested: TestStruct,
    pub choice: TestEnum,
}

#[derive(Resource, Debug, Reflect, Clone, Default)]
pub struct TestStruct3(pub bool);
