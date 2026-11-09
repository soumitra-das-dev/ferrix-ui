//! Component execution context, hook storage, and lifecycle management.

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::node_id::NodeIdAllocator;
use ferrix_reactivity::Signal;

/// State hook container wrapping a reactive Signal.
#[derive(Clone, Debug)]
pub struct State<T> {
    signal: Signal<T>,
}

impl<T: Clone + 'static> State<T> {
    pub fn new(signal: Signal<T>) -> Self {
        Self { signal }
    }

    #[inline]
    pub fn get(&self) -> T {
        self.signal.get()
    }

    #[inline]
    pub fn set(&self, val: T) {
        self.signal.set(val);
    }

    #[inline]
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        self.signal.update(f);
    }

    #[inline]
    pub fn signal(&self) -> &Signal<T> {
        &self.signal
    }
}

/// Execution context for a functional UI component providing stable-order hook storage.
pub struct ComponentContext {
    start_id: u64,
    allocator: NodeIdAllocator,
    hooks: Vec<Box<dyn Any>>,
    hook_cursor: usize,
    cleanups: Vec<Option<Box<dyn FnOnce()>>>,
}

impl ComponentContext {
    /// Creates a new component context starting with the default node ID allocator.
    pub fn new() -> Self {
        Self {
            start_id: 2,
            allocator: NodeIdAllocator::new(),
            hooks: Vec::new(),
            hook_cursor: 0,
            cleanups: Vec::new(),
        }
    }

    /// Creates a context with a specific starting node ID.
    pub fn with_start_id(start_id: u64) -> Self {
        Self {
            start_id,
            allocator: NodeIdAllocator::with_start(start_id),
            hooks: Vec::new(),
            hook_cursor: 0,
            cleanups: Vec::new(),
        }
    }

    /// Accessor for the node ID allocator.
    pub fn allocator(&self) -> &NodeIdAllocator {
        &self.allocator
    }

    /// Resets the hook cursor and node ID sequence before beginning a component render pass.
    pub fn prepare_render(&mut self) {
        self.hook_cursor = 0;
        self.allocator.reset(self.start_id);
    }

    /// Retrieves a persistent Signal stored at hook index `index`.
    pub fn get_signal<T: Clone + 'static>(&self, index: usize) -> Option<Signal<T>> {
        self.hooks
            .get(index)
            .and_then(|slot| slot.downcast_ref::<Signal<T>>().cloned())
    }

    /// Hook: `useState<T>`
    /// Provides persistent reactive state across renders.
    pub fn use_state<T: Clone + 'static>(&mut self, initial: T) -> State<T> {
        if self.hook_cursor < self.hooks.len() {
            let slot = &self.hooks[self.hook_cursor];
            self.hook_cursor += 1;
            let signal = slot
                .downcast_ref::<Signal<T>>()
                .expect("Hook type mismatch in useState slot")
                .clone();
            State::new(signal)
        } else {
            let signal = Signal::new(initial);
            self.hooks.push(Box::new(signal.clone()));
            self.hook_cursor += 1;
            State::new(signal)
        }
    }

    /// Hook: `useSignal<T>`
    /// Provides direct access to a low-level reactive Signal.
    pub fn use_signal<T: Clone + 'static>(&mut self, initial: T) -> Signal<T> {
        if self.hook_cursor < self.hooks.len() {
            let slot = &self.hooks[self.hook_cursor];
            self.hook_cursor += 1;
            slot.downcast_ref::<Signal<T>>()
                .expect("Hook type mismatch in useSignal slot")
                .clone()
        } else {
            let signal = Signal::new(initial);
            self.hooks.push(Box::new(signal.clone()));
            self.hook_cursor += 1;
            signal
        }
    }

    /// Hook: `useRef<T>`
    /// Provides a persistent mutable reference that does not trigger re-renders on mutation.
    pub fn use_ref<T: 'static>(&mut self, initial: T) -> Rc<RefCell<T>> {
        if self.hook_cursor < self.hooks.len() {
            let slot = &self.hooks[self.hook_cursor];
            self.hook_cursor += 1;
            slot.downcast_ref::<Rc<RefCell<T>>>()
                .expect("Hook type mismatch in useRef slot")
                .clone()
        } else {
            let cell = Rc::new(RefCell::new(initial));
            self.hooks.push(Box::new(cell.clone()));
            self.hook_cursor += 1;
            cell
        }
    }

    /// Hook: `useMemo<T, D>`
    /// Computes and caches a value as long as `deps` remain unchanged.
    pub fn use_memo<T: Clone + 'static, D: PartialEq + 'static>(
        &mut self,
        compute: impl FnOnce() -> T,
        deps: D,
    ) -> T {
        struct MemoSlot<T, D> {
            cached_val: T,
            deps: D,
        }

        if self.hook_cursor < self.hooks.len() {
            let slot = self.hooks[self.hook_cursor]
                .downcast_mut::<MemoSlot<T, D>>()
                .expect("Hook type mismatch in useMemo slot");
            self.hook_cursor += 1;

            if slot.deps != deps {
                slot.cached_val = compute();
                slot.deps = deps;
            }
            slot.cached_val.clone()
        } else {
            let val = compute();
            let slot = MemoSlot {
                cached_val: val.clone(),
                deps,
            };
            self.hooks.push(Box::new(slot));
            self.hook_cursor += 1;
            val
        }
    }

    /// Hook: `useEffect<D>`
    /// Registers a side effect that executes when dependencies change, with optional cleanup.
    pub fn use_effect<D: PartialEq + 'static>(
        &mut self,
        effect: impl FnOnce() -> Option<Box<dyn FnOnce()>>,
        deps: D,
    ) {
        struct EffectSlot<D> {
            deps: D,
            cleanup: Option<Box<dyn FnOnce()>>,
        }

        if self.hook_cursor < self.hooks.len() {
            let slot = self.hooks[self.hook_cursor]
                .downcast_mut::<EffectSlot<D>>()
                .expect("Hook type mismatch in useEffect slot");
            self.hook_cursor += 1;

            if slot.deps != deps {
                if let Some(cleanup) = slot.cleanup.take() {
                    cleanup();
                }
                slot.deps = deps;
                slot.cleanup = effect();
            }
        } else {
            let cleanup = effect();
            let slot = EffectSlot { deps, cleanup };
            self.hooks.push(Box::new(slot));
            self.hook_cursor += 1;
        }
    }

    /// Cleans up all active effect cleanups when the component unmounts.
    pub fn unmount(&mut self) {
        for cb in self.cleanups.drain(..).flatten() {
            cb();
        }
    }
}

impl Default for ComponentContext {
    fn default() -> Self {
        Self::new()
    }
}
