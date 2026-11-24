use bevy_app::{Plugin, PreUpdate, PropagateOver};
use bevy_camera::visibility::Visibility;
use bevy_ecs::{
    change_detection::DetectChanges,
    component::Component,
    entity::Entity,
    hierarchy::{ChildOf, Children},
    lifecycle::RemovedComponents,
    query::{Added, Changed, Has, With},
    reflect::ReflectComponent,
    schedule::IntoScheduleConfigs,
    system::{Commands, Query, Res},
};
use bevy_input_focus::tab_navigation::TabIndex;
use bevy_picking::{cursor::EntityCursor, Pickable, PickingSystems};
use bevy_reflect::std_traits::ReflectDefault;
use bevy_reflect::Reflect;
use bevy_scene::prelude::*;
use bevy_text::{
    EditableText, FontSourceTemplate, FontWeight, LineBreak, TextCursorStyle, TextFont, TextLayout,
    TextReadWriteMode,
};
use bevy_ui::{
    px, widget::Text, AlignItems, BorderRadius, Display, InteractionDisabled, JustifyContent, Node,
    PositionType, UiRect,
};
use bevy_ui_widgets::TextInput;

use crate::{
    constants::{fonts, size},
    focus::FocusWithinIndicator,
    font_styles::InheritableFont,
    theme::{
        InheritableThemeTextColor, SurfaceLevel, ThemeBackgroundColor, ThemeContext, ThemedText,
        UiTheme,
    },
    tokens,
};

/// Decorative frame around a text input widget. This is a separate entity to allow icons
/// (such as "search" or "clear") to be inserted adjacent to the input.
///
/// This is spawnable by inheriting it as a "scene component".
#[derive(SceneComponent, Default, Clone, Reflect)]
#[reflect(Component, Default, Clone)]
pub struct FeathersTextInputContainer;

impl FeathersTextInputContainer {
    fn scene() -> impl Scene {
        bsn! {
            Node {
                height: size::ROW_HEIGHT,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect {
                    right: px(3.0),
                    left: px(3.0),
                },
                flex_grow: 1.0,
                border_radius: {BorderRadius::all(px(4.0))},
                column_gap: px(4),
            }
            FeathersTextInputContainer
            FocusWithinIndicator
            ThemeBackgroundColor(tokens::TEXT_INPUT_BG)
            InheritableThemeTextColor(tokens::TEXT_INPUT_TEXT)
            InheritableFont {
                font: fonts::REGULAR,
                font_size: size::COMPACT_FONT,
                weight: FontWeight::NORMAL,
            }
        }
    }
}

/// Scene function to spawn a text input. For proper styling, this should be enclosed by a [`FeathersTextInputContainer`].
///
/// This is spawnable by inheriting it as a "scene component" with optional [`FeathersTextInputProps`].
///
/// ```ignore
/// @FeathersTextInputContainer
/// Children [
///     :FeathersTextInput
/// ]
/// ```
#[derive(SceneComponent, Default, Clone)]
#[scene(FeathersTextInputProps)]
#[derive(Reflect)]
#[reflect(Component, Default, Clone)]
pub struct FeathersTextInput;

/// Props used to construct the [`FeathersTextInput`] scene.
#[derive(Default, Clone)]
pub struct FeathersTextInputProps {
    /// Visible width
    pub visible_width: Option<f32>,
    /// Max characters
    pub max_characters: Option<usize>,
}

impl FeathersTextInput {
    fn scene(props: FeathersTextInputProps) -> impl Scene {
        bsn! {
            Node {
                flex_grow: {
                    if props.visible_width.is_some() {
                        0_f32
                    } else {
                        1_f32
                    }
                } ,
            }
            FeathersTextInput
            TextInput
            EditableText {
                cursor_width: 0.3,
                visible_width: {props.visible_width},
                max_characters: {props.max_characters},
            }
            ThemedText
            TextLayout {
                linebreak: LineBreak::NoWrap,
            }
            TabIndex(0)
            TextFont {
                font: FontSourceTemplate::Handle(fonts::REGULAR),
                font_size: size::COMPACT_FONT,
                weight: FontWeight::NORMAL,
            }
            PropagateOver<TextFont>
            EntityCursor::System(bevy_window::SystemCursorIcon::Text)
            TextCursorStyle
        }
    }
}

/// Marker for the placeholder label of a text input; see [`text_input_placeholder`].
#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default, Clone)]
pub struct FeathersTextInputPlaceholder;

/// Scene function for a placeholder label, shown while the sibling [`FeathersTextInput`] is empty.
///
/// Place it inside the same [`FeathersTextInputContainer`] as the input. It is absolutely
/// positioned over the input and ignores picking, so clicks still reach the input. Patch its
/// `Node` to line it up with a container that uses different padding, and its
/// [`InheritableThemeTextColor`] to change the text color.
///
/// ```ignore
/// @FeathersTextInputContainer
/// Children [
///     @FeathersTextInput
///     --
///     @text_input_placeholder("Search...")
/// ]
/// ```
pub fn text_input_placeholder(text: impl Into<String>) -> impl Scene {
    bsn! {
        // The container spans the input's height and centers the text vertically.
        Node {
            position_type: PositionType::Absolute,
            left: px(3),
            top: px(0),
            bottom: px(0),
            display: Display::Flex,
            align_items: AlignItems::Center,
        }
        FeathersTextInputPlaceholder
        Pickable::IGNORE
        InheritableThemeTextColor(tokens::TEXT_DIM)
        Children [
            Text(text)
            ThemedText
            TextFont {
                font: FontSourceTemplate::Handle(fonts::REGULAR),
                font_size: size::COMPACT_FONT,
                weight: FontWeight::NORMAL,
            }
            Pickable::IGNORE
        ]
    }
}

/// Shows each placeholder only while the text input next to it is empty.
fn update_text_input_placeholders(
    q_inputs: Query<(&EditableText, &ChildOf), (With<FeathersTextInput>, Changed<EditableText>)>,
    q_children: Query<&Children>,
    mut q_placeholders: Query<&mut Visibility, With<FeathersTextInputPlaceholder>>,
) {
    for (editable, child_of) in &q_inputs {
        let visibility = if editable.value() == "" {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        let Ok(siblings) = q_children.get(child_of.parent()) else {
            continue;
        };
        for &sibling in siblings {
            if let Ok(mut placeholder) = q_placeholders.get_mut(sibling)
                && *placeholder != visibility
            {
                *placeholder = visibility;
            }
        }
    }
}

fn update_text_cursor_color(
    mut q_text_input: Query<(&mut TextCursorStyle, Option<&ThemeContext>), With<FeathersTextInput>>,
    theme: Res<UiTheme>,
) {
    if theme.is_changed() {
        for (mut cursor_style, theme_context) in q_text_input.iter_mut() {
            let context = theme_context.map(|tc| tc.0).unwrap_or(SurfaceLevel::Base);
            cursor_style.color = theme.context_color(&tokens::TEXT_INPUT_CURSOR, context);
            cursor_style.selection_color =
                theme.context_color(&tokens::TEXT_INPUT_SELECTION, context);
            cursor_style.unfocused_selection_color =
                theme.context_color(&tokens::TEXT_INPUT_SELECTION_UNFOCUSED, context);
        }
    }
}

fn update_text_input_styles(
    q_inputs: Query<
        (Entity, Has<InteractionDisabled>, &InheritableThemeTextColor),
        (With<FeathersTextInput>, Added<InteractionDisabled>),
    >,
    mut commands: Commands,
) {
    for (input_ent, disabled, font_color) in q_inputs.iter() {
        set_text_input_styles(input_ent, disabled, font_color, &mut commands);
        commands
            .entity(input_ent)
            .insert(TextReadWriteMode::ReadOnly);
    }
}

fn update_text_input_styles_remove(
    q_inputs: Query<
        (Entity, Has<InteractionDisabled>, &InheritableThemeTextColor),
        With<FeathersTextInput>,
    >,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
    mut commands: Commands,
) {
    removed_disabled.read().for_each(|ent| {
        if let Ok((input_ent, disabled, font_color)) = q_inputs.get(ent) {
            set_text_input_styles(input_ent, disabled, font_color, &mut commands);
            commands
                .entity(input_ent)
                .insert(TextReadWriteMode::Editable);
        }
    });
}

fn set_text_input_styles(
    input_ent: Entity,
    disabled: bool,
    font_color: &InheritableThemeTextColor,
    commands: &mut Commands,
) {
    let font_color_token = match disabled {
        true => tokens::TEXT_INPUT_TEXT_DISABLED,
        false => tokens::TEXT_INPUT_TEXT,
    };

    let cursor_shape = match disabled {
        true => bevy_window::SystemCursorIcon::NotAllowed,
        false => bevy_window::SystemCursorIcon::Text,
    };

    // Change font color
    if font_color.0 != font_color_token {
        commands
            .entity(input_ent)
            .insert(InheritableThemeTextColor(font_color_token));
    }

    // Change cursor shape
    commands
        .entity(input_ent)
        .insert(EntityCursor::System(cursor_shape));
}

/// Plugin which registers the systems for updating the text input styles.
pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            PreUpdate,
            (
                update_text_cursor_color,
                update_text_input_styles,
                update_text_input_styles_remove,
                update_text_input_placeholders,
            )
                .in_set(PickingSystems::Last),
        );
    }
}
