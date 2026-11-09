//! Reactive query and mutation hooks powered by `ferrix-network`.

use ferrix_network::{MutationStatus, QueryStatus};
use ferrix_reactivity::Signal;
use std::rc::Rc;

use crate::component::ComponentContext;

/// Handle returned by `use_mutation` with reactive status and an execution trigger.
#[derive(Clone)]
pub struct MutationHandle<T: Clone + 'static, V: 'static> {
    pub status: Signal<MutationStatus<T>>,
    pub mutate: Rc<dyn Fn(V)>,
}

/// Reactive hook: `use_query`
/// Fetches remote data and exposes `Loading`, `Success(T)`, or `Error(String)` states.
pub fn use_query<T: Clone + 'static>(
    cx: &mut ComponentContext,
    query_key: impl Into<String>,
    fetcher: impl FnOnce() -> Result<T, String> + 'static,
) -> Signal<QueryStatus<T>> {
    let key = query_key.into();
    let status_signal = cx.use_signal(QueryStatus::Loading);

    let sig_clone = status_signal.clone();
    cx.use_effect(
        move || {
            match fetcher() {
                Ok(data) => sig_clone.set(QueryStatus::Success(data)),
                Err(err) => sig_clone.set(QueryStatus::Error(err)),
            }
            // Cleanup on unmount
            None
        },
        key,
    );

    status_signal
}

/// Reactive hook: `use_mutation`
/// Triggers asynchronous remote mutations and tracks their execution state.
pub fn use_mutation<T: Clone + 'static, V: 'static>(
    cx: &mut ComponentContext,
    mutator: impl Fn(V) -> Result<T, String> + 'static,
) -> MutationHandle<T, V> {
    let status = cx.use_signal(MutationStatus::Idle);

    let status_clone = status.clone();
    let mutator_rc = Rc::new(mutator);

    let mutate: Rc<dyn Fn(V)> = Rc::new(move |variables: V| {
        status_clone.set(MutationStatus::Loading);
        match mutator_rc(variables) {
            Ok(res) => status_clone.set(MutationStatus::Success(res)),
            Err(err) => status_clone.set(MutationStatus::Error(err)),
        }
    });

    MutationHandle { status, mutate }
}
