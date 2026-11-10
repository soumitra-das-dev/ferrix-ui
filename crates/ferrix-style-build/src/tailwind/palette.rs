//! Standard Tailwind CSS color palette (v3/v4) and color resolver.

use std::collections::HashMap;
use std::sync::OnceLock;

static PALETTE: OnceLock<HashMap<(&'static str, u16), u32>> = OnceLock::new();

fn init_palette() -> HashMap<(&'static str, u16), u32> {
    let mut m = HashMap::new();

    // Helper macro to register shades
    macro_rules! add_shades {
        ($name:expr, [
            $s50:expr, $s100:expr, $s200:expr, $s300:expr, $s400:expr,
            $s500:expr, $s600:expr, $s700:expr, $s800:expr, $s900:expr, $s950:expr
        ]) => {
            m.insert(($name, 50), $s50);
            m.insert(($name, 100), $s100);
            m.insert(($name, 200), $s200);
            m.insert(($name, 300), $s300);
            m.insert(($name, 400), $s400);
            m.insert(($name, 500), $s500);
            m.insert(($name, 600), $s600);
            m.insert(($name, 700), $s700);
            m.insert(($name, 800), $s800);
            m.insert(($name, 900), $s900);
            m.insert(($name, 950), $s950);
        };
    }

    // Slate
    add_shades!(
        "slate",
        [
            0xFFF8FAFC, 0xFFF1F5F9, 0xFFE2E8F0, 0xFFCBD5E1, 0xFF94A3B8, 0xFF64748B, 0xFF475569,
            0xFF334155, 0xFF1E293B, 0xFF0F172A, 0xFF020617
        ]
    );

    // Gray
    add_shades!(
        "gray",
        [
            0xFFF9FAFB, 0xFFF3F4F6, 0xFFE5E7EB, 0xFFD1D5DB, 0xFF9CA3AF, 0xFF6B7280, 0xFF4B5563,
            0xFF374151, 0xFF1F2937, 0xFF111827, 0xFF030712
        ]
    );

    // Zinc
    add_shades!(
        "zinc",
        [
            0xFFFAFAFA, 0xFFF4F4F5, 0xFFE4E4E7, 0xFFD4D4D8, 0xFFA1A1AA, 0xFF71717A, 0xFF52525B,
            0xFF3F3F46, 0xFF27272A, 0xFF18181B, 0xFF09090B
        ]
    );

    // Neutral
    add_shades!(
        "neutral",
        [
            0xFFFAFAFA, 0xFFF5F5F5, 0xFFE5E5E5, 0xFFD4D4D4, 0xFFA3A3A3, 0xFF737373, 0xFF525252,
            0xFF404040, 0xFF262626, 0xFF171717, 0xFF0A0A0A
        ]
    );

    // Red
    add_shades!(
        "red",
        [
            0xFFFEF2F2, 0xFFFEE2E2, 0xFFFECACA, 0xFFFCA5A5, 0xFFF87171, 0xFFEF4444, 0xFFDC2626,
            0xFFB91C1C, 0xFF991B1B, 0xFF7F1D1D, 0xFF450A0A
        ]
    );

    // Orange
    add_shades!(
        "orange",
        [
            0xFFFFF7ED, 0xFFFFEDD5, 0xFFFED7AA, 0xFFFDBA74, 0xFFFB923C, 0xFFF97316, 0xFFEA580C,
            0xFFC2410C, 0xFF9A3412, 0xFF7C2D12, 0xFF431407
        ]
    );

    // Amber
    add_shades!(
        "amber",
        [
            0xFFFFFBEB, 0xFFFEF3C7, 0xFFFDE68A, 0xFFFCD34D, 0xFFFBBF24, 0xFFF59E0B, 0xFFD97706,
            0xFFB45309, 0xFF92400E, 0xFF78350F, 0xFF451A03
        ]
    );

    // Yellow
    add_shades!(
        "yellow",
        [
            0xFFFEFCE8, 0xFFFEF9C3, 0xFFFEF08A, 0xFFFDE047, 0xFFFACC15, 0xFFEAB308, 0xFFCA8A04,
            0xFFA16207, 0xFF854D0E, 0xFF713F12, 0xFF422006
        ]
    );

    // Green
    add_shades!(
        "green",
        [
            0xFFF0FDF4, 0xFFDCFCE7, 0xFFBBF7D0, 0xFF86EFAC, 0xFF4ADE80, 0xFF22C55E, 0xFF16A34A,
            0xFF15803D, 0xFF166534, 0xFF14532D, 0xFF052E16
        ]
    );

    // Emerald
    add_shades!(
        "emerald",
        [
            0xFFECFDF5, 0xFFD1FAE5, 0xFFA7F3D0, 0xFF6EE7B7, 0xFF34D399, 0xFF10B981, 0xFF059669,
            0xFF047857, 0xFF065F46, 0xFF064E3B, 0xFF022C22
        ]
    );

    // Teal
    add_shades!(
        "teal",
        [
            0xFFF0FDFA, 0xFFCCFBF1, 0xFF99F6E4, 0xFF5EEAD4, 0xFF2DD4BF, 0xFF14B8A6, 0xFF0D9488,
            0xFF0F766E, 0xFF115E59, 0xFF134E4A, 0xFF042F2E
        ]
    );

    // Cyan
    add_shades!(
        "cyan",
        [
            0xFFECFEFF, 0xFFCFFAFE, 0xFFA5F3FC, 0xFF67E8F9, 0xFF22D3EE, 0xFF06B6D4, 0xFF0891B2,
            0xFF0E7490, 0xFF155E75, 0xFF164E63, 0xFF083344
        ]
    );

    // Sky
    add_shades!(
        "sky",
        [
            0xFFF0F9FF, 0xFFE0F2FE, 0xFFBAE6FD, 0xFF7DD3FC, 0xFF38BDF8, 0xFF0EA5E9, 0xFF0284C7,
            0xFF0369A1, 0xFF075985, 0xFF0C4A6E, 0xFF082F49
        ]
    );

    // Blue
    add_shades!(
        "blue",
        [
            0xFFEFF6FF, 0xFFDBEAFE, 0xFFBFDBFE, 0xFF93C5FD, 0xFF60A5FA, 0xFF3B82F6, 0xFF2563EB,
            0xFF1D4ED8, 0xFF1E40AF, 0xFF1E3A8A, 0xFF172554
        ]
    );

    // Indigo
    add_shades!(
        "indigo",
        [
            0xFFEEF2FF, 0xFFE0E7FF, 0xFFC7D2FE, 0xFFA5B4FC, 0xFF818CF8, 0xFF6366F1, 0xFF4F46E5,
            0xFF4338CA, 0xFF3730A3, 0xFF312E81, 0xFF1E1B4B
        ]
    );

    // Violet
    add_shades!(
        "violet",
        [
            0xFFF5F3FF, 0xFFEDE9FE, 0xFFDDD6FE, 0xFFC4B5FD, 0xFFA78BFA, 0xFF8B5CF6, 0xFF7C3AED,
            0xFF6D28D9, 0xFF5B21B6, 0xFF4C1D95, 0xFF2E1065
        ]
    );

    // Purple
    add_shades!(
        "purple",
        [
            0xFFFAF5FF, 0xFFF3E8FF, 0xFFE9D5FF, 0xFFD8B4FE, 0xFFC084FC, 0xFFA855F7, 0xFF9333EA,
            0xFF7E22CE, 0xFF6B21A8, 0xFF581C87, 0xFF3B0764
        ]
    );

    // Fuchsia
    add_shades!(
        "fuchsia",
        [
            0xFFFDF4FF, 0xFFFAE8FF, 0xFFF5D0FE, 0xFFF0ABFC, 0xFFE879F9, 0xFFD946EF, 0xFFC026D3,
            0xFFA21CAF, 0xFF86198F, 0xFF701A75, 0xFF4A044E
        ]
    );

    // Pink
    add_shades!(
        "pink",
        [
            0xFFFDF2F8, 0xFFFCE7F3, 0xFFFBCFE8, 0xFFF9A8D4, 0xFFF472B6, 0xFFEC4899, 0xFFDB2777,
            0xFFBE185D, 0xFF9D174D, 0xFF831843, 0xFF500724
        ]
    );

    // Rose
    add_shades!(
        "rose",
        [
            0xFFFFF1F2, 0xFFFFE4E6, 0xFFFECDD3, 0xFFFDA4AF, 0xFFFB7185, 0xFFF43F5E, 0xFFE11D48,
            0xFFBE123C, 0xFF9F1239, 0xFF881337, 0xFF4C0519
        ]
    );

    m
}

/// Resolves a Tailwind color name and optional shade into a 32-bit ARGB color.
pub fn resolve_tailwind_color(name: &str, shade: Option<u16>) -> Option<u32> {
    match name {
        "white" => return Some(0xFFFFFFFF),
        "black" => return Some(0xFF000000),
        "transparent" => return Some(0x00000000),
        _ => {}
    }

    let shade = shade.unwrap_or(500);
    let palette = PALETTE.get_or_init(init_palette);
    palette.get(&(name, shade)).copied()
}
