//! Various non-themable constants for the Feathers look and feel.

/// Font asset paths
pub mod fonts {
    /// Default regular font path
    pub const REGULAR: &str = "embedded://bevy_feathers/assets/fonts/FiraSans-Regular.ttf";
    /// Regular italic font path
    pub const ITALIC: &str = "embedded://bevy_feathers/assets/fonts/FiraSans-Italic.ttf";
    /// Medium-weight font path
    pub const MEDIUM: &str = "embedded://bevy_feathers/assets/fonts/FiraSans-Medium.ttf";
    /// Semibold font path
    pub const SEMIBOLD: &str = "embedded://bevy_feathers/assets/fonts/FiraSans-SemiBold.ttf";
    /// Bold font path
    pub const BOLD: &str = "embedded://bevy_feathers/assets/fonts/FiraSans-Bold.ttf";
    /// Bold italic font path
    pub const BOLD_ITALIC: &str = "embedded://bevy_feathers/assets/fonts/FiraSans-BoldItalic.ttf";
    /// Monospace font path
    pub const MONO: &str = "embedded://bevy_feathers/assets/fonts/FiraMono-Medium.ttf";
}

/// Icon paths
pub mod icons {
    /// Downward-pointing chevron
    pub const CHEVRON_DOWN: &str = "embedded://bevy_feathers/assets/icons/chevron-down.png";
    /// Right-pointing chevron
    pub const CHEVRON_RIGHT: &str = "embedded://bevy_feathers/assets/icons/chevron-right.png";
    /// Diagonal Cross
    pub const X: &str = "embedded://bevy_feathers/assets/icons/x.png";
    /// Plus sign
    pub const PLUS: &str = "embedded://bevy_feathers/assets/icons/plus.png";
    /// Funnel (filter)
    pub const FUNNEL: &str = "embedded://bevy_feathers/assets/icons/funnel.png";
    /// Horizontal ellipsis
    pub const ELLIPSIS: &str = "embedded://bevy_feathers/assets/icons/ellipsis.png";
    /// Package with a plus sign
    pub const PACKAGE_PLUS: &str = "embedded://bevy_feathers/assets/icons/package-plus.png";
    /// Video camera
    pub const VIDEO: &str = "embedded://bevy_feathers/assets/icons/video.png";
    /// Open eye
    pub const EYE: &str = "embedded://bevy_feathers/assets/icons/eye.png";
    /// Crossed-out eye
    pub const EYE_OFF: &str = "embedded://bevy_feathers/assets/icons/eye-off.png";
    /// Small dot
    pub const DOT: &str = "embedded://bevy_feathers/assets/icons/dot.png";
    /// Light bulb
    pub const LIGHTBULB: &str = "embedded://bevy_feathers/assets/icons/lightbulb.png";
    /// Isometric box
    pub const BOX: &str = "embedded://bevy_feathers/assets/icons/box.png";
    /// Isometric box with a heavier stroke, for large (30px) tiles
    pub const BOX_LARGE: &str = "embedded://bevy_feathers/assets/icons/box-large.png";
    /// Magnifying glass
    pub const SEARCH: &str = "embedded://bevy_feathers/assets/icons/search.png";
    /// File with a picture
    pub const FILE_IMAGE: &str = "embedded://bevy_feathers/assets/icons/file-image.png";
    /// Blank file
    pub const FILE: &str = "embedded://bevy_feathers/assets/icons/file.png";
    /// Blank file, stroke tuned for 12px
    pub const FILE_SMALL: &str = "embedded://bevy_feathers/assets/icons/file-small.png";
    /// Calendar with a magnifying glass
    pub const CALENDAR_SEARCH: &str = "embedded://bevy_feathers/assets/icons/calendar-search.png";
    /// Vertical drag grip (two columns of dots)
    pub const GRIP_VERTICAL: &str = "embedded://bevy_feathers/assets/icons/grip-vertical.png";
    /// Three axes with arrowheads
    pub const MOVE_3D: &str = "embedded://bevy_feathers/assets/icons/move-3d.png";
    /// Three axes with arrowheads, stroke tuned for 13px
    pub const MOVE_3D_SMALL: &str = "embedded://bevy_feathers/assets/icons/move-3d-small.png";
    /// Orbit arrows around an axis
    pub const ROTATE_3D: &str = "embedded://bevy_feathers/assets/icons/rotate-3d.png";
    /// Axes with corner boxes
    pub const SCALE_3D: &str = "embedded://bevy_feathers/assets/icons/scale-3d.png";
    /// Horseshoe magnet
    pub const MAGNET: &str = "embedded://bevy_feathers/assets/icons/magnet.png";
    /// Terminal prompt
    pub const TERMINAL: &str = "embedded://bevy_feathers/assets/icons/terminal.png";
    /// Open folder
    pub const FOLDER_OPEN: &str = "embedded://bevy_feathers/assets/icons/folder-open.png";
    /// Solid folder
    pub const FOLDER_FILLED: &str = "embedded://bevy_feathers/assets/icons/folder-filled.png";
    /// Branch with arrow (pull request)
    pub const GIT_PULL_REQUEST_ARROW: &str =
        "embedded://bevy_feathers/assets/icons/git-pull-request-arrow.png";
    /// Cross inside a circle
    pub const CIRCLE_X: &str = "embedded://bevy_feathers/assets/icons/circle-x.png";
    /// Exclamation inside a triangle
    pub const TRIANGLE_ALERT: &str = "embedded://bevy_feathers/assets/icons/triangle-alert.png";
    /// Play triangle
    pub const PLAY: &str = "embedded://bevy_feathers/assets/icons/play.png";
    /// Two pause bars
    pub const PAUSE: &str = "embedded://bevy_feathers/assets/icons/pause.png";
    /// Thin diagonal cross, for small (5px) close buttons
    pub const X_SMALL: &str = "embedded://bevy_feathers/assets/icons/x-small.png";
    /// Solid circle
    pub const CIRCLE_FILLED: &str = "embedded://bevy_feathers/assets/icons/circle-filled.png";
    /// Concave corner joining a tab to its bar
    pub const TAB_CORNER: &str = "embedded://bevy_feathers/assets/icons/tab-corner.png";
}

/// Size constants
pub mod size {
    use bevy_text::FontSize;
    use bevy_ui::Val;

    /// Common row size for buttons, sliders, spinners, etc.
    pub const ROW_HEIGHT: Val = Val::Px(24.0);

    /// Width and height of a checkbox
    pub const CHECKBOX_SIZE: Val = Val::Px(18.0);

    /// Height for pane headers
    pub const HEADER_HEIGHT: Val = Val::Px(30.0);

    /// Width and height of a radio button
    pub const RADIO_SIZE: Val = Val::Px(18.0);

    /// Width of a toggle switch
    pub const TOGGLE_WIDTH: Val = Val::Px(32.0);

    /// Height of a toggle switch
    pub const TOGGLE_HEIGHT: Val = Val::Px(18.0);

    /// Font size used for dialog box titles
    pub const HEADER_FONT: FontSize = FontSize::Px(16.0);

    /// Regular font size, used for most widget captions
    pub const MEDIUM_FONT: FontSize = FontSize::Px(14.0);

    /// Slightly smaller font size, used for text inputs
    pub const COMPACT_FONT: FontSize = FontSize::Px(13.0);

    /// Small font size
    pub const SMALL_FONT: FontSize = FontSize::Px(12.0);

    /// Extra-small font size
    pub const EXTRA_SMALL_FONT: FontSize = FontSize::Px(11.0);
}
