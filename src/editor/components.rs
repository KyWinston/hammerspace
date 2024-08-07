use bevy::prelude::*;
use ease::Ease;
use sickle_ui::prelude::*;

#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct UiMainRootNode;

#[derive(Component, Debug, Default, Reflect, UiContext)]
#[reflect(Component)]
pub struct UiFooterRootNode;

impl DefaultTheme for UiFooterRootNode {
    fn default_theme() -> Option<Theme<UiFooterRootNode>> {
        UiFooterRootNode::theme().into()
    }
}

impl UiFooterRootNode {
    pub fn theme() -> Theme<UiFooterRootNode> {
        let base_theme = PseudoTheme::deferred(None, UiFooterRootNode::primary_style);
        Theme::new(vec![base_theme])
    }

    pub fn primary_style(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let theme_spacing = theme_data.spacing;
        let colors = theme_data.colors();

        style_builder
            .justify_content(JustifyContent::SpaceBetween)
            .width(Val::Percent(100.))
            .height(Val::Px(theme_spacing.areas.medium))
            .border(UiRect::top(Val::Px(theme_spacing.borders.extra_small)))
            .border_color(colors.accent(Accent::Shadow))
            .background_color(colors.container(Container::SurfaceMid));
    }

    pub fn frame() -> impl Bundle {
        (Name::new("UiFooterRootNode"), NodeBundle::default())
    }
}


#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Reflect, States, Hash)]
#[reflect(Component)]
pub enum Page {
    #[default]
    None,
    Layout,
    Playground,
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct ExitAppButton;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct ShowcaseContainer;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HierarchyPanel;

#[derive(Component, Debug)]
pub struct ThemeSwitch;

#[derive(Component, Debug)]
pub struct ThemeContrastSelect;

#[derive(Component, Debug, Default, Reflect, UiContext)]
#[reflect(Component)]
pub struct TextureAtlasInteraction;

impl DefaultTheme for TextureAtlasInteraction {
    fn default_theme() -> Option<Theme<TextureAtlasInteraction>> {
        TextureAtlasInteraction::theme().into()
    }
}

impl TextureAtlasInteraction {
    pub fn theme() -> Theme<TextureAtlasInteraction> {
        let base_theme = PseudoTheme::deferred(None, TextureAtlasInteraction::primary_style);
        Theme::new(vec![base_theme])
    }

    fn primary_style(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let theme_spacing = theme_data.spacing;
        let colors = theme_data.colors();

        style_builder
            .size(Val::Px(96.))
            .align_self(AlignSelf::Center)
            .justify_self(JustifySelf::Center)
            .margin(UiRect::all(Val::Px(30.)))
            .background_color(colors.accent(Accent::OutlineVariant))
            .outline(Outline {
                width: Val::Px(5.),
                color: colors.accent(Accent::Primary),
                ..default()
            })
            .padding(UiRect::all(Val::Px(theme_spacing.gaps.small)))
            .animated()
            .atlas_index(AnimatedVals {
                enter_from: Some(0),
                idle: 7,
                idle_alt: Some(0),
                hover: Some(8),
                hover_alt: Some(15),
                press: Some(16),
                press_alt: Some(23),
                cancel: Some(31),
                ..default()
            })
            .enter(0.4, Ease::Linear, 0.)
            .idle(0.4, Ease::Linear, 0., 0., AnimationLoop::PingPongContinous)
            .pointer_enter(0.4, Ease::Linear, 0.)
            .hover(0.4, Ease::Linear, 0., 0., AnimationLoop::PingPongContinous)
            .pointer_leave(0.4, Ease::Linear, 0.)
            .press(0.4, Ease::Linear, 0.)
            .pressed(0.4, Ease::Linear, 0., 0., AnimationLoop::PingPongContinous)
            .release(0.4, Ease::Linear, 0.)
            .cancel(0.8, Ease::Linear, 0.)
            .cancel_reset(1.2, Ease::InOutCubic, 0.1);
    }

    pub fn frame() -> impl Bundle {
        (
            Name::new("TextureAtlasInteraction"),
            ImageBundle::default(),
            Outline::default(),
        )
    }
}

#[derive(Component, Debug, Default, Reflect, UiContext)]
#[reflect(Component)]
pub struct OutlinedBlock;

impl DefaultTheme for OutlinedBlock {
    fn default_theme() -> Option<Theme<OutlinedBlock>> {
        OutlinedBlock::theme().into()
    }
}

impl OutlinedBlock {
    pub fn theme() -> Theme<OutlinedBlock> {
        let base_theme = PseudoTheme::deferred(None, OutlinedBlock::primary_style);
        Theme::new(vec![base_theme])
    }

    fn primary_style(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
        let theme_spacing = theme_data.spacing;
        let colors = theme_data.colors();

        style_builder
            .size(Val::Px(100.))
            .align_self(AlignSelf::Center)
            .justify_self(JustifySelf::Center)
            .margin(UiRect::all(Val::Px(30.)))
            .background_color(colors.accent(Accent::Primary))
            .padding(UiRect::all(Val::Px(theme_spacing.gaps.small)))
            .animated()
            .outline_width(AnimatedVals {
                idle: Val::Px(0.),
                hover: Val::Px(10.).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);

        style_builder
            .animated()
            .outline_color(AnimatedVals {
                idle: colors.accent(Accent::Outline),
                hover: colors.accent(Accent::OutlineVariant).into(),
                hover_alt: colors.accent(Accent::Outline).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation)
            .hover(
                0.3,
                Ease::InOutBounce,
                0.5,
                0.,
                AnimationLoop::PingPongContinous,
            );

        style_builder
            .animated()
            .outline_offset(AnimatedVals {
                idle: Val::Px(0.),
                press: Val::Px(10.).into(),
                press_alt: Val::Px(12.).into(),
                ..default()
            })
            .copy_from(theme_data.interaction_animation)
            .pressed(
                0.3,
                Ease::InOutBounce,
                0.5,
                0.,
                AnimationLoop::PingPongContinous,
            );
    }

    pub fn frame() -> impl Bundle {
        (
            Name::new("Outlined Block"),
            NodeBundle::default(),
            Outline::default(),
        )
    }
}
