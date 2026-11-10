use ferrix_core::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct TodoItem {
    id: u64,
    text: String,
    completed: bool,
}

/// Hand-written functional Todo application with reactive list state and keyed rows.
fn todo_app_component(
    cx: &mut ComponentContext,
    initial: Vec<TodoItem>,
) -> (Element, State<Vec<TodoItem>>) {
    let todos = cx.use_state(initial);

    let mut col = Column::new()
        .padding(16.0)
        .spacing(10.0)
        .child(Text::new("Ferrix Todo App").size(24.0).color(0xFF0F172A))
        .child(
            Button::new("Add Todo")
                .background_color(0xFF16A34A)
                .text_color(0xFFFFFFFF),
        );

    // Render keyed todo list
    for item in todos.get() {
        let status = if item.completed { "[x] " } else { "[ ] " };
        let item_key = format!("todo_{}", item.id);
        let row = Row::new()
            .key(item_key.clone())
            .spacing(8.0)
            .child(
                Text::new(format!("{}{}", status, item.text))
                    .size(16.0)
                    .color(if item.completed {
                        0xFF94A3B8
                    } else {
                        0xFF1E293B
                    }),
            )
            .child(
                Button::new("Toggle")
                    .background_color(0xFF3B82F6)
                    .text_color(0xFFFFFFFF),
            )
            .child(
                Button::new("Delete")
                    .background_color(0xFFEF4444)
                    .text_color(0xFFFFFFFF),
            );
        col = col.child(row);
    }

    let tree = col.build_with_id(NodeId::new(1), cx.allocator());
    (tree, todos)
}

#[test]
fn test_todo_app_add_toggle_and_keyed_delete() {
    let mut cx = ComponentContext::with_start_id(2);

    // 1. Initial render with 1 item
    cx.prepare_render();
    let initial_items = vec![TodoItem {
        id: 1,
        text: "Buy groceries".to_string(),
        completed: false,
    }];
    let (tree_1, todos) = todo_app_component(&mut cx, initial_items);
    let mount_patches = generate_mount_patches(&tree_1);
    assert!(!mount_patches.is_empty());

    // 2. Add second item
    todos.update(|list| {
        list.push(TodoItem {
            id: 2,
            text: "Read Ferrix book".to_string(),
            completed: false,
        });
    });

    cx.prepare_render();
    let (tree_2, _) = todo_app_component(&mut cx, Vec::new());
    let diff_add = diff_elements(&tree_1, &tree_2);

    // Expect: new row appended
    assert!(diff_add.patches.iter().any(|p| matches!(
        p,
        Patch::SetText { text, .. } if text == "[ ] Read Ferrix book"
    )));
    assert!(diff_add.patches.iter().any(|p| matches!(
        p,
        Patch::AppendChild { parent_id, .. } if *parent_id == NodeId::new(1)
    )));

    // 3. Toggle completion of item 1
    todos.update(|list| {
        list[0].completed = true;
    });

    cx.prepare_render();
    let (tree_3, _) = todo_app_component(&mut cx, Vec::new());
    let diff_toggle = diff_elements(&tree_2, &tree_3);

    // Expect: text updated to "[x] Buy groceries"
    assert!(diff_toggle.patches.iter().any(|p| matches!(
        p,
        Patch::SetText { text, .. } if text == "[x] Buy groceries"
    )));
    // Keyed nodes were NOT recreated: zero CreateNode for existing items
    assert!(!diff_toggle
        .patches
        .iter()
        .any(|p| matches!(p, Patch::CreateNode { .. })));

    // 4. Delete item 1
    todos.update(|list| {
        list.remove(0);
    });

    cx.prepare_render();
    let (tree_4, _) = todo_app_component(&mut cx, Vec::new());
    let diff_delete = diff_elements(&tree_3, &tree_4);

    // Expect: RemoveChild and DestroyNode for item 1
    assert!(diff_delete.patches.iter().any(|p| matches!(
        p,
        Patch::RemoveChild { parent_id, .. } if *parent_id == NodeId::new(1)
    )));
    assert!(diff_delete
        .patches
        .iter()
        .any(|p| matches!(p, Patch::DestroyNode { .. })));

    // Item 2 was preserved in tree_4
    assert!(tree_4
        .children
        .iter()
        .any(|c| c.key.as_deref() == Some("todo_2")));
}
