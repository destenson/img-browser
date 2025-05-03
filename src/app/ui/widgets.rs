
//! Platform-independent UI widgets for media management.

/// Trait for a generic UI widget.
pub trait Widget {
    /// Render the widget (platform-specific implementation).
    fn render(&mut self);
    /// Handle input/events (platform-specific implementation).
    fn handle_event(&mut self, event: &WidgetEvent);
}

/// Basic event type for widgets.
pub enum WidgetEvent {
    // ...expand as needed...
    Click,
    Hover,
    KeyPress(char),
    // etc.
}

/// Widget for displaying a single image.
pub struct ImageView {
    pub image_path: String,
    // Add more fields as needed (zoom, pan, etc.)
}

impl Widget for ImageView {
    fn render(&mut self) {
        // Platform-specific rendering handled elsewhere.
    }
    fn handle_event(&mut self, _event: &WidgetEvent) {
        // Handle image-specific events.
    }
}

/// Widget for displaying a video.
pub struct VideoView {
    pub video_path: String,
    // Add more fields as needed (playback state, controls, etc.)
}

impl Widget for VideoView {
    fn render(&mut self) {}
    fn handle_event(&mut self, _event: &WidgetEvent) {}
}

/// Widget for categorizing and tagging media.
pub struct TaggingWidget {
    pub tags: Vec<String>,
    // Add more fields as needed (suggestions, editing state, etc.)
}

impl Widget for TaggingWidget {
    fn render(&mut self) {}
    fn handle_event(&mut self, _event: &WidgetEvent) {}
}

/// Widget for navigating projects or directories.
pub struct NavigatorWidget {
    pub current_path: String,
    pub entries: Vec<String>,
    // Add more fields as needed (history, selection, etc.)
}

impl Widget for NavigatorWidget {
    fn render(&mut self) {}
    fn handle_event(&mut self, _event: &WidgetEvent) {}
}

/// Widget for sorting and filtering media.
pub struct SortFilterWidget {
    pub sort_by: SortBy,
    pub ascending: bool,
    // Add more fields as needed (filters, search, etc.)
}

pub enum SortBy {
    Name,
    Date,
    Size,
    // Extend as needed.
}

impl Widget for SortFilterWidget {
    fn render(&mut self) {}
    fn handle_event(&mut self, _event: &WidgetEvent) {}
}

// ...add more widgets as needed...
