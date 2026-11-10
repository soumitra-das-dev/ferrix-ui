//! Node ID allocator and identifier utilities.

use ferrix_renderer_protocol::NodeId;
use std::sync::atomic::{AtomicU64, Ordering};

/// Thread-safe monotonic generator for UI node identifiers.
#[derive(Debug)]
pub struct NodeIdAllocator {
    next_id: AtomicU64,
}

impl NodeIdAllocator {
    /// Creates a new allocator starting after the root node ID (starts at 2).
    pub const fn new() -> Self {
        Self {
            next_id: AtomicU64::new(2),
        }
    }

    /// Creates an allocator starting at an explicit initial ID.
    pub const fn with_start(start: u64) -> Self {
        Self {
            next_id: AtomicU64::new(start),
        }
    }

    /// Allocates the next stable, unique `NodeId`.
    pub fn allocate(&self) -> NodeId {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        NodeId::new(id)
    }

    /// Resets the allocator back to a specific ID (useful for tests or component scopes).
    pub fn reset(&self, start: u64) {
        self.next_id.store(start, Ordering::Relaxed);
    }
}

impl Default for NodeIdAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocator_sequential() {
        let alloc = NodeIdAllocator::new();
        assert_eq!(alloc.allocate(), NodeId::new(2));
        assert_eq!(alloc.allocate(), NodeId::new(3));
        assert_eq!(alloc.allocate(), NodeId::new(4));
    }

    #[test]
    fn test_allocator_reset() {
        let alloc = NodeIdAllocator::new();
        assert_eq!(alloc.allocate(), NodeId::new(2));
        alloc.reset(100);
        assert_eq!(alloc.allocate(), NodeId::new(100));
        assert_eq!(alloc.allocate(), NodeId::new(101));
    }
}
