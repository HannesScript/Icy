# Icy TODOs

## Priority 0: Foundation

- [ ] Define crate module layout (`App`, `Component`, `StateManagement`, `Renderer`, `Events`)
- [ ] Add SDL3 dependency and verify compile/link setup on Linux
- [ ] Replace placeholder `src/main.rs` with framework bootstrap entry
- [ ] Decide naming conventions (Rust idiomatic vs framework-specific style)

## Priority 1: SDL3 Window + Loop

- [ ] Implement `IcyApp` core struct
- [ ] Implement `IcyWindow` config struct (width, height, title)
- [ ] Implement app lifecycle methods:
  - [ ] `create_icy_app()`
  - [ ] `mount_to_window(&window)`
  - [ ] `show_window()`
- [ ] Create SDL3 event loop abstraction
- [ ] Handle app quit and window close events

## Priority 2: Component System

- [ ] Define base `Component` type
- [ ] Define text node type (`Text`)
- [ ] Define primitive widgets (`h1`, `p`, `button`)
- [ ] Support nested children tree composition
- [ ] Implement validation for malformed component trees

## Priority 3: Rendering Pipeline

- [ ] Build tree traversal renderer
- [ ] Render text content to SDL3 surface/texture
- [ ] Render basic layout flow (vertical stack first)
- [ ] Add redraw scheduling and frame invalidation
- [ ] Add dirty-region or full-frame rerender strategy

## Priority 4: State Management

- [ ] Define `IcyState<T>` generic API
- [ ] Implement `new_state(...)` creation API
- [ ] Implement mutation/update notifications
- [ ] Trigger rerender on state changes automatically
- [ ] Prevent unsafe mutable aliasing patterns

## Priority 5: Events and Actions

- [ ] Define action callback model for widgets
- [ ] Wire `button` click events to state mutation callbacks
- [ ] Add keyboard and mouse input routing
- [ ] Add focus handling for interactive components

## Priority 6: Example App Integration

- [ ] Make `example_app/main.rs` compile against real framework code
- [ ] Implement working counter demo with increment/decrement
- [ ] Verify automatic rerender updates visible counter text
- [ ] Add an app screenshot/GIF once rendering works

## Priority 7: Developer Experience

- [ ] Add top-level crate docs with architecture overview
- [ ] Add API docs for each public module
- [ ] Add `cargo` run instructions for framework + example app
- [ ] Add contribution guidelines and coding style section

## Quality and Testing

- [ ] Add unit tests for state updates and component tree behavior
- [ ] Add integration tests for lifecycle and event loop glue
- [ ] Add smoke test that launches and closes a window cleanly
- [ ] Add CI checks (`cargo fmt`, `cargo clippy`, `cargo test`)

## Stretch Goals

- [ ] Simple layout system (padding, alignment, spacing)
- [ ] Theming support (colors, typography)
- [ ] Time-based animation hooks
- [ ] Multi-window support
- [ ] Hot-reload-like dev workflow
