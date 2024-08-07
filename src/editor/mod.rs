use bevy::prelude::*;

use components::{OutlinedBlock, Page, TextureAtlasInteraction, UiFooterRootNode};
use resources::CurrentPage;
use sickle_ui::{
    dev_panels::{
        hierarchy::HierarchyTreeViewPlugin,
        scene_view::{SceneViewPlugin, SpawnSceneViewPreUpdate},
    },
    prelude::*,
};

#[cfg(feature = "editor")]
use sickle_ui::SickleUiPlugin;
use systems::{clear_content_on_menu_change, despawn_hierarchy_view, exit_app_on_menu_item, handle_theme_contrast_select, handle_theme_data_update, handle_theme_switch, interaction_showcase, layout_showcase, setup, spawn_hierarchy_view, update_current_page};

pub struct EditorPlugin;

pub mod components;
pub mod resources;
pub mod systems;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "editor")]
        app.add_plugins(SickleUiPlugin)
            .add_plugins(UiFooterRootNodePlugin)
            .add_plugins(OutlinedBlockPlugin)
            .add_plugins(TextureAtlasInteractionPlugin)
            .init_resource::<CurrentPage>()
            .init_state::<Page>()
            .add_plugins(HierarchyTreeViewPlugin)
            .add_plugins(SceneViewPlugin)
            .add_systems(Startup, setup.in_set(UiStartupSet))
            .add_systems(OnEnter(Page::Layout), layout_showcase)
            .add_systems(OnExit(Page::Layout), clear_content_on_menu_change)
            .add_systems(OnEnter(Page::Playground), interaction_showcase)
            .add_systems(OnExit(Page::Playground), clear_content_on_menu_change)
            .add_systems(PreUpdate, exit_app_on_menu_item)
            .add_systems(
                PreUpdate,
                (spawn_hierarchy_view, despawn_hierarchy_view).after(SpawnSceneViewPreUpdate),
            )
            .add_systems(
                Update,
                (
                    update_current_page,
                    handle_theme_data_update,
                    handle_theme_switch,
                    handle_theme_contrast_select,
                )
                    .chain()
                    .after(WidgetLibraryUpdate),
            );
    }
}


#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub struct UiStartupSet;


// Example themed widgets, generated with snipped
pub struct UiFooterRootNodePlugin;

impl Plugin for UiFooterRootNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentThemePlugin::<UiFooterRootNode>::default());
    }
}

pub trait UiUiFooterRootNodeExt {
    fn ui_footer(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl UiUiFooterRootNodeExt for UiBuilder<'_, Entity> {
    fn ui_footer(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.container(
            (UiFooterRootNode::frame(), UiFooterRootNode),
            spawn_children,
        )
    }
}

pub struct OutlinedBlockPlugin;

impl Plugin for OutlinedBlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentThemePlugin::<OutlinedBlock>::default());
    }
}


pub trait UiOutlinedBlockExt {
    fn outlined_block(&mut self) -> UiBuilder<Entity>;
}

impl UiOutlinedBlockExt for UiBuilder<'_, Entity> {
    fn outlined_block(&mut self) -> UiBuilder<Entity> {
        self.spawn((OutlinedBlock::frame(), OutlinedBlock))
    }
}

pub struct TextureAtlasInteractionPlugin;

impl Plugin for TextureAtlasInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentThemePlugin::<TextureAtlasInteraction>::default());
    }
}

pub trait UiTextureAtlasInteractionExt {
    fn atlas_example(&mut self) -> UiBuilder<Entity>;
}

impl UiTextureAtlasInteractionExt for UiBuilder<'_, Entity> {
    fn atlas_example(&mut self) -> UiBuilder<Entity> {
        let mut result = self.spawn((TextureAtlasInteraction::frame(), TextureAtlasInteraction));
        result.style().image(ImageSource::Atlas(
            String::from("examples/Daisy.png"),
            TextureAtlasLayout::from_grid(UVec2::splat(128), 8, 4, None, None),
        ));

        result
    }
}
