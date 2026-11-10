//! Navigation options and presentation styles.

use serde::{Deserialize, Serialize};

/// Visual presentation style for screens.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PresentationStyle {
    #[default]
    Card = 0,
    Modal = 1,
    TransparentModal = 2,
}

impl PresentationStyle {
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(val: u8) -> Self {
        match val {
            1 => Self::Modal,
            2 => Self::TransparentModal,
            _ => Self::Card,
        }
    }
}

/// Transition animation for screen navigation.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TransitionAnimation {
    #[default]
    Default = 0,
    Fade = 1,
    SlideFromRight = 2,
    SlideFromBottom = 3,
    None = 4,
}

impl TransitionAnimation {
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(val: u8) -> Self {
        match val {
            1 => Self::Fade,
            2 => Self::SlideFromRight,
            3 => Self::SlideFromBottom,
            4 => Self::None,
            _ => Self::Default,
        }
    }
}

/// Screen configuration options configuring native header and presentation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenOptions {
    pub title: String,
    pub header_shown: bool,
    pub presentation: PresentationStyle,
    pub animation: TransitionAnimation,
}

impl Default for ScreenOptions {
    fn default() -> Self {
        Self {
            title: String::new(),
            header_shown: true,
            presentation: PresentationStyle::Card,
            animation: TransitionAnimation::Default,
        }
    }
}
