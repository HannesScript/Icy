# Icy

Icy is a Rust-based app development framework focused on declarative UI and automatic window rendering.

## Vision

Build apps by describing components and state, then let Icy handle:
- SDL3 window creation
- rendering the full window content automatically
- rerendering when state changes
- event wiring (button actions, input, window events)

## Project Status

This repository is in an early scaffold phase.

Current root code is minimal (`src/main.rs`), and `example_app/` illustrates the intended framework syntax and API style.

## Intended Developer Experience

The target API style (from `example_app`) is:

```rust
use Icy::App::*;

fn main() {
    let app: IcyApp = create_icy_app();
    let window: IcyWindow = IcyWindow { width: 960, height: 540 };

    app.mount_main_component();
    app.mount_to_window(&window);
    app.show_window();
}
```

With component-based UI composition:

```rust
use Icy::Component::*;

pub fn app_main() {
    return Component {
        children: vec![
            h1 { content: Text { text: "Counter App" } },
            counter(),
        ],
    };
}
```

And state-driven updates:

```rust
use Icy::StateManagement::*;

pub fn counter() {
    let counter_state: IcyState = new_state(i32, 0);
    // UI should rerender automatically when state changes
}
```

## Core Framework Goals

1. SDL3-based rendering backend for desktop windows.
2. Declarative component tree (`Component`, `Text`, `button`, etc.).
3. Automatic diff/reconciliation and rerender pipeline.
4. State system that triggers UI updates without manual redraw calls.
5. App lifecycle and event loop managed by framework.
6. Extensible architecture for additional widgets and layout primitives.

## Proposed Architecture

- `App` module:
  - App initialization
  - window mounting
  - run loop lifecycle
- `Renderer` module:
  - SDL3 setup
  - draw traversal of component tree
  - frame scheduling
- `Component` module:
  - node definitions
  - props/content
  - children composition
- `StateManagement` module:
  - state containers
  - mutation/update notifications
  - rerender triggers
- `Events` module:
  - input handlers
  - component action dispatch

## Running Today

Current runnable state is the default crate entrypoint in `src/main.rs`.

As the framework APIs are implemented, this README should be updated with:
- actual module paths
- working example app integration
- build and run commands for SDL3 runtime

## Example App Folder

The `example_app/` folder is treated as a reference for desired syntax and semantics.

Files:
- `example_app/main.rs`
- `example_app/app.rs`
- `example_app/counter.rs`

## Next Steps

See `TODOS.md` for an implementation roadmap and prioritized tasks.
