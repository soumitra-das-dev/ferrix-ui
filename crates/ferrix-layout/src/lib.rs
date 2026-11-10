//! # ferrix-layout
//!
//! Taffy-based layout engine for Ferrix under Architecture A.
//! Computes layout geometry in Rust (single source of truth) which is applied directly
//! by platform renderers without competing platform layout passes.

use ferrix_renderer_protocol::{NodeId, TextMeasureRequest, TextMeasureResponse};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
pub use taffy::prelude::{
    AlignContent, AlignItems, AlignSelf, AvailableSpace, Dimension, Display, FlexDirection,
    FlexWrap, JustifyContent, LengthPercentage, LengthPercentageAuto, Position, Rect, Size, Style,
    TaffyTree,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LayoutError {
    #[error("Taffy layout error: {0}")]
    Taffy(#[from] taffy::TaffyError),
    #[error("Node not found in layout tree: {0:?}")]
    NodeNotFound(NodeId),
}

/// Cache key for intrinsic text measurement lookups.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextMeasureKey {
    pub text_hash: u64,
    pub font_size_sp_bits: u32,
    pub font_weight: u16,
    pub font_style: u8,
    pub max_width_bits: u32,
}

impl TextMeasureKey {
    pub fn new(
        text: &str,
        font_size_sp: f32,
        font_weight: u16,
        font_style: u8,
        max_width: f32,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        Self {
            text_hash: hasher.finish(),
            font_size_sp_bits: font_size_sp.to_bits(),
            font_weight,
            font_style,
            max_width_bits: max_width.to_bits(),
        }
    }
}

/// Thread-safe in-memory cache for native text measurements.
#[derive(Clone, Debug, Default)]
pub struct TextMeasureCache {
    cache: HashMap<TextMeasureKey, (f32, f32)>,
    pending_requests: HashMap<u32, (TextMeasureKey, NodeId)>,
    next_request_id: u32,
}

impl TextMeasureCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            pending_requests: HashMap::new(),
            next_request_id: 1,
        }
    }

    pub fn get(
        &self,
        text: &str,
        font_size_sp: f32,
        font_weight: u16,
        font_style: u8,
        max_width: f32,
    ) -> Option<(f32, f32)> {
        let key = TextMeasureKey::new(text, font_size_sp, font_weight, font_style, max_width);
        self.cache.get(&key).copied()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert(
        &mut self,
        text: &str,
        font_size_sp: f32,
        font_weight: u16,
        font_style: u8,
        max_width: f32,
        width: f32,
        height: f32,
    ) {
        let key = TextMeasureKey::new(text, font_size_sp, font_weight, font_style, max_width);
        self.cache.insert(key, (width, height));
    }

    /// Prepares a batch request for an unmeasured text entry.
    pub fn queue_request(
        &mut self,
        node_id: NodeId,
        text: String,
        font_size_sp: f32,
        font_weight: u16,
        font_style: u8,
        max_width: f32,
    ) -> TextMeasureRequest {
        let req_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);

        let key = TextMeasureKey::new(&text, font_size_sp, font_weight, font_style, max_width);
        self.pending_requests.insert(req_id, (key, node_id));

        TextMeasureRequest {
            request_id: req_id,
            text,
            font_size_sp,
            font_weight,
            font_style,
            max_width,
        }
    }

    /// Ingests measured responses from Kotlin.
    pub fn apply_responses(&mut self, responses: &[TextMeasureResponse]) {
        for resp in responses {
            if let Some((key, _)) = self.pending_requests.remove(&resp.request_id) {
                self.cache.insert(key, (resp.width, resp.height));
            }
        }
    }

    /// Fast typographic estimation heuristic used when native measurement is pending.
    pub fn estimate(
        text: &str,
        font_size_sp: f32,
        _font_weight: u16,
        _font_style: u8,
        max_width: f32,
    ) -> (f32, f32) {
        let char_count = text.chars().count();
        // Standard Latin character aspect ratio is ~0.55 width to height
        let approx_char_w = font_size_sp * 0.55;
        let line_height = font_size_sp * 1.25;
        let single_line_w = char_count as f32 * approx_char_w;

        if max_width > 0.0 && single_line_w > max_width {
            let lines = (single_line_w / max_width).ceil();
            (max_width, lines * line_height)
        } else {
            (single_line_w, line_height)
        }
    }
}

/// Computed geometry for a layout node in physical / scaled pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedLayout {
    pub node_id: NodeId,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Flexible dimension specification.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DimensionValue {
    #[default]
    Auto,
    Points(f32),
    Percent(f32),
}

impl DimensionValue {
    pub fn to_taffy_dimension(self) -> Dimension {
        match self {
            Self::Auto => Dimension::Auto,
            Self::Points(pts) => Dimension::Length(pts),
            Self::Percent(pct) => Dimension::Percent(pct / 100.0),
        }
    }

    pub fn to_taffy_length_percentage_auto(self) -> LengthPercentageAuto {
        match self {
            Self::Auto => LengthPercentageAuto::Auto,
            Self::Points(pts) => LengthPercentageAuto::Length(pts),
            Self::Percent(pct) => LengthPercentageAuto::Percent(pct / 100.0),
        }
    }

    pub fn to_taffy_length_percentage(self) -> LengthPercentage {
        match self {
            Self::Auto => LengthPercentage::Length(0.0),
            Self::Points(pts) => LengthPercentage::Length(pts),
            Self::Percent(pct) => LengthPercentage::Percent(pct / 100.0),
        }
    }
}

/// High-level flexbox style definition for a Ferrix element.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutStyle {
    pub flex_direction: FlexDirection,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: DimensionValue,
    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignSelf>,
    pub align_content: Option<AlignContent>,
    pub justify_content: Option<JustifyContent>,
    pub flex_wrap: FlexWrap,
    pub gap: (f32, f32), // (row_gap, column_gap)
    pub position: Position,
    pub left: DimensionValue,
    pub right: DimensionValue,
    pub top: DimensionValue,
    pub bottom: DimensionValue,
    pub start: DimensionValue,
    pub end: DimensionValue,
    pub width: DimensionValue,
    pub height: DimensionValue,
    pub min_width: DimensionValue,
    pub max_width: DimensionValue,
    pub min_height: DimensionValue,
    pub max_height: DimensionValue,
    pub aspect_ratio: Option<f32>,
    pub margin_left: DimensionValue,
    pub margin_right: DimensionValue,
    pub margin_top: DimensionValue,
    pub margin_bottom: DimensionValue,
    pub margin_start: DimensionValue,
    pub margin_end: DimensionValue,
    pub padding_left: DimensionValue,
    pub padding_right: DimensionValue,
    pub padding_top: DimensionValue,
    pub padding_bottom: DimensionValue,
    pub padding_start: DimensionValue,
    pub padding_end: DimensionValue,
    pub display: Display,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            flex_direction: FlexDirection::Column,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: DimensionValue::Auto,
            align_items: None,
            align_self: None,
            align_content: None,
            justify_content: None,
            flex_wrap: FlexWrap::NoWrap,
            gap: (0.0, 0.0),
            position: Position::Relative,
            left: DimensionValue::Auto,
            right: DimensionValue::Auto,
            top: DimensionValue::Auto,
            bottom: DimensionValue::Auto,
            start: DimensionValue::Auto,
            end: DimensionValue::Auto,
            width: DimensionValue::Auto,
            height: DimensionValue::Auto,
            min_width: DimensionValue::Auto,
            max_width: DimensionValue::Auto,
            min_height: DimensionValue::Auto,
            max_height: DimensionValue::Auto,
            aspect_ratio: None,
            margin_left: DimensionValue::Points(0.0),
            margin_right: DimensionValue::Points(0.0),
            margin_top: DimensionValue::Points(0.0),
            margin_bottom: DimensionValue::Points(0.0),
            margin_start: DimensionValue::Auto,
            margin_end: DimensionValue::Auto,
            padding_left: DimensionValue::Points(0.0),
            padding_right: DimensionValue::Points(0.0),
            padding_top: DimensionValue::Points(0.0),
            padding_bottom: DimensionValue::Points(0.0),
            padding_start: DimensionValue::Auto,
            padding_end: DimensionValue::Auto,
            display: Display::Flex,
        }
    }
}

impl LayoutStyle {
    /// Converts this `LayoutStyle` into a Taffy `Style`, resolving `is_rtl` direction.
    pub fn to_taffy_style(&self, is_rtl: bool) -> Style {
        let (resolved_left, resolved_right) = if is_rtl {
            let left = if self.end != DimensionValue::Auto {
                self.end
            } else {
                self.left
            };
            let right = if self.start != DimensionValue::Auto {
                self.start
            } else {
                self.right
            };
            (left, right)
        } else {
            let left = if self.start != DimensionValue::Auto {
                self.start
            } else {
                self.left
            };
            let right = if self.end != DimensionValue::Auto {
                self.end
            } else {
                self.right
            };
            (left, right)
        };

        let (resolved_margin_left, resolved_margin_right) = if is_rtl {
            let left = if self.margin_end != DimensionValue::Auto {
                self.margin_end
            } else {
                self.margin_left
            };
            let right = if self.margin_start != DimensionValue::Auto {
                self.margin_start
            } else {
                self.margin_right
            };
            (left, right)
        } else {
            let left = if self.margin_start != DimensionValue::Auto {
                self.margin_start
            } else {
                self.margin_left
            };
            let right = if self.margin_end != DimensionValue::Auto {
                self.margin_end
            } else {
                self.margin_right
            };
            (left, right)
        };

        let (resolved_padding_left, resolved_padding_right) = if is_rtl {
            let left = if self.padding_end != DimensionValue::Auto {
                self.padding_end
            } else {
                self.padding_left
            };
            let right = if self.padding_start != DimensionValue::Auto {
                self.padding_start
            } else {
                self.padding_right
            };
            (left, right)
        } else {
            let left = if self.padding_start != DimensionValue::Auto {
                self.padding_start
            } else {
                self.padding_left
            };
            let right = if self.padding_end != DimensionValue::Auto {
                self.padding_end
            } else {
                self.padding_right
            };
            (left, right)
        };

        Style {
            display: self.display,
            flex_direction: self.flex_direction,
            flex_wrap: self.flex_wrap,
            align_items: self.align_items,
            align_self: self.align_self,
            align_content: self.align_content,
            justify_content: self.justify_content,
            position: self.position,
            inset: Rect {
                left: resolved_left.to_taffy_length_percentage_auto(),
                right: resolved_right.to_taffy_length_percentage_auto(),
                top: self.top.to_taffy_length_percentage_auto(),
                bottom: self.bottom.to_taffy_length_percentage_auto(),
            },
            margin: Rect {
                left: resolved_margin_left.to_taffy_length_percentage_auto(),
                right: resolved_margin_right.to_taffy_length_percentage_auto(),
                top: self.margin_top.to_taffy_length_percentage_auto(),
                bottom: self.margin_bottom.to_taffy_length_percentage_auto(),
            },
            padding: Rect {
                left: resolved_padding_left.to_taffy_length_percentage(),
                right: resolved_padding_right.to_taffy_length_percentage(),
                top: self.padding_top.to_taffy_length_percentage(),
                bottom: self.padding_bottom.to_taffy_length_percentage(),
            },
            size: Size {
                width: self.width.to_taffy_dimension(),
                height: self.height.to_taffy_dimension(),
            },
            min_size: Size {
                width: self.min_width.to_taffy_dimension(),
                height: self.min_height.to_taffy_dimension(),
            },
            max_size: Size {
                width: self.max_width.to_taffy_dimension(),
                height: self.max_height.to_taffy_dimension(),
            },
            aspect_ratio: self.aspect_ratio,
            gap: Size {
                width: LengthPercentage::Length(self.gap.1),
                height: LengthPercentage::Length(self.gap.0),
            },
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            flex_basis: self.flex_basis.to_taffy_dimension(),
            ..Default::default()
        }
    }
}

/// A node in the Ferrix layout tree.
#[derive(Clone, Debug)]
pub struct LayoutNodeData {
    pub node_id: NodeId,
    pub style: LayoutStyle,
    /// Intrinsic measurement if this node is intrinsic-sized (e.g. Text).
    pub intrinsic_size: Option<(f32, f32)>,
    pub children: Vec<NodeId>,
}

/// The Ferrix Layout Engine managing Taffy trees and layout computations.
pub struct LayoutEngine {
    taffy: TaffyTree<NodeId>,
    node_map: HashMap<NodeId, NodeIdData>,
    is_rtl: bool,
}

struct NodeIdData {
    taffy_node: NodeIdTaffy,
    style: LayoutStyle,
    intrinsic_size: Option<(f32, f32)>,
    children: Vec<NodeId>,
}

type NodeIdTaffy = taffy::NodeId;

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
            node_map: HashMap::new(),
            is_rtl: false,
        }
    }

    pub fn set_rtl(&mut self, is_rtl: bool) {
        self.is_rtl = is_rtl;
    }

    pub fn is_rtl(&self) -> bool {
        self.is_rtl
    }

    /// Upserts a node in the layout engine.
    pub fn upsert_node(
        &mut self,
        node_id: NodeId,
        style: LayoutStyle,
        intrinsic_size: Option<(f32, f32)>,
        children: Vec<NodeId>,
    ) -> Result<(), LayoutError> {
        let taffy_style = style.to_taffy_style(self.is_rtl);

        if let Some(data) = self.node_map.get_mut(&node_id) {
            data.style = style;
            data.intrinsic_size = intrinsic_size;
            data.children = children.clone();
            self.taffy.set_style(data.taffy_node, taffy_style)?;
        } else {
            let taffy_node = if let Some((w, h)) = intrinsic_size {
                let mut style_with_size = taffy_style;
                if style_with_size.size.width == Dimension::Auto {
                    style_with_size.size.width = Dimension::Length(w);
                }
                if style_with_size.size.height == Dimension::Auto {
                    style_with_size.size.height = Dimension::Length(h);
                }
                self.taffy.new_leaf_with_context(style_with_size, node_id)?
            } else {
                self.taffy.new_leaf_with_context(taffy_style, node_id)?
            };

            self.node_map.insert(
                node_id,
                NodeIdData {
                    taffy_node,
                    style,
                    intrinsic_size,
                    children: children.clone(),
                },
            );
        }

        // Update children linkage
        let parent_taffy = self.node_map.get(&node_id).unwrap().taffy_node;
        let mut child_taffy_nodes = Vec::new();
        for child_id in children {
            if let Some(child_data) = self.node_map.get(&child_id) {
                child_taffy_nodes.push(child_data.taffy_node);
            }
        }
        self.taffy.set_children(parent_taffy, &child_taffy_nodes)?;

        Ok(())
    }

    /// Computes layout for the subtree rooted at `root_id` with given viewport constraints.
    pub fn compute_layout(
        &mut self,
        root_id: NodeId,
        available_width: f32,
        available_height: f32,
    ) -> Result<Vec<ComputedLayout>, LayoutError> {
        let root_taffy = self
            .node_map
            .get(&root_id)
            .ok_or(LayoutError::NodeNotFound(root_id))?
            .taffy_node;

        let available_space = Size {
            width: AvailableSpace::Definite(available_width),
            height: AvailableSpace::Definite(available_height),
        };

        self.taffy.compute_layout(root_taffy, available_space)?;

        let mut results = Vec::new();
        self.collect_layouts(root_id, 0.0, 0.0, &mut results);

        Ok(results)
    }

    fn collect_layouts(
        &self,
        node_id: NodeId,
        parent_abs_x: f32,
        parent_abs_y: f32,
        out: &mut Vec<ComputedLayout>,
    ) {
        if let Some(data) = self.node_map.get(&node_id) {
            if let Ok(layout) = self.taffy.layout(data.taffy_node) {
                let abs_x = parent_abs_x + layout.location.x;
                let abs_y = parent_abs_y + layout.location.y;

                out.push(ComputedLayout {
                    node_id,
                    x: abs_x,
                    y: abs_y,
                    width: layout.size.width,
                    height: layout.size.height,
                });

                for child_id in &data.children {
                    self.collect_layouts(*child_id, abs_x, abs_y, out);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_column_layout_calculation() {
        let mut engine = LayoutEngine::new();

        let root_id = NodeId(1);
        let child1_id = NodeId(2);
        let child2_id = NodeId(3);

        let root_style = LayoutStyle {
            flex_direction: FlexDirection::Column,
            width: DimensionValue::Points(360.0),
            height: DimensionValue::Points(640.0),
            padding_left: DimensionValue::Points(16.0),
            padding_top: DimensionValue::Points(16.0),
            padding_right: DimensionValue::Points(16.0),
            padding_bottom: DimensionValue::Points(16.0),
            gap: (8.0, 0.0),
            ..Default::default()
        };

        let child1_style = LayoutStyle {
            height: DimensionValue::Points(48.0),
            width: DimensionValue::Percent(100.0),
            ..Default::default()
        };

        let child2_style = LayoutStyle {
            height: DimensionValue::Points(64.0),
            width: DimensionValue::Percent(100.0),
            ..Default::default()
        };

        engine
            .upsert_node(child1_id, child1_style, None, vec![])
            .unwrap();
        engine
            .upsert_node(child2_id, child2_style, None, vec![])
            .unwrap();
        engine
            .upsert_node(root_id, root_style, None, vec![child1_id, child2_id])
            .unwrap();

        let layouts = engine.compute_layout(root_id, 360.0, 640.0).unwrap();
        assert_eq!(layouts.len(), 3);

        let root_layout = layouts.iter().find(|l| l.node_id == root_id).unwrap();
        assert_eq!(root_layout.width, 360.0);
        assert_eq!(root_layout.height, 640.0);

        let c1_layout = layouts.iter().find(|l| l.node_id == child1_id).unwrap();
        assert_eq!(c1_layout.x, 16.0);
        assert_eq!(c1_layout.y, 16.0);
        assert_eq!(c1_layout.width, 328.0); // 360 - 32
        assert_eq!(c1_layout.height, 48.0);

        let c2_layout = layouts.iter().find(|l| l.node_id == child2_id).unwrap();
        assert_eq!(c2_layout.x, 16.0);
        assert_eq!(c2_layout.y, 16.0 + 48.0 + 8.0); // 72.0 (padding + height + gap)
        assert_eq!(c2_layout.width, 328.0);
        assert_eq!(c2_layout.height, 64.0);
    }

    #[test]
    fn test_rtl_layout_inversion() {
        let mut engine = LayoutEngine::new();
        engine.set_rtl(true);

        let root_id = NodeId(1);
        let child_id = NodeId(2);

        let root_style = LayoutStyle {
            flex_direction: FlexDirection::Row,
            width: DimensionValue::Points(400.0),
            height: DimensionValue::Points(100.0),
            padding_start: DimensionValue::Points(20.0),
            ..Default::default()
        };

        let child_style = LayoutStyle {
            width: DimensionValue::Points(100.0),
            height: DimensionValue::Points(50.0),
            margin_start: DimensionValue::Points(10.0),
            ..Default::default()
        };

        engine
            .upsert_node(child_id, child_style, None, vec![])
            .unwrap();
        engine
            .upsert_node(root_id, root_style, None, vec![child_id])
            .unwrap();

        let layouts = engine.compute_layout(root_id, 400.0, 100.0).unwrap();
        assert_eq!(layouts.len(), 2);
    }

    #[test]
    fn test_text_measure_cache_and_estimation() {
        let mut cache = TextMeasureCache::new();

        // 1. Estimation fallback
        let (est_w, est_h) = TextMeasureCache::estimate("Hello", 16.0, 400, 0, 0.0);
        assert!(est_w > 0.0);
        assert!(est_h > 0.0);

        // 2. Queue and apply native responses
        let req = cache.queue_request(NodeId(5), "Native Text".to_string(), 18.0, 700, 0, 300.0);
        assert_eq!(req.request_id, 1);
        assert_eq!(cache.get("Native Text", 18.0, 700, 0, 300.0), None);

        cache.apply_responses(&[TextMeasureResponse {
            request_id: 1,
            width: 110.0,
            height: 24.0,
        }]);

        let measured = cache.get("Native Text", 18.0, 700, 0, 300.0);
        assert_eq!(measured, Some((110.0, 24.0)));
    }
}
