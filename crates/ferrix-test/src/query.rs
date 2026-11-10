//! Query search engine and errors for headless tree inspection.

use crate::node::TestNode;
use thiserror::Error;

/// Error types emitted during tree queries.
#[derive(Debug, Error, PartialEq)]
pub enum QueryError {
    #[error("Unable to find an element with text: '{0}'")]
    TextNotFound(String),
    #[error("Found multiple elements with text: '{0}' (count: {1})")]
    MultipleWithText(String, usize),
    #[error("Unable to find an element with test-id: '{0}'")]
    TestIdNotFound(String),
    #[error("Found multiple elements with test-id: '{0}' (count: {1})")]
    MultipleWithTestId(String, usize),
    #[error("Unable to find an element with role: '{0}'")]
    RoleNotFound(String),
    #[error("Found multiple elements with role: '{0}' (count: {1})")]
    MultipleWithRole(String, usize),
    #[error("Unable to find an element with accessibility label: '{0}'")]
    LabelNotFound(String),
    #[error("Found multiple elements with accessibility label: '{0}' (count: {1})")]
    MultipleWithLabel(String, usize),
}

/// Executes queries on a root TestNode.
pub struct QueryEngine;

impl QueryEngine {
    /// Returns all nodes in the tree.
    pub fn all(root: &TestNode) -> Vec<TestNode> {
        root.flatten()
    }

    /// Finds all nodes containing the specified text (case-insensitive substring or exact).
    pub fn find_all_by_text(root: &TestNode, text: &str) -> Vec<TestNode> {
        Self::all(root)
            .into_iter()
            .filter(|n| n.text().is_some_and(|t| t.contains(text)))
            .collect()
    }

    /// Finds exactly one node matching `text`, or returns an error.
    pub fn get_by_text(root: &TestNode, text: &str) -> Result<TestNode, QueryError> {
        let matches = Self::find_all_by_text(root, text);
        match matches.len() {
            0 => Err(QueryError::TextNotFound(text.to_string())),
            1 => Ok(matches.into_iter().next().unwrap()),
            count => Err(QueryError::MultipleWithText(text.to_string(), count)),
        }
    }

    /// Finds all nodes with the given test_id.
    pub fn find_all_by_test_id(root: &TestNode, test_id: &str) -> Vec<TestNode> {
        Self::all(root)
            .into_iter()
            .filter(|n| n.test_id() == Some(test_id))
            .collect()
    }

    /// Finds exactly one node matching `test_id`.
    pub fn get_by_test_id(root: &TestNode, test_id: &str) -> Result<TestNode, QueryError> {
        let matches = Self::find_all_by_test_id(root, test_id);
        match matches.len() {
            0 => Err(QueryError::TestIdNotFound(test_id.to_string())),
            1 => Ok(matches.into_iter().next().unwrap()),
            count => Err(QueryError::MultipleWithTestId(test_id.to_string(), count)),
        }
    }

    /// Finds all nodes with the given accessibility role.
    pub fn find_all_by_role(root: &TestNode, role: &str) -> Vec<TestNode> {
        Self::all(root)
            .into_iter()
            .filter(|n| n.role().is_some_and(|r| r.eq_ignore_ascii_case(role)))
            .collect()
    }

    /// Finds exactly one node matching `role`.
    pub fn get_by_role(root: &TestNode, role: &str) -> Result<TestNode, QueryError> {
        let matches = Self::find_all_by_role(root, role);
        match matches.len() {
            0 => Err(QueryError::RoleNotFound(role.to_string())),
            1 => Ok(matches.into_iter().next().unwrap()),
            count => Err(QueryError::MultipleWithRole(role.to_string(), count)),
        }
    }

    /// Finds all nodes with the given accessibility label.
    pub fn find_all_by_label(root: &TestNode, label: &str) -> Vec<TestNode> {
        Self::all(root)
            .into_iter()
            .filter(|n| n.label().is_some_and(|l| l.contains(label)))
            .collect()
    }

    /// Finds exactly one node matching `label`.
    pub fn get_by_label(root: &TestNode, label: &str) -> Result<TestNode, QueryError> {
        let matches = Self::find_all_by_label(root, label);
        match matches.len() {
            0 => Err(QueryError::LabelNotFound(label.to_string())),
            1 => Ok(matches.into_iter().next().unwrap()),
            count => Err(QueryError::MultipleWithLabel(label.to_string(), count)),
        }
    }
}
