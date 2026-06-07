use ratatui::style::{Color, Modifier, Style};

pub const BORDER: Color = Color::Cyan;

#[allow(dead_code)]
pub const BORDER_DIM: Color = Color::DarkGray;

pub const TITLE: Color = Color::White;

pub const TEXT: Color = Color::White;

pub const TEXT_DIM: Color = Color::Gray;

pub const TEXT_HIGHLIGHT: Color = Color::Black;

pub const GAUGE_LOW: Color = Color::Green;

pub const GAUGE_MED: Color = Color::Yellow;

pub const GAUGE_HIGH: Color = Color::Red;

pub const GAUGE_BG: Color = Color::DarkGray;

pub const ROW_SELECTED_BG: Color = Color::Cyan;

#[allow(dead_code)]
pub const ROW_ALT_BG: Color = Color::Reset;

pub const ACTIVITY_UP: Color = Color::LightRed;

pub const ACTIVITY_DOWN: Color = Color::LightGreen;

pub const ACTIVITY_STABLE: Color = Color::Gray;

#[allow(dead_code)]
#[inline]
pub fn default_style() -> Style {
    Style::default().fg(TEXT)
}

#[inline]
pub fn header_style() -> Style {
    Style::default().fg(TEXT).add_modifier(Modifier::BOLD)
}

#[inline]
pub fn dim_style() -> Style {
    Style::default().fg(TEXT_DIM)
}

#[inline]
pub fn selected_style() -> Style {
    Style::default()
        .fg(TEXT_HIGHLIGHT)
        .bg(ROW_SELECTED_BG)
        .add_modifier(Modifier::BOLD)
}

#[inline]
pub fn border_style() -> Style {
    Style::default().fg(BORDER)
}

#[inline]
pub fn title_style() -> Style {
    Style::default().fg(TITLE).add_modifier(Modifier::BOLD)
}

#[inline]
pub fn gauge_color(fraction: f64) -> Color {
    if fraction < 0.5 {
        GAUGE_LOW
    } else if fraction < 0.8 {
        GAUGE_MED
    } else {
        GAUGE_HIGH
    }
}
