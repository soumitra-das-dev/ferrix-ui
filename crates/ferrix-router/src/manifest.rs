//! Route manifest, directory tree scanner, and URL pattern matching.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RouterError {
    #[error("Duplicate route registered: {0}")]
    DuplicateRoute(String),
    #[error("Ambiguous route conflict between '{0}' and '{1}'")]
    AmbiguousRoute(String, String),
    #[error("Route not found: {0}")]
    NotFound(String),
    #[error("Serialization failed: {0}")]
    SerializationError(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SegmentPattern {
    Static(String),
    Dynamic(String),  // [id]
    CatchAll(String), // [...rest]
    Group(String),    // (auth) - ignored in URL matching
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteEntry {
    pub pattern: String,
    pub component_name: String,
    pub layout_name: Option<String>,
    pub is_modal: bool,
    pub is_not_found: bool,
    pub is_error: bool,
    pub segments: Vec<SegmentPattern>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RouteMatch {
    pub route: RouteEntry,
    pub pathname: String,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RouteManifest {
    pub routes: Vec<RouteEntry>,
}

impl RouteManifest {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Registers a route into the manifest with validation against duplicates.
    pub fn add_route(&mut self, route: RouteEntry) -> Result<(), RouterError> {
        if self.routes.iter().any(|r| r.pattern == route.pattern) {
            return Err(RouterError::DuplicateRoute(route.pattern));
        }
        self.routes.push(route);
        Ok(())
    }

    /// Matches an incoming URL path against registered routes.
    pub fn match_url(&self, full_url: &str) -> RouteMatch {
        let (path, query_str) = match full_url.split_once('?') {
            Some((p, q)) => (p, Some(q)),
            None => (full_url, None),
        };

        let mut query = HashMap::new();
        if let Some(q) = query_str {
            for pair in q.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    query.insert(k.to_string(), v.to_string());
                } else if !pair.is_empty() {
                    query.insert(pair.to_string(), String::new());
                }
            }
        }

        let clean_path = path.trim_matches('/');
        let incoming_segments: Vec<&str> = if clean_path.is_empty() {
            Vec::new()
        } else {
            clean_path.split('/').collect()
        };

        for route in &self.routes {
            if route.is_not_found || route.is_error {
                continue;
            }

            if let Some(params) = Self::match_segments(&route.segments, &incoming_segments) {
                return RouteMatch {
                    route: route.clone(),
                    pathname: path.to_string(),
                    params,
                    query,
                };
            }
        }

        // Fallback to +not-found route if registered, else a synthetic 404 entry
        if let Some(nf) = self.routes.iter().find(|r| r.is_not_found) {
            RouteMatch {
                route: nf.clone(),
                pathname: path.to_string(),
                params: HashMap::new(),
                query,
            }
        } else {
            RouteMatch {
                route: RouteEntry {
                    pattern: "+not-found".to_string(),
                    component_name: "NotFoundScreen".to_string(),
                    layout_name: None,
                    is_modal: false,
                    is_not_found: true,
                    is_error: false,
                    segments: vec![SegmentPattern::Static("+not-found".to_string())],
                },
                pathname: path.to_string(),
                params: HashMap::new(),
                query,
            }
        }
    }

    /// Finds the nearest ancestor `+error` route for a given path or falls back to synthetic error.
    pub fn match_error_route(&self, path: &str, error_message: &str) -> RouteMatch {
        let mut params = HashMap::new();
        params.insert("error".to_string(), error_message.to_string());

        // Find route with is_error = true
        if let Some(err_route) = self.routes.iter().find(|r| r.is_error) {
            RouteMatch {
                route: err_route.clone(),
                pathname: format!("{}/+error", path.trim_end_matches('/')),
                params,
                query: HashMap::new(),
            }
        } else {
            RouteMatch {
                route: RouteEntry {
                    pattern: "+error".to_string(),
                    component_name: "ErrorScreen".to_string(),
                    layout_name: None,
                    is_modal: false,
                    is_not_found: false,
                    is_error: true,
                    segments: vec![SegmentPattern::Static("+error".to_string())],
                },
                pathname: format!("{}/+error", path.trim_end_matches('/')),
                params,
                query: HashMap::new(),
            }
        }
    }

    fn match_segments(
        patterns: &[SegmentPattern],
        incoming: &[&str],
    ) -> Option<HashMap<String, String>> {
        // Filter out route groups (e.g. (auth)) from URL comparison
        let url_patterns: Vec<&SegmentPattern> = patterns
            .iter()
            .filter(|p| !matches!(p, SegmentPattern::Group(_)))
            .collect();

        let mut params = HashMap::new();
        let mut p_idx = 0;
        let mut i_idx = 0;

        while p_idx < url_patterns.len() && i_idx < incoming.len() {
            match url_patterns[p_idx] {
                SegmentPattern::Static(s) => {
                    if s != incoming[i_idx] {
                        return None;
                    }
                    p_idx += 1;
                    i_idx += 1;
                }
                SegmentPattern::Dynamic(key) => {
                    params.insert(key.clone(), incoming[i_idx].to_string());
                    p_idx += 1;
                    i_idx += 1;
                }
                SegmentPattern::CatchAll(key) => {
                    let rest = incoming[i_idx..].join("/");
                    params.insert(key.clone(), rest);
                    return Some(params);
                }
                SegmentPattern::Group(_) => unreachable!(),
            }
        }

        if p_idx == url_patterns.len() && i_idx == incoming.len() {
            Some(params)
        } else {
            None
        }
    }

    /// Scans a list of relative file paths under `app/` and builds a verified `RouteManifest`.
    pub fn scan_virtual_dir(file_paths: &[&str]) -> Result<Self, RouterError> {
        let mut manifest = Self::new();

        for file_path in file_paths {
            let path = file_path.trim_start_matches("app/").trim_start_matches('/');
            if path.starts_with('_') && !path.contains("layout") {
                continue; // Skip private partials like _components.fsx
            }

            let without_ext = path.strip_suffix(".fsx").unwrap_or(path);
            let raw_segments: Vec<&str> = without_ext.split('/').collect();

            let is_not_found = raw_segments.last() == Some(&"+not-found");
            let is_error = raw_segments.last() == Some(&"+error");
            let is_layout = raw_segments.last() == Some(&"_layout");
            let is_modal = raw_segments
                .iter()
                .any(|s| *s == "modal" || *s == "(modal)");

            if is_layout {
                continue; // Layouts are linked as containers to sibling routes
            }

            let mut segments = Vec::new();
            let mut url_parts = Vec::new();

            for seg in &raw_segments {
                if seg.starts_with('(') && seg.ends_with(')') {
                    let group_name = seg[1..seg.len() - 1].to_string();
                    segments.push(SegmentPattern::Group(group_name));
                } else if seg.starts_with("[...") && seg.ends_with(']') {
                    let key = seg[4..seg.len() - 1].to_string();
                    segments.push(SegmentPattern::CatchAll(key.clone()));
                    url_parts.push(format!("[...{}]", key));
                } else if seg.starts_with('[') && seg.ends_with(']') {
                    let key = seg[1..seg.len() - 1].to_string();
                    segments.push(SegmentPattern::Dynamic(key.clone()));
                    url_parts.push(format!("[{}]", key));
                } else if *seg == "index" {
                    // index maps to the directory itself
                } else {
                    segments.push(SegmentPattern::Static(seg.to_string()));
                    url_parts.push(seg.to_string());
                }
            }

            let pattern = if url_parts.is_empty() {
                "/".to_string()
            } else {
                format!("/{}", url_parts.join("/"))
            };

            let component_name = raw_segments
                .iter()
                .filter(|s| !s.starts_with('('))
                .map(|s| {
                    let clean = s.replace(['[', ']', '.', '+', '-'], "");
                    let mut chars = clean.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<String>>()
                .join("")
                + "Screen";

            manifest.add_route(RouteEntry {
                pattern,
                component_name,
                layout_name: None,
                is_modal,
                is_not_found,
                is_error,
                segments,
            })?;
        }

        Ok(manifest)
    }
}
