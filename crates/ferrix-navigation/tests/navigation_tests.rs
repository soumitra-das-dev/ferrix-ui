//! Unit and integration tests for ferrix-navigation (F2).

use ferrix_navigation::prelude::*;
use ferrix_renderer_protocol::{Command, CommandBatch, NodeId};

#[test]
fn test_stack_navigator_lifecycle_and_commands() {
    let container_id = NodeId(10);
    let mut stack = StackNavigator::new(container_id);

    assert_eq!(stack.len(), 0);
    assert!(!stack.can_go_back());

    // Push Screen 1 (Home)
    let s1 = NodeId(20);
    let opts1 = ScreenOptions {
        title: "Home".to_string(),
        header_shown: true,
        presentation: PresentationStyle::Card,
        animation: TransitionAnimation::SlideFromRight,
    };
    let batch1 = stack.push(s1, "home", opts1);

    assert_eq!(stack.len(), 1);
    assert!(!stack.can_go_back());
    assert_eq!(batch1.len(), 2);
    assert_eq!(
        batch1.commands[0],
        Command::SetScreenOptions {
            screen_id: s1,
            title: "Home".to_string(),
            header_shown: true,
            presentation: 0,
        }
    );
    assert_eq!(
        batch1.commands[1],
        Command::Navigate {
            container_id,
            screen_id: s1,
            animation: 2,
        }
    );

    // Push Screen 2 (Details)
    let s2 = NodeId(30);
    let opts2 = ScreenOptions {
        title: "Details".to_string(),
        header_shown: true,
        presentation: PresentationStyle::Modal,
        animation: TransitionAnimation::SlideFromBottom,
    };
    let _batch2 = stack.push(s2, "details", opts2);

    assert_eq!(stack.len(), 2);
    assert!(stack.can_go_back());
    assert_eq!(stack.active_screen().unwrap().route_name, "details");

    // Pop Screen 2
    let (popped, pop_batch) = stack.pop(None);
    assert!(popped);
    assert_eq!(stack.len(), 1);
    assert!(!stack.can_go_back());
    assert_eq!(pop_batch.len(), 1);
    assert_eq!(
        pop_batch.commands[0],
        Command::GoBack {
            container_id,
            animation: 0,
        }
    );

    // Cannot pop root
    let (popped_root, _) = stack.pop(None);
    assert!(!popped_root);
}

#[test]
fn test_tab_navigator_switching() {
    let container_id = NodeId(50);
    let mut tabs = TabNavigator::new(container_id);

    let tab1_screen = NodeId(51);
    let tab2_screen = NodeId(52);
    tabs.add_tab(tab1_screen, "Feed", Some("home_icon"));
    tabs.add_tab(tab2_screen, "Profile", Some("user_icon"));

    assert_eq!(tabs.len(), 2);
    assert_eq!(tabs.active_index(), 0);
    assert_eq!(tabs.active_tab().unwrap().title, "Feed");

    // Switch to tab 1 (Profile)
    let batch = tabs.switch_tab(1).expect("Tab switch should return batch");
    assert_eq!(tabs.active_index(), 1);
    assert_eq!(tabs.active_tab().unwrap().title, "Profile");
    assert_eq!(batch.len(), 1);
    assert_eq!(
        batch.commands[0],
        Command::Navigate {
            container_id,
            screen_id: tab2_screen,
            animation: 4, // None
        }
    );

    // Switching to current tab returns None
    assert!(tabs.switch_tab(1).is_none());
}

#[test]
fn test_command_batch_serialization_roundtrip() {
    let container_id = NodeId(100);
    let mut stack = StackNavigator::new(container_id);

    let s1 = NodeId(101);
    let opts = ScreenOptions {
        title: "Settings".to_string(),
        header_shown: true,
        presentation: PresentationStyle::Card,
        animation: TransitionAnimation::Fade,
    };
    let batch = stack.push(s1, "settings", opts);

    let encoded = batch.encode();
    let decoded = CommandBatch::decode(&encoded).expect("Decode should succeed");
    assert_eq!(batch, decoded);
}
