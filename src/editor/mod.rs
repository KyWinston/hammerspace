use std::sync::Arc;

use bevy::prelude::*;

use bevy_mod_picking::debug::DebugPickingMode;
use bevy_mod_picking::focus::HoverMap;
use bevy_mod_picking::{backends::raycast::RaycastBackendSettings, DefaultPickingPlugins};
use bevy_mod_stylebuilder::StyleBuilderLayout;
use bevy_quill::*;
use bevy_quill_obsidian::*;
use bevy_quill_obsidian_inspect::{InspectableResource, Inspector, InspectorPlugin};
use controls::{
    Button, ButtonVariant, Checkbox, Dialog, DialogBody, DialogFooter, DialogHeader, ListView,
    Slider, Splitter, SplitterDirection, ToolButton, ToolPalette,
};
use focus::TabGroup;

use resources::{ClickLog, PanelWidth, SelectedShape, TestStruct, TestStruct2, TestStruct3};
use styles::{
    style_aside, style_button_flex, style_button_row, style_column_group, style_main,
    style_scroll_area, style_slider, wrapper_style,
};
use systems::{
    close_on_esc, enter_preview_mode, exit_preview_mode, rotate, setup, setup_ui, start_editor,
};

pub mod components;
pub mod resources;
pub mod styles;
pub mod systems;

#[derive(Debug, Reflect, Clone, Default)]
pub enum TestEnum {
    #[default]
    Unit,
    Float(f32),
    Color(Srgba),
    Struct {
        position: Vec3,
        color: Srgba,
    },
}

#[derive(Clone)]
pub struct ResourcePropertyInspector<T: Resource> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Resource> PartialEq for ResourcePropertyInspector<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: Resource> ResourcePropertyInspector<T> {
    pub fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: Resource + Reflect> ViewTemplate for ResourcePropertyInspector<T> {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        Inspector::new(Arc::<InspectableResource<T>>::default())
    }
}

#[derive(Clone, PartialEq)]
struct CenterPanel;

impl ViewTemplate for CenterPanel {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .children((NodeGraphDemo {},))
            .style(wrapper_style)
    }
}

const X_EXTENT: f32 = 14.5;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum EditorState {
    #[default]
    Preview,
    Graph,
    Split,
}

#[derive(Clone, PartialEq)]
pub struct LogList;

impl ViewTemplate for LogList {
    type View = impl View;
    fn create(&self, cx: &mut Cx) -> Self::View {
        let log = cx.use_resource::<ClickLog>();
        ListView::new()
            .children(For::each(log.0.clone(), |msg| msg.clone()))
            .style(style_scroll_area)
    }
}

#[derive(Clone, PartialEq)]
struct NodeGraphDemo;

impl ViewTemplate for NodeGraphDemo {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        // ()
    }
}
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
            #[cfg(feature="editor")]
            app.init_resource::<HoverMap>()
                .insert_resource(RaycastBackendSettings {
                    require_markers: true,
                    ..default()
                })
                .init_resource::<SelectedShape>()
                .init_resource::<TrackingScopeTracing>()
                .init_resource::<ClickLog>()
                .insert_resource(TestStruct {
                    unlit: Some(true),
                    ..default()
                })
                .insert_resource(TestStruct2 {
                    nested: TestStruct::default(),
                    ..default()
                })
                .insert_resource(TestStruct3(true))
                .insert_resource(PanelWidth(200.))
                .init_resource::<viewport::ViewportInset>()
                .add_plugins((
                    InspectorPlugin,
                    DefaultPlugins.set(ImagePlugin::default_nearest()),
                    DefaultPickingPlugins,
                    HammerspacePlugin {
                        level_folder: "levels".to_string(),
                    },
                    QuillPlugin,
                    ObsidianUiPlugin,
                ))
                .insert_resource(DebugPickingMode::Disabled)
                .init_state::<EditorState>()
                .add_event::<PathEvent>()
                .add_systems(Startup, (setup, setup_ui.pipe(start_editor)))
                .add_systems(
                    Update,
                    (
                        close_on_esc,
                        rotate.run_if(in_state(EditorState::Preview)),
                        rotate.run_if(in_state(EditorState::Split)),
                        viewport::update_viewport_inset.run_if(in_state(EditorState::Preview)),
                        viewport::update_viewport_inset.run_if(in_state(EditorState::Split)),
                        viewport::update_camera_viewport.run_if(in_state(EditorState::Preview)),
                        viewport::update_camera_viewport.run_if(in_state(EditorState::Split)),
                    ),
                )
                .add_systems(OnEnter(EditorState::Preview), enter_preview_mode)
                .add_systems(OnExit(EditorState::Preview), exit_preview_mode)
                .add_systems(OnEnter(EditorState::Split), enter_preview_mode)
                .add_systems(OnExit(EditorState::Split), exit_preview_mode);
        }
    }

/// A view template
#[derive(Clone, PartialEq)]
pub struct MainDock(Entity);

impl ViewTemplate for MainDock {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        // Access data in a resource
        let dialog_open = cx.create_mutable(false);
        let checked_1 = cx.create_mutable(false);
        let checked_2 = cx.create_mutable(true);
        let red = cx.create_mutable::<f32>(128.);
        let panel_width = cx.use_resource::<PanelWidth>().0;
        let camera = self.0;

        cx.insert(TargetCamera(camera));
        Element::<NodeBundle>::new()
            .named("Main")
            .style((typography::text_default, style_main))
            .insert_dyn(move |_| (TabGroup::default(), TargetCamera(camera)), ())
            .children((
                Dialog::new()
                    .width(Val::Px(400.))
                    .open(dialog_open.get(cx))
                    .on_close(cx.create_callback(move |world: &mut World| {
                        dialog_open.set(world, false);
                    }))
                    .children((
                        DialogHeader::new().children("Dialog Header"),
                        DialogBody::new().children("Example dialog body text."),
                        DialogFooter::new().children((
                            Button::new()
                                .children("Cancel")
                                .on_click(cx.create_callback(move |world: &mut World| {
                                    dialog_open.set(world, false);
                                })),
                            Button::new()
                                .children("Close")
                                .variant(ButtonVariant::Primary)
                                .autofocus(true)
                                .on_click(cx.create_callback(move |world: &mut World| {
                                    dialog_open.set(world, false);
                                })),
                        )),
                    )),
                Element::<NodeBundle>::new()
                    .named("ControlPalette")
                    .style(style_aside)
                    .style_dyn(
                        move |width, sb| {
                            sb.width(Val::Px(width));
                        },
                        panel_width,
                    )
                    .children((
                        ToolPalette::new().columns(3).children((
                            ToolButton::new()
                                .children("Preview")
                                .corners(RoundedCorners::Left)
                                .variant({
                                    let st = cx.use_resource::<State<EditorState>>();
                                    if *st.get() == EditorState::Preview {
                                        ButtonVariant::Selected
                                    } else {
                                        ButtonVariant::Default
                                    }
                                })
                                .on_click(cx.create_callback(
                                    |mut mode: ResMut<NextState<EditorState>>| {
                                        mode.set(EditorState::Preview);
                                    },
                                )),
                            ToolButton::new()
                                .children("Materials")
                                .corners(RoundedCorners::None)
                                .variant({
                                    let st = cx.use_resource::<State<EditorState>>();
                                    if *st.get() == EditorState::Graph {
                                        ButtonVariant::Selected
                                    } else {
                                        ButtonVariant::Default
                                    }
                                })
                                .on_click(cx.create_callback(
                                    |mut mode: ResMut<NextState<EditorState>>| {
                                        mode.set(EditorState::Graph);
                                    },
                                )),
                            ToolButton::new()
                                .children("Split")
                                .corners(RoundedCorners::Right)
                                .variant({
                                    let st = cx.use_resource::<State<EditorState>>();
                                    if *st.get() == EditorState::Split {
                                        ButtonVariant::Selected
                                    } else {
                                        ButtonVariant::Default
                                    }
                                })
                                .on_click(cx.create_callback(
                                    |mut mode: ResMut<NextState<EditorState>>| {
                                        mode.set(EditorState::Split);
                                    },
                                )),
                        )),
                        Element::<NodeBundle>::new()
                            .style(style_button_row)
                            .children((
                                Button::new()
                                    .children("Open…")
                                    .on_click(cx.create_callback(move |world: &mut World| {
                                        let mut log = world.get_resource_mut::<ClickLog>().unwrap();
                                        log.0.push("Open clicked".to_string());
                                        dialog_open.set(world, true);
                                    }))
                                    .style(style_button_flex),
                                Button::new()
                                    .children("Save")
                                    .on_click(cx.create_callback(
                                        move |mut log: ResMut<ClickLog>| {
                                            log.0.push("Save clicked".to_string());
                                        },
                                    ))
                                    .style(style_button_flex),
                            )),
                        Element::<NodeBundle>::new()
                            .style(style_column_group)
                            .children((
                                Checkbox::new()
                                    // .style(|ss: &mut StyleBuilder| {
                                    //     ss.cursor_image("demo://unlock.png", Vec2::new(8., 8.));
                                    // })
                                    .label("Include Author Name")
                                    .checked(checked_1.get(cx))
                                    .on_change(cx.create_callback(
                                        move |checked: In<bool>, world: &mut World| {
                                            let mut log =
                                                world.get_resource_mut::<ClickLog>().unwrap();
                                            log.0
                                                .push(format!("Include Author Name: {}", *checked));
                                            checked_1.set(world, *checked);
                                        },
                                    )),
                                Checkbox::new()
                                    .label("Include Metadata")
                                    .checked(checked_2.get(cx))
                                    .on_change(cx.create_callback(
                                        move |checked: In<bool>, world: &mut World| {
                                            let mut log =
                                                world.get_resource_mut::<ClickLog>().unwrap();
                                            log.0.push(format!("Include Metadata: {}", *checked));
                                            checked_2.set(world, *checked);
                                        },
                                    )),
                            )),
                        Element::<NodeBundle>::new()
                            .style(style_column_group)
                            .children(
                                Slider::new()
                                    .min(0.)
                                    .max(255.)
                                    .value(red.get(cx))
                                    .style(style_slider)
                                    .precision(1)
                                    .on_change(cx.create_callback(
                                        move |value: In<f32>, world: &mut World| {
                                            let mut log =
                                                world.get_resource_mut::<ClickLog>().unwrap();
                                            log.0.push(format!("Slider value: {}", *value));
                                            red.set(world, *value);
                                        },
                                    )),
                                // )
                                // TextInput::new(TextInputProps {
                                //     value: name.signal(),
                                //     on_change: Some(cx.create_callback(
                                //         move |cx: &mut Cx, value: String| {
                                //             name.set_clone(cx, value.clone());
                                //         },
                                //     )),
                                //     ..default()
                                // }
                            ),
                        ResourcePropertyInspector::<TestStruct>::new(),
                        ResourcePropertyInspector::<TestStruct2>::new(),
                        ResourcePropertyInspector::<TestStruct3>::new(),
                        // ReactionsTable,
                        LogList,
                    )),
                Splitter::new()
                    .direction(SplitterDirection::Vertical)
                    .value(panel_width)
                    .on_change(cx.create_callback(|value: In<f32>, world: &mut World| {
                        let mut panel_width = world.get_resource_mut::<PanelWidth>().unwrap();
                        panel_width.0 = value.max(200.);
                    })),
                CenterPanel,
            ))
    }
}
