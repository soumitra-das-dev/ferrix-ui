//! Color normalization and parsing utilities for CSS hex, rgb, rgba, hsl, oklch, and color-mix.

/// Color parser producing 32-bit ARGB `u32` compatible with native Android color representation.
pub struct Color;

impl Color {
    pub const TRANSPARENT: u32 = 0x00000000;
    pub const BLACK: u32 = 0xFF000000;
    pub const WHITE: u32 = 0xFFFFFFFF;
    pub const RED: u32 = 0xFFFF0000;
    pub const GREEN: u32 = 0xFF00FF00;
    pub const BLUE: u32 = 0xFF0000FF;
    pub const GRAY: u32 = 0xFF808080;
    pub const LIGHT_GRAY: u32 = 0xFFE5E7EB;
    pub const DARK_GRAY: u32 = 0xFF1F2937;
    pub const PRIMARY: u32 = 0xFF2563EB;

    /// Creates an ARGB color from RGBA components.
    #[inline]
    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> u32 {
        let alpha = ((a * 255.0).clamp(0.0, 255.0)) as u32;
        (alpha << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    /// Creates an opaque ARGB color from RGB components.
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
        Self::rgba(r, g, b, 1.0)
    }

    /// Parses a CSS hex string into ARGB `u32` (e.g. "#RGB", "#RGBA", "#RRGGBB", "#RRGGBBAA").
    pub fn hex(s: &str) -> u32 {
        Self::parse(s).unwrap_or(Self::BLACK)
    }

    /// Converts HSL to ARGB `u32`.
    /// h in [0, 360], s in [0, 1], l in [0, 1], a in [0, 1]
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> u32 {
        let h = (h % 360.0 + 360.0) % 360.0;
        let s = s.clamp(0.0, 1.0);
        let l = l.clamp(0.0, 1.0);

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r_prime, g_prime, b_prime) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let r = ((r_prime + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = ((g_prime + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = ((b_prime + m) * 255.0).round().clamp(0.0, 255.0) as u8;

        Self::rgba(r, g, b, a)
    }

    /// Converts OKLCH to sRGB ARGB `u32`.
    /// l in [0, 1], c in [0, 0.4], h in [0, 360], a in [0, 1]
    #[allow(clippy::excessive_precision)]
    pub fn oklch(l: f32, c: f32, h: f32, a: f32) -> u32 {
        let h_rad = h.to_radians();
        let a_comp = c * h_rad.cos();
        let b_comp = c * h_rad.sin();

        // OKLab to linear sRGB approximation
        let l_ = l + 0.3963377774 * a_comp + 0.2158037573 * b_comp;
        let m_ = l - 0.1055613458 * a_comp - 0.0638541728 * b_comp;
        let s_ = l - 0.0894841775 * a_comp - 1.2914855480 * b_comp;

        let l3 = l_ * l_ * l_;
        let m3 = m_ * m_ * m_;
        let s3 = s_ * s_ * s_;

        let r_lin = 4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3;
        let g_lin = -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3;
        let b_lin = -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3;

        #[inline]
        fn to_srgb(x: f32) -> u8 {
            let clamped = x.clamp(0.0, 1.0);
            let srgb = if clamped <= 0.0031308 {
                12.92 * clamped
            } else {
                1.055 * clamped.powf(1.0 / 2.4) - 0.055
            };
            (srgb * 255.0).round().clamp(0.0, 255.0) as u8
        }

        Self::rgba(to_srgb(r_lin), to_srgb(g_lin), to_srgb(b_lin), a)
    }

    /// Mixes two colors linearly by percentage in sRGB (color-mix equivalent).
    pub fn mix(color1: u32, color2: u32, weight: f32) -> u32 {
        let w = weight.clamp(0.0, 1.0);
        let inv_w = 1.0 - w;

        let a1 = ((color1 >> 24) & 0xFF) as f32;
        let r1 = ((color1 >> 16) & 0xFF) as f32;
        let g1 = ((color1 >> 8) & 0xFF) as f32;
        let b1 = (color1 & 0xFF) as f32;

        let a2 = ((color2 >> 24) & 0xFF) as f32;
        let r2 = ((color2 >> 16) & 0xFF) as f32;
        let g2 = ((color2 >> 8) & 0xFF) as f32;
        let b2 = (color2 & 0xFF) as f32;

        let a = (a1 * inv_w + a2 * w).round().clamp(0.0, 255.0) as u32;
        let r = (r1 * inv_w + r2 * w).round().clamp(0.0, 255.0) as u32;
        let g = (g1 * inv_w + g2 * w).round().clamp(0.0, 255.0) as u32;
        let b = (b1 * inv_w + b2 * w).round().clamp(0.0, 255.0) as u32;

        (a << 24) | (r << 16) | (g << 8) | b
    }

    /// Parses a CSS color string (hex, named, rgb, rgba, hsl, hsla, oklch).
    pub fn parse(s: &str) -> Option<u32> {
        let s = s.trim();
        if let Some(hex) = s.strip_prefix('#') {
            match hex.len() {
                3 => {
                    let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                    let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                    let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                    Some(Self::rgb(r * 17, g * 17, b * 17))
                }
                4 => {
                    let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                    let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                    let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                    let a = u8::from_str_radix(&hex[3..4], 16).ok()?;
                    Some(Self::rgba(r * 17, g * 17, b * 17, (a * 17) as f32 / 255.0))
                }
                6 => {
                    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                    Some(Self::rgb(r, g, b))
                }
                8 => {
                    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                    let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                    Some(Self::rgba(r, g, b, a as f32 / 255.0))
                }
                _ => None,
            }
        } else if s.starts_with("rgb(") || s.starts_with("rgba(") {
            let inner = s
                .strip_prefix("rgba(")
                .or_else(|| s.strip_prefix("rgb("))?
                .strip_suffix(')')?;
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 3 {
                let r = parts[0].trim().parse::<u8>().ok()?;
                let g = parts[1].trim().parse::<u8>().ok()?;
                let b = parts[2].trim().parse::<u8>().ok()?;
                Some(Self::rgb(r, g, b))
            } else if parts.len() == 4 {
                let r = parts[0].trim().parse::<u8>().ok()?;
                let g = parts[1].trim().parse::<u8>().ok()?;
                let b = parts[2].trim().parse::<u8>().ok()?;
                let a = parts[3].trim().parse::<f32>().ok()?;
                Some(Self::rgba(r, g, b, a))
            } else {
                None
            }
        } else if s.starts_with("hsl(") || s.starts_with("hsla(") {
            let inner = s
                .strip_prefix("hsla(")
                .or_else(|| s.strip_prefix("hsl("))?
                .strip_suffix(')')?;
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() >= 3 {
                let h = parts[0]
                    .trim()
                    .trim_end_matches("deg")
                    .parse::<f32>()
                    .ok()?;
                let s_val = parts[1].trim().trim_end_matches('%').parse::<f32>().ok()? / 100.0;
                let l_val = parts[2].trim().trim_end_matches('%').parse::<f32>().ok()? / 100.0;
                let a_val = if parts.len() == 4 {
                    parts[3].trim().parse::<f32>().ok()?
                } else {
                    1.0
                };
                Some(Self::hsla(h, s_val, l_val, a_val))
            } else {
                None
            }
        } else if s.starts_with("oklch(") {
            let inner = s.strip_prefix("oklch(")?.strip_suffix(')')?;
            // Format: oklch(L C H [/ A]) or comma separated
            let (main_part, alpha_part) = if let Some((m, a)) = inner.split_once('/') {
                (m.trim(), Some(a.trim()))
            } else {
                (inner.trim(), None)
            };
            let parts: Vec<&str> = if main_part.contains(',') {
                main_part.split(',').collect()
            } else {
                main_part.split_whitespace().collect()
            };
            if parts.len() >= 3 {
                let l = if parts[0].ends_with('%') {
                    parts[0].trim_end_matches('%').parse::<f32>().ok()? / 100.0
                } else {
                    parts[0].parse::<f32>().ok()?
                };
                let c = parts[1].parse::<f32>().ok()?;
                let h = parts[2].trim_end_matches("deg").parse::<f32>().ok()?;
                let a = if let Some(a_str) = alpha_part {
                    if a_str.ends_with('%') {
                        a_str.trim_end_matches('%').parse::<f32>().ok()? / 100.0
                    } else {
                        a_str.parse::<f32>().ok()?
                    }
                } else {
                    1.0
                };
                Some(Self::oklch(l, c, h, a))
            } else {
                None
            }
        } else {
            match s.to_lowercase().as_str() {
                "transparent" => Some(Self::TRANSPARENT),
                "black" => Some(Self::BLACK),
                "white" => Some(Self::WHITE),
                "red" => Some(Self::RED),
                "green" => Some(Self::GREEN),
                "blue" => Some(Self::BLUE),
                "gray" | "grey" => Some(Self::GRAY),
                "slate-900" => Some(0xFF0F172A),
                "slate-800" => Some(0xFF1E293B),
                "slate-100" => Some(0xFFF1F5F9),
                "blue-600" => Some(0xFF2563EB),
                "emerald-500" => Some(0xFF10B981),
                _ => None,
            }
        }
    }
}
