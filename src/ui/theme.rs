use eframe::egui::{self, Color32, Rounding, Stroke, Style, Visuals};

/// Spotify-inspired dark theme colors
pub struct SpotifyTheme {
    // Main colors
    pub background: Color32,
    pub surface: Color32,
    pub surface_variant: Color32,
    pub primary: Color32,
    pub secondary: Color32,
    pub accent: Color32,
    
    // Text colors
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_disabled: Color32,
    
    // Interaction colors
    pub hover: Color32,
    pub selected: Color32,
    pub border: Color32,
}

impl Default for SpotifyTheme {
    fn default() -> Self {
        Self {
            // Spotify-like dark theme
            background: Color32::from_rgb(18, 18, 18),        // Very dark background
            surface: Color32::from_rgb(33, 33, 33),           // Card/panel background
            surface_variant: Color32::from_rgb(45, 45, 45),   // Elevated surfaces
            primary: Color32::from_rgb(29, 185, 84),          // Spotify green
            secondary: Color32::from_rgb(179, 179, 179),      // Muted text
            accent: Color32::from_rgb(255, 255, 255),         // White accent
            
            text_primary: Color32::from_rgb(255, 255, 255),   // Primary text
            text_secondary: Color32::from_rgb(179, 179, 179), // Secondary text
            text_disabled: Color32::from_rgb(117, 117, 117),  // Disabled text
            
            hover: Color32::from_rgba_premultiplied(255, 255, 255, 10), // Subtle hover
            selected: Color32::from_rgba_premultiplied(29, 185, 84, 20), // Selected state
            border: Color32::from_rgb(60, 60, 60),            // Border color
        }
    }
}

impl SpotifyTheme {
    /// Apply the Spotify theme to egui context
    pub fn apply_to_context(ctx: &egui::Context) {
        let theme = Self::default();
        
        ctx.set_style({
            let mut style = Style::default();
            
            // Set dark visuals
            style.visuals = Visuals {
                dark_mode: true,
                override_text_color: Some(theme.text_primary),
                hyperlink_color: theme.primary,
                faint_bg_color: theme.surface,
                extreme_bg_color: theme.background,
                code_bg_color: theme.surface_variant,
                warn_fg_color: Color32::from_rgb(255, 143, 0),
                error_fg_color: Color32::from_rgb(255, 69, 58),
                window_fill: theme.background,
                panel_fill: theme.surface,
                window_stroke: Stroke::new(1.0, theme.border),
                menu_rounding: Rounding::same(8.0),
                window_rounding: Rounding::same(12.0),
                ..Visuals::dark()
            };
            
            // Configure spacing for better layout
            style.spacing.item_spacing = egui::vec2(8.0, 8.0);
            style.spacing.button_padding = egui::vec2(16.0, 8.0);
            style.spacing.menu_margin = egui::style::Margin::same(8.0);
            style.spacing.indent = 20.0;
            style.spacing.window_margin = egui::style::Margin::same(8.0);
            
            style
        });
    }
    
    /// Get button style with Spotify-like appearance
    pub fn styled_button(text: &str, primary: bool) -> egui::Button {
        let mut button = egui::Button::new(text);
        
        if primary {
            // Primary button (Spotify green)
            button = button.fill(SpotifyTheme::default().primary);
        } else {
            // Secondary button (transparent with border)
            button = button.fill(Color32::TRANSPARENT)
                .stroke(Stroke::new(1.0, SpotifyTheme::default().border));
        }
        
        button
    }
    
    /// Get a card-style frame
    pub fn card_frame() -> egui::Frame {
        egui::Frame::default()
            .fill(SpotifyTheme::default().surface)
            .rounding(Rounding::same(12.0))
            .stroke(Stroke::new(1.0, SpotifyTheme::default().border))
            .inner_margin(egui::style::Margin::same(16.0))
    }
    
    /// Get a sidebar frame
    pub fn sidebar_frame() -> egui::Frame {
        egui::Frame::default()
            .fill(SpotifyTheme::default().background)
            .inner_margin(egui::style::Margin::same(12.0))
    }
}