# F2 Learnings: Native Navigation Containers (`ferrix-navigation`)

## Architecture Decisions & Container Paradigm
Navigation under **Architecture A** enforces strict role separation:
- Rust owns the navigation hierarchy, active screen stack, tab indices, and transition states.
- Kotlin renders native Android View containers and executes 60fps/120fps hardware-accelerated transitions.

### 1. Custom ViewGroups vs Android Fragments (ADR)
- **Problem**: Android `Fragment` and `FragmentManager` introduce complex transaction commit scheduling (`commitNow()`, asynchronous pending transactions), competing back-stack management, and unpredictable lifecycle loops that conflict with Rust's single source of truth.
- **Solution**: Navigation is implemented using lightweight custom `ViewGroup`s:
  - `FerrixNavContainer` (custom layout managing screen stacks and transitions)
  - `FerrixScreenView` (screen view container with active/inactive/transitioning lifecycle states)
  - `FerrixHeaderView` (native action bar with title, back button, and actions)
  - `FerrixTabBarView` (bottom navigation tab bar)
- This eliminates Fragment lifecycle race conditions entirely while maintaining exact $O(1)$ stack operations.

### 2. Additive Wire Protocol Commands
Navigation actions are transmitted using typed binary commands over the existing `CommandBatch` channel:
- `Command::Navigate { container_id, screen_id, animation }` (opcode `0x05`)
- `Command::GoBack { container_id, animation }` (opcode `0x06`)
- `Command::SetScreenOptions { screen_id, title, header_shown, presentation }` (opcode `0x07`)

### 3. Non-Blocking Native Transitions
- Android property animators (`translationX`, `translationY`, `alpha`) run natively on the Android RenderThread/Main Thread.
- Animations do not block the Rust runtime thread or require per-frame JNI roundtrips.
- Presentation styles supported: `Card`, `Modal`, `TransparentModal`.
- Transitions supported: `Default` (SlideFromRight), `Fade`, `SlideFromRight`, `SlideFromBottom` (Modal), `None`.

### 4. Verification Evidence
- Rust tests: 3 unit tests in `crates/ferrix-navigation/tests/navigation_tests.rs` (stack push/pop, tab switching, command batch serialization round-trip).
- Protocol tests: `test_command_batch_round_trip` in `crates/ferrix-renderer-protocol`.
- Android Robolectric tests: 3 test cases in `NavigationTest.kt` verifying ViewFactory creation, container stack depth, title updates, back button visibility, and command execution.
- Full workspace tests: 74 Rust tests passing, all Android Robolectric unit tests passing.
