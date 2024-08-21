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

#[derive(Component, Debug, Default, Reflect, UiContext)]
#[reflect(Component)]
pub struct TextureAtlasInteraction;

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

    pub fn primary_style(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
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
