use ratatui::style::{Color, Modifier, Style};

// Borders & chrome
pub const BORDER:     Color = Color::Rgb(100, 160, 200);
pub const TITLE:      Color = Color::Rgb(220, 220, 255);

// Text
pub const TEXT:       Color = Color::Rgb(220, 220, 220);
pub const TEXT_DIM:   Color = Color::Rgb(110, 110, 130);
pub const TEXT_FAINT: Color = Color::Rgb(70, 70, 90);

// Selection
pub const ROW_SELECTED_BG: Color = Color::Rgb(80, 60, 100);
pub const TEXT_HIGHLIGHT:  Color = Color::Rgb(255, 220, 255);

// Bars: blue (low) → pink (mid) → red (high)
pub const BAR_LOW:  Color = Color::Rgb(80, 180, 220);
pub const BAR_MED:  Color = Color::Rgb(210, 100, 160);
pub const BAR_HIGH: Color = Color::Rgb(220, 50, 80);
pub const GAUGE_BG: Color = Color::Rgb(40, 40, 55);

// Activity sidebar
pub const ACTIVITY_UP:     Color = Color::Rgb(220, 80, 100);
pub const ACTIVITY_DOWN:   Color = Color::Rgb(80, 200, 160);
pub const ACTIVITY_STABLE: Color = Color::Rgb(100, 100, 120);

// Reserved
#[allow(dead_code)] pub const BORDER_DIM: Color = Color::Rgb(50, 60, 80);
#[allow(dead_code)] pub const ROW_ALT_BG: Color = Color::Reset;

// Style builders
#[allow(dead_code)]
#[inline] pub fn default_style() -> Style { Style::default().fg(TEXT) }

#[inline] pub fn header_style() -> Style {
    Style::default().fg(TEXT).add_modifier(Modifier::BOLD)
}
#[inline] pub fn dim_style() -> Style { Style::default().fg(TEXT_DIM) }

#[allow(dead_code)]
#[inline] pub fn faint_style() -> Style { Style::default().fg(TEXT_FAINT) }

#[inline] pub fn selected_style() -> Style {
    Style::default()
        .fg(TEXT_HIGHLIGHT)
        .bg(ROW_SELECTED_BG)
        .add_modifier(Modifier::BOLD)
}
#[inline] pub fn border_style() -> Style { Style::default().fg(BORDER) }
#[inline] pub fn title_style() -> Style {
    Style::default().fg(TITLE).add_modifier(Modifier::BOLD)
}

/// Bar colour by load fraction. Blue (calm) → pink → red (critical).
#[inline] pub fn gauge_color(fraction: f64) -> Color {
    if fraction < 0.5      { BAR_LOW  }
    else if fraction < 0.8 { BAR_MED  }
    else                   { BAR_HIGH }
}

/// Process name fades from bright → dim → faint as rank increases.
/// Rank 0 = top process. Mirrors btop's visual fade effect.
#[inline] pub fn process_name_color(rank: usize) -> Color {
    if rank < 3       { TEXT       }
    else if rank < 10 { TEXT_DIM   }
    else              { TEXT_FAINT }
}
