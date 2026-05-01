/// The cursor grab mode of a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorGrabMode {
    /// The cursor is not grabbed.
    #[default]
    None,

    /// The cursor is confined to the window area.
    Confined,

    /// The cursor is locked to the window.
    Locked,
}
