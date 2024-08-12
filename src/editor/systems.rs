use bevy::prelude::*;
use sickle_ui::{
    dev_panels::{
        hierarchy::UiHierarchyExt,
        scene_view::{SceneView, UiSceneViewExt},
    },
    prelude::*,
    ui_commands::{SetCursorExt, UpdateStatesExt},
};

use super::{
    components::{
        ExitAppButton, HierarchyPanel, Page, ShowcaseContainer, ThemeContrastSelect, ThemeSwitch,
        UiCamera, UiMainRootNode,
    },
    UiOutlinedBlockExt, UiTextureAtlasInteractionExt, UiUiFooterRootNodeExt,
};

pub fn get_selected_scheme(
    theme_switch: &RadioGroup,
    theme_contrast_select: &Dropdown,
) -> Option<Scheme> {
    let contrast = match theme_contrast_select.value() {
        Some(index) => match index {
            0 => Contrast::Standard,
            1 => Contrast::Medium,
            2 => Contrast::High,
            _ => Contrast::Standard,
        },
        None => Contrast::Standard,
    };

    if let Some(index) = theme_switch.selected() {
        let scheme = match index {
            0 => Scheme::Light(contrast),
            1 => Scheme::Dark(contrast),
            _ => Scheme::Light(contrast),
        };

        Some(scheme)
    } else {
        None
    }
}

pub fn handle_theme_contrast_select(
    mut theme_data: ResMut<ThemeData>,
    q_theme_switch: Query<&RadioGroup, With<ThemeSwitch>>,
    q_theme_contrast_select: Query<&Dropdown, (With<ThemeContrastSelect>, Changed<Dropdown>)>,
) {
    let Ok(theme_contrast_select) = q_theme_contrast_select.get_single() else {
        return;
    };

    let Ok(theme_switch) = q_theme_switch.get_single() else {
        return;
    };

    if let Some(scheme) = get_selected_scheme(theme_switch, theme_contrast_select) {
        if theme_data.active_scheme != scheme {
            theme_data.active_scheme = scheme;
        }
    }
}

pub fn handle_theme_switch(
    mut theme_data: ResMut<ThemeData>,
    q_theme_switch: Query<&RadioGroup, (With<ThemeSwitch>, Changed<RadioGroup>)>,
    q_theme_contrast_select: Query<&Dropdown, With<ThemeContrastSelect>>,
) {
    let Ok(theme_switch) = q_theme_switch.get_single() else {
        return;
    };

    let Ok(theme_contrast_select) = q_theme_contrast_select.get_single() else {
        return;
    };

    if let Some(scheme) = get_selected_scheme(theme_switch, theme_contrast_select) {
        if theme_data.active_scheme != scheme {
            theme_data.active_scheme = scheme;
        }
    }
}

pub fn handle_theme_data_update(
    theme_data: Res<ThemeData>,
    mut q_theme_switch: Query<&mut RadioGroup, With<ThemeSwitch>>,
    mut q_theme_contrast_select: Query<&mut Dropdown, With<ThemeContrastSelect>>,
) {
    if theme_data.is_changed() {
        let Ok(mut theme_switch) = q_theme_switch.get_single_mut() else {
            return;
        };

        let Ok(mut theme_contrast_select) = q_theme_contrast_select.get_single_mut() else {
            return;
        };

        match theme_data.active_scheme {
            Scheme::Light(contrast) => {
                theme_switch.select(0);
                match contrast {
                    Contrast::Standard => theme_contrast_select.set_value(0),
                    Contrast::Medium => theme_contrast_select.set_value(1),
                    Contrast::High => theme_contrast_select.set_value(2),
                };
            }
            Scheme::Dark(contrast) => {
                theme_switch.select(1);
                match contrast {
                    Contrast::Standard => theme_contrast_select.set_value(0),
                    Contrast::Medium => theme_contrast_select.set_value(1),
                    Contrast::High => theme_contrast_select.set_value(2),
                };
            }
        };
    }
}

pub fn setup(mut commands: Commands) {
    // The main camera which will render UI
    let main_camera = commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    order: 1,
                    clear_color: Color::BLACK.into(),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 30., 0.))
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            UiCamera,
        ))
        .id();

    let mut root_entity = Entity::PLACEHOLDER;
    commands
        .ui_builder(UiRoot)
        .container(
            (
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                },
                TargetCamera(main_camera),
            ),
            |container| {
                root_entity = container
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            ..default()
                        },
                        UiMainRootNode,
                    ))
                    .id();

                container.ui_footer(|_| {});
            },
        )
        .floating_panel(
            FloatingPanelConfig {
                title: Some("Root floating panel".into()),
                ..default()
            },
            FloatingPanelLayout {
                size: Vec2::new(200., 300.),
                position: Vec2::new(100., 100.).into(),
                droppable: true,
            },
            |_| {},
        )
        .insert(TargetCamera(main_camera));

    // Use the UI builder of the root entity with styling applied via commands
    commands.ui_builder(root_entity).column(|column| {
        column
            .style()
            .width(Val::Percent(100.))
            .background_color(Color::srgb(0.15, 0.155, 0.16));

        column.menu_bar(|bar| {
            bar.menu(
                MenuConfig {
                    name: "Showcase".into(),
                    alt_code: KeyCode::KeyS.into(),
                    ..default()
                },
                |menu| {
                    menu.menu_item(MenuItemConfig {
                        name: "Layout".into(),
                        shortcut: vec![KeyCode::KeyL].into(),
                        alt_code: KeyCode::KeyL.into(),
                        ..default()
                    })
                    .insert(Page::Layout);
                    menu.menu_item(MenuItemConfig {
                        name: "Interactions".into(),
                        shortcut: vec![KeyCode::ControlLeft, KeyCode::KeyI].into(),
                        alt_code: KeyCode::KeyI.into(),
                        ..default()
                    })
                    .insert(Page::Playground);

                    menu.separator();

                    let icons = ThemeData::default().icons;
                    menu.menu_item(MenuItemConfig {
                        name: "Exit".into(),
                        leading_icon: icons.exit_to_app,
                        ..default()
                    })
                    .insert(ExitAppButton);
                },
            );
            bar.menu(
                MenuConfig {
                    name: "Use case".into(),
                    alt_code: KeyCode::KeyS.into(),
                    ..default()
                },
                |menu| {
                    menu.menu_item(MenuItemConfig {
                        name: "Standard menu item".into(),
                        ..default()
                    });
                    menu.menu_item(MenuItemConfig {
                        name: "Menu item with leading icon".into(),
                        leading_icon: IconData::Image(
                            "embedded://sickle_ui/icons/details_menu.png".into(),
                            Color::WHITE,
                        ),
                        ..default()
                    });
                    menu.menu_item(MenuItemConfig {
                        name: "Menu item with trailing icon".into(),
                        trailing_icon: IconData::Image(
                            "embedded://sickle_ui/icons/tiles_menu.png".into(),
                            Color::WHITE,
                        ),
                        ..default()
                    });

                    menu.menu_item(MenuItemConfig {
                        name: "Menu item with both icons".into(),
                        leading_icon: IconData::Image(
                            "embedded://sickle_ui/icons/details_menu.png".into(),
                            Color::WHITE,
                        ),
                        trailing_icon: IconData::Image(
                            "embedded://sickle_ui/icons/tiles_menu.png".into(),
                            Color::WHITE,
                        ),
                        ..default()
                    });

                    menu.separator();

                    menu.toggle_menu_item(ToggleMenuItemConfig {
                        name: "Toggle item".into(),
                        shortcut: vec![KeyCode::ControlLeft, KeyCode::KeyT].into(),
                        ..default()
                    });
                    menu.toggle_menu_item(ToggleMenuItemConfig {
                        name: "Already toggled item".into(),
                        initially_checked: true,
                        ..default()
                    });
                    menu.toggle_menu_item(ToggleMenuItemConfig {
                        name: "Toggle item with trailing icon".into(),
                        trailing_icon: IconData::Image(
                            "embedded://sickle_ui/icons/tiles_menu.png".into(),
                            Color::WHITE,
                        ),
                        ..default()
                    });

                    menu.separator();

                    menu.submenu(
                        SubmenuConfig {
                            name: "Submenu".into(),
                            ..default()
                        },
                        |submenu| {
                            submenu.menu_item(MenuItemConfig {
                                name: "Standard menu item".into(),
                                ..default()
                            });
                            submenu.menu_item(MenuItemConfig {
                                name: "Menu item with leading icon".into(),
                                leading_icon: IconData::Image(
                                    "embedded://sickle_ui/icons/details_menu.png".into(),
                                    Color::WHITE,
                                ),
                                ..default()
                            });
                            submenu.menu_item(MenuItemConfig {
                                name: "Menu item with trailing icon".into(),
                                trailing_icon: IconData::Image(
                                    "embedded://sickle_ui/icons/tiles_menu.png".into(),
                                    Color::WHITE,
                                ),
                                ..default()
                            });
                        },
                    );
                },
            );

            bar.menu(
                MenuConfig {
                    name: "Test case".into(),
                    alt_code: KeyCode::KeyS.into(),
                    ..default()
                },
                |menu| {
                    menu.menu_item(MenuItemConfig {
                        name: "Standard menu item".into(),
                        ..default()
                    });
                    menu.menu_item(MenuItemConfig {
                        name: "Menu item with leading icon".into(),
                        leading_icon: IconData::Image(
                            "embedded://sickle_ui/icons/details_menu.png".into(),
                            Color::WHITE,
                        ),
                        ..default()
                    });
                    menu.menu_item(MenuItemConfig {
                        name: "Menu item with trailing icon".into(),
                        trailing_icon: IconData::Image(
                            "embedded://sickle_ui/icons/tiles_menu.png".into(),
                            Color::WHITE,
                        ),
                        ..default()
                    });

                    menu.menu_item(MenuItemConfig {
                        name: "Menu item with both icons".into(),
                        leading_icon: IconData::Image(
                            "embedded://sickle_ui/icons/details_menu.png".into(),
                            Color::WHITE,
                        ),
                        trailing_icon: IconData::Image(
                            "embedded://sickle_ui/icons/tiles_menu.png".into(),
                            Color::WHITE,
                        ),
                        ..default()
                    });

                    menu.separator();

                    menu.toggle_menu_item(ToggleMenuItemConfig {
                        name: "Toggle item".into(),
                        shortcut: vec![KeyCode::ControlLeft, KeyCode::KeyT].into(),
                        ..default()
                    });
                    menu.toggle_menu_item(ToggleMenuItemConfig {
                        name: "Already toggled item".into(),
                        initially_checked: true,
                        ..default()
                    });
                    menu.toggle_menu_item(ToggleMenuItemConfig {
                        name: "Toggle item with trailing icon".into(),
                        trailing_icon: IconData::Image(
                            "embedded://sickle_ui/icons/tiles_menu.png".into(),
                            Color::WHITE,
                        ),
                        ..default()
                    });

                    menu.separator();

                    menu.submenu(
                        SubmenuConfig {
                            name: "Submenu".into(),
                            ..default()
                        },
                        |submenu| {
                            submenu.menu_item(MenuItemConfig {
                                name: "Standard menu item".into(),
                                ..default()
                            });
                            submenu.menu_item(MenuItemConfig {
                                name: "Menu item with leading icon".into(),
                                leading_icon: IconData::Image(
                                    "embedded://sickle_ui/icons/details_menu.png".into(),
                                    Color::WHITE,
                                ),
                                ..default()
                            });
                            submenu.menu_item(MenuItemConfig {
                                name: "Menu item with trailing icon".into(),
                                trailing_icon: IconData::Image(
                                    "embedded://sickle_ui/icons/tiles_menu.png".into(),
                                    Color::WHITE,
                                ),
                                ..default()
                            });

                            submenu.submenu(
                                SubmenuConfig {
                                    name: "Submenu with lead icon".into(),
                                    leading_icon: IconData::Image(
                                        "embedded://sickle_ui/icons/details_menu.png".into(),
                                        Color::WHITE,
                                    ),
                                    ..default()
                                },
                                |submenu| {
                                    submenu.menu_item(MenuItemConfig {
                                        name: "Standard menu item".into(),
                                        ..default()
                                    });
                                    submenu.menu_item(MenuItemConfig {
                                        name: "Menu item with leading icon".into(),
                                        leading_icon: IconData::Image(
                                            "embedded://sickle_ui/icons/details_menu.png".into(),
                                            Color::WHITE,
                                        ),
                                        ..default()
                                    });
                                    submenu.menu_item(MenuItemConfig {
                                        name: "Menu item with trailing icon".into(),
                                        trailing_icon: IconData::Image(
                                            "embedded://sickle_ui/icons/tiles_menu.png".into(),
                                            Color::WHITE,
                                        ),
                                        ..default()
                                    });
                                },
                            );
                        },
                    );
                },
            );

            bar.separator();

            bar.extra_menu(|extra| {
                extra
                    .radio_group(vec!["Light", "Dark"], 1, false)
                    .insert(ThemeSwitch);
                extra
                    .dropdown(vec!["Standard", "Medium Contrast", "High Contrast"], 0)
                    .insert(ThemeContrastSelect)
                    .style()
                    .width(Val::Px(150.));
            });
        });

        column
            .row(|_| {})
            .insert((ShowcaseContainer, UiContextRoot))
            .style()
            .height(Val::Percent(100.))
            .background_color(Color::NONE);
    });

    commands.next_state(Page::Layout);
}

pub fn exit_app_on_menu_item(
    q_menu_items: Query<&MenuItem, (With<ExitAppButton>, Changed<MenuItem>)>,
    q_windows: Query<Entity, With<Window>>,
    mut commands: Commands,
) {
    let Ok(item) = q_menu_items.get_single() else {
        return;
    };

    if item.interacted() {
        for entity in &q_windows {
            commands.entity(entity).remove::<Window>();
        }
    }
}

pub fn update_current_page(
    mut next_state: ResMut<NextState<Page>>,
    q_menu_items: Query<(&Page, &MenuItem), Changed<MenuItem>>,
) {
    for (menu_type, menu_item) in &q_menu_items {
        if menu_item.interacted() {
            next_state.set(*menu_type);
        }
    }
}

pub fn clear_content_on_menu_change(
    root_node: Query<Entity, With<ShowcaseContainer>>,
    mut commands: Commands,
) {
    let root_entity = root_node.single();
    commands.entity(root_entity).despawn_descendants();
    commands.set_cursor(CursorIcon::Default);
}

pub fn spawn_hierarchy_view(
    q_added_scene_view: Query<&SceneView, Added<SceneView>>,
    q_hierarchy_panel: Query<Entity, With<HierarchyPanel>>,

    mut commands: Commands,
) {
    for scene_view in &q_added_scene_view {
        let Ok(container) = q_hierarchy_panel.get_single() else {
            return;
        };

        commands.entity(container).despawn_descendants();
        commands
            .ui_builder(container)
            .hierarchy_for(scene_view.asset_root());
        break;
    }
}

pub fn despawn_hierarchy_view(
    q_hierarchy_panel: Query<Entity, With<HierarchyPanel>>,
    q_removed_scene_view: RemovedComponents<SceneView>,
    mut commands: Commands,
) {
    let Ok(container) = q_hierarchy_panel.get_single() else {
        return;
    };

    if q_removed_scene_view.len() > 0 {
        commands.entity(container).despawn_descendants();
    }
}

pub fn layout_showcase(root_node: Query<Entity, With<ShowcaseContainer>>, mut commands: Commands) {
    let root_entity = root_node.single();

    commands
        .ui_builder(root_entity)
        .row(|row| {
            row.docking_zone_split(
                SizedZoneConfig {
                    size: 75.,
                    ..default()
                },
                |left_side| {
                    left_side.docking_zone_split(
                        SizedZoneConfig {
                            size: 75.,
                            ..default()
                        },
                        |left_side_top| {
                            left_side_top.docking_zone(
                                SizedZoneConfig {
                                    size: 25.,
                                    ..default()
                                },
                                true,
                                |tab_container| {
                                    tab_container.add_tab("Hierarchy".into(), |panel| {
                                        panel.insert(HierarchyPanel);
                                    });
                                    tab_container.add_tab("Tab 3".into(), |panel| {
                                        panel.label(LabelConfig {
                                            label: "Panel 3".into(),
                                            ..default()
                                        });
                                    });
                                },
                            );
                            left_side_top.docking_zone(
                                SizedZoneConfig {
                                    size: 75.,
                                    ..default()
                                },
                                false,
                                |tab_container| {
                                    tab_container.add_tab("Scene View".into(), |panel| {
                                        panel.scene_view("levels/derelict_district.gltf#Scene0");
                                    });
                                    tab_container.add_tab("Tab 2".into(), |panel| {
                                        panel.label(LabelConfig {
                                            label: "Panel 2".into(),
                                            ..default()
                                        });
                                    });
                                    tab_container.add_tab("Tab 3".into(), |panel| {
                                        panel.label(LabelConfig {
                                            label: "Panel 3".into(),
                                            ..default()
                                        });
                                    });
                                },
                            );
                        },
                    );

                    left_side.docking_zone(
                        SizedZoneConfig {
                            size: 25.,
                            ..default()
                        },
                        true,
                        |tab_container| {
                            tab_container.add_tab("Systems".into(), |panel| {
                                panel.label(LabelConfig {
                                    label: "Systems".into(),
                                    ..default()
                                });
                            });
                            tab_container.add_tab("Tab 6".into(), |panel| {
                                panel.label(LabelConfig {
                                    label: "Panel 6".into(),
                                    ..default()
                                });
                            });
                        },
                    );
                },
            );

            row.docking_zone_split(
                SizedZoneConfig {
                    size: 25.,
                    ..default()
                },
                |right_side| {
                    right_side.docking_zone(
                        SizedZoneConfig {
                            size: 25.,
                            ..default()
                        },
                        true,
                        |tab_container| {
                            tab_container.add_tab("Placeholder".into(), |placeholder| {
                                placeholder.style().padding(UiRect::all(Val::Px(10.)));

                                placeholder.row(|row| {
                                    row.checkbox(None, false);
                                    row.radio_group(vec!["Light", "Dark"], 1, false);
                                });

                                placeholder.row(|row| {
                                    row.style().justify_content(JustifyContent::SpaceBetween);
                                    row.dropdown(
                                        vec![
                                            "Standard",
                                            "Medium Contrast",
                                            "High Contrast - High Contrast",
                                        ],
                                        None,
                                    );

                                    row.dropdown(
                                        vec![
                                            "Standard",
                                            "Medium Contrast",
                                            "High Contrast - High Contrast",
                                        ],
                                        None,
                                    );
                                });

                                placeholder.outlined_block();
                                placeholder.atlas_example();

                                placeholder.row(|row| {
                                    row.style().justify_content(JustifyContent::SpaceBetween);
                                    row.dropdown(
                                        vec![
                                            "Standard",
                                            "Medium Contrast",
                                            "High Contrast - High Contrast",
                                        ],
                                        None,
                                    );
                                    row.checkbox(None, false);
                                    row.dropdown(
                                        vec![
                                            "Standard",
                                            "Medium Contrast",
                                            "High Contrast - High Contrast",
                                        ],
                                        None,
                                    );
                                });
                            });

                            tab_container.add_tab("Sliders".into(), |slider_tab| {
                                slider_tab
                                    .row(|row| {
                                        row.slider(SliderConfig::vertical(
                                            String::from("Slider"),
                                            0.,
                                            5.,
                                            2.,
                                            true,
                                        ));

                                        row.slider(SliderConfig::vertical(None, 0., 5., 2., true));

                                        row.slider(SliderConfig::vertical(
                                            String::from("Slider"),
                                            0.,
                                            5.,
                                            2.,
                                            false,
                                        ));

                                        row.slider(SliderConfig::vertical(None, 0., 5., 2., false));
                                    })
                                    .style()
                                    .height(Val::Percent(50.));

                                slider_tab
                                    .column(|row| {
                                        row.slider(SliderConfig::horizontal(
                                            String::from("Slider"),
                                            0.,
                                            5.,
                                            2.,
                                            true,
                                        ));
                                        row.slider(SliderConfig::horizontal(
                                            None, 0., 5., 2., true,
                                        ));
                                        row.slider(SliderConfig::horizontal(
                                            String::from("Slider"),
                                            0.,
                                            5.,
                                            2.,
                                            false,
                                        ));
                                        row.slider(SliderConfig::horizontal(
                                            None, 0., 5., 2., false,
                                        ));
                                    })
                                    .style()
                                    .justify_content(JustifyContent::End)
                                    .height(Val::Percent(50.))
                                    .width(Val::Percent(100.));
                            });
                        },
                    );
                },
            );
        })
        .style()
        .height(Val::Percent(100.));
}

pub fn interaction_showcase(
    root_node: Query<Entity, With<ShowcaseContainer>>,
    mut commands: Commands,
) {
    let root_entity = root_node.single();

    commands.ui_builder(root_entity).column(|_column| {
        // Test here simply by calling methods on the `column`
    });
}
