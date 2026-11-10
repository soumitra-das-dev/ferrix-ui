use ferrix_core::component::ComponentContext;
use ferrix_core::query_hooks::{use_mutation, use_query};
use ferrix_network::{MutationStatus, QueryStatus};

#[derive(Clone, Debug, PartialEq)]
struct UserProfile {
    id: u64,
    name: String,
}

#[test]
fn test_use_query_success_state() {
    let mut cx = ComponentContext::new();

    let query_signal = use_query(&mut cx, "user_1", || {
        Ok(UserProfile {
            id: 1,
            name: "Alice Ferrix".to_string(),
        })
    });

    assert_eq!(
        query_signal.get(),
        QueryStatus::Success(UserProfile {
            id: 1,
            name: "Alice Ferrix".to_string(),
        })
    );
}

#[test]
fn test_use_query_error_state() {
    let mut cx = ComponentContext::new();

    let query_signal =
        use_query::<UserProfile>(&mut cx, "user_999", || Err("User not found".to_string()));

    assert_eq!(
        query_signal.get(),
        QueryStatus::Error("User not found".to_string())
    );
}

#[test]
fn test_use_mutation_lifecycle() {
    let mut cx = ComponentContext::new();

    let handle = use_mutation(&mut cx, |new_name: String| {
        if new_name.is_empty() {
            Err("Name cannot be empty".to_string())
        } else {
            Ok(UserProfile {
                id: 42,
                name: new_name,
            })
        }
    });

    assert_eq!(handle.status.get(), MutationStatus::Idle);

    // Perform successful mutation
    (handle.mutate)("Bob Ferrix".to_string());
    assert_eq!(
        handle.status.get(),
        MutationStatus::Success(UserProfile {
            id: 42,
            name: "Bob Ferrix".to_string(),
        })
    );

    // Perform failed mutation
    (handle.mutate)("".to_string());
    assert_eq!(
        handle.status.get(),
        MutationStatus::Error("Name cannot be empty".to_string())
    );
}
