# GUI Library Decision: egui/eframe

- **Date:** 2026-02-17
- **Status:** Accepted

## Decision

egui/eframe is Beltane's GUI library.

## Context

- Phase 1 vertical slice is complete; the GUI is the next milestone per `BELTANE_HEX_ARCH_PLAN.md`
- Beltane needs custom rendering for piano roll, waveforms, and mixer — visual richness is the top priority
- Ran a structured spike: the same app (transport controls + animated waveform) was built in iced, egui, and slint

## Candidates — what the spike revealed

**iced (~265 LOC)**
Strong Elm-like architecture with the best modifier-key detection of the three. However, verbose — the `canvas::Program` trait requires generic `Message`/`Theme`/`Renderer` threading, projection needs an `mpsc` channel, import name collisions are common, and the `tokio` feature is required just for a timer.

**egui (~170 LOC)**
Immediate-mode `Painter` API maps directly to DAW primitives. Modifier keys work via the `modifiers` field on key events. `Arc<Mutex<T>>` projection is straightforward. The `&mut self` update model eliminates type ceremony.

**slint (~230 LOC)**
Declarative markup is good for layout, but waveform rendering requires serializing SVG path strings every frame. `Rc<RefCell<>>` is needed for runtime access. Hex color literals like `#14141e` collide with Rust exponent-literal syntax inside the inline macro.

## Why egui won

- Fewest lines for equivalent functionality
- `Painter` + `Shape::line()` maps directly to DAW rendering (polylines, rects, fills)
- Modifier keys available out of the box on every key event
- No generic type machinery — plain `&mut self` update loop
- `Arc<Mutex<T>>` projection fits the hex architecture naturally
- Fastest iteration speed for visual experimentation

## Known risks & mitigations

Immediate-mode full-tree redraw is the primary concern (ref: Billy DM's "DAW Frontend Development Struggles" blog post).

| Risk | Mitigation |
|---|---|
| Full-tree redraw at DAW-scale complexity | Arrangement view shows blocks, not rendered waveforms/MIDI (level-of-detail, carried from Imbolc) |
| Detail rendering cost | Piano roll / waveform editors are focused single-panel views, not full-screen redraws |
| Live metering (VU etc.) | Small regions + simple geometry = egui's sweet spot |
| Future scalability | Hex architecture keeps the GUI adapter swappable |
| Blog post age | egui caching has improved significantly since the Meadowlark era |

## Architectural constraints (restated from hex arch plan)

- GUI is an inbound adapter only — sends `Command`s, reads projections
- Audio metering data flows via a lock-free port, not through egui's state
- No domain logic in the GUI crate
- `beltane-gui/src/lib.rs` retains the `GuiAction -> Command` adapter mapping

## References

- `BELTANE_HEX_ARCH_PLAN.md` &sect;GUI Strategy
- Spike promoted into `crates/beltane-gui/` (egui code in `src/main.rs`, adapter boundary in `src/lib.rs`)
