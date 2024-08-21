use bevy::prelude::*;

use components::{OutlinedBlock, Page, TextureAtlasInteraction, UiFooterRootNode};
use ease::Ease;
use resources::CurrentPage;
use sickle_ui::{
    dev_panels::{
        hierarchy::HierarchyTreeViewPlugin,
        scene_view::{SceneViewPlugin, SpawnSceneViewPreUpdate},
    },
    prelude::*,
    SickleUiPlugin,
};
use systems::{
    clear_content_on_menu_change, despawn_hierarchy_view, exit_app_on_menu_item,
    handle_theme_contrast_select, handle_theme_data_update, handle_theme_switch,
    interaction_showcase, layout_showcase, setup, spawn_hierarchy_view, update_current_page,
};

pub mod components;
pub mod resources;
pub mod systems;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SickleUiPlugin,
            UiFooterRootNodePlugin,
            OutlinedBlockPlugin,
            TextureAtlasInteractionPlugin,
        ))
        .init_resource::<CurrentPage>()
        .init_state::<Page>()
        .add_plugins((HierarchyTreeViewPlugin, SceneViewPlugin))
        .add_systems(Startup, setup.in_set(UiStartupSet))
        .add_systems(OnEnter(Page::Layout), layout_showcase)
        .add_systems(OnExit(Page::Layout), clear_content_on_menu_change)
        .add_systems(OnEnter(Page::Playground), interaction_showcase)
        .add_systems(OnExit(Page::Playground), clear_content_on_menu_change)
        .add_systems(
            PreUpdate,
            (
                exit_app_on_menu_item,
                spawn_hierarchy_view,
                despawn_hierarchy_view,
            )
                .after(SpawnSceneViewPreUpdate),
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
        )
        .run();
    }
}

pub struct UiFooterRootNodePlugin;

impl Plugin for UiFooterRootNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentThemePlugin::<UiFooterRootNode>::default());
    }
}

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

    fn primary_style(style_builder: &mut StyleBuilder, theme_data: &ThemeData) {
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

    fn frame() -> impl Bundle {
        (Name::new("UiFooterRootNode"), NodeBundle::default())
    }
}

pub trait UiFooterRootNodeExt {
    fn ui_footer(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl UiFooterRootNodeExt for UiBuilder<'_, Entity> {
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

    fn frame() -> impl Bundle {
        (
            Name::new("TextureAtlasInteraction"),
            ImageBundle::default(),
            Outline::default(),
        )
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

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub struct UiStartupSet;
