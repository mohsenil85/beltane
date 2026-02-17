# Beltane Test Migration Matrix

Date: 2026-02-17

This matrix classifies existing Imbolc test files for Beltane migration.

Legend:

- KEEP: Port mostly as-is; only rename/module path adjustments expected.
- ADAPT: Reuse behavioral scenarios, but rewrite harness/contracts for hex architecture.
- DROP: Do not port directly; replace with new Beltane tests for equivalent outcomes.

## Inventory Summary

| Source crate | Test files | #[test] count |
|---|---:|---:|
| imbolc-types | 33 | 374 |
| imbolc-core | 32 | 317 |
| imbolc-audio | 9 | 96 |
| imbolc-net | 10 | 105 |
| imbolc-ui | 28 | 219 |
| Total | 112 | 1111 |

## First-Wave Migration (Do First)

| Priority | Source file | Decision | Beltane target |
|---:|---|---|---|
| 1 | `imbolc-core/src/state/automation/tests.rs` | ADAPT | beltane-domain + beltane-application (automation module) |
| 2 | `imbolc-core/src/state/persistence/tests/basic.rs` | ADAPT | beltane-adapter-persistence-sqlite |
| 3 | `imbolc-core/src/state/persistence/tests/decoders.rs` | ADAPT | beltane-adapter-persistence-sqlite |
| 4 | `imbolc-audio/src/bus_allocator.rs` | KEEP | beltane-adapter-audio-sc internals |
| 5 | `imbolc-audio/src/event_log.rs` | ADAPT | beltane-runtime projection/event stream |
| 6 | `imbolc-audio/src/playback.rs` | ADAPT | beltane-adapter-audio-sc scheduling |
| 7 | `imbolc-net/tests/protocol_roundtrip.rs` | ADAPT | beltane-adapter-net protocol tests |
| 8 | `imbolc-net/tests/handshake.rs` | ADAPT | beltane-adapter-net integration |
| 9 | `imbolc-types/src/tuning/mod.rs` | KEEP | beltane-domain::tuning |
| 10 | `imbolc-types/src/tuning/chord_detect.rs` | KEEP | beltane-domain::tuning |
| 11 | `imbolc-types/src/state/music.rs` | KEEP | beltane-domain::music |
| 12 | `imbolc-types/src/state/mixer.rs` | ADAPT | beltane-domain::mixer |
| 13 | `imbolc-core/src/dispatch/arrangement.rs` | ADAPT | beltane-application::arrangement |
| 14 | `imbolc-core/src/dispatch/sequencer.rs` | ADAPT | beltane-application::drum_seq |
| 15 | `imbolc-core/src/dispatch/mixer.rs` | ADAPT | beltane-application::mixer |

## Full File Matrix

| Source file | Tests | Decision | Beltane destination | Phase | Why |
|---|---:|---|---|---|---|
| `imbolc-types/src/state/instrument/mod.rs` | 63 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-audio/src/engine/mod.rs` | 52 | `ADAPT` | `beltane-adapter-audio-sc + beltane-runtime` | `P2-P6` | Realtime behavior must be preserved, but queueing and effect interfaces will change. |
| `imbolc-core/src/dispatch/arrangement.rs` | 35 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-net/tests/broadcast.rs` | 31 | `ADAPT` | `beltane-adapter-net` | `P8` | Protocol and ownership scenarios are valuable once networking is reintroduced. |
| `imbolc-core/src/dispatch/session.rs` | 30 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-net/src/server.rs` | 29 | `ADAPT` | `beltane-adapter-net` | `P8` | Protocol and ownership scenarios are valuable once networking is reintroduced. |
| `imbolc-net/tests/protocol_roundtrip.rs` | 28 | `ADAPT` | `beltane-adapter-net` | `P8` | Protocol and ownership scenarios are valuable once networking is reintroduced. |
| `imbolc-core/src/state/automation/tests.rs` | 25 | `ADAPT` | `beltane-domain::automation + beltane-application::automation` | `P6` | High-value behavior tests; port assertions to new module boundaries. |
| `imbolc-ui/src/ui/piano_keyboard.rs` | 24 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-core/src/dispatch/midi.rs` | 22 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-types/src/state/mixer.rs` | 19 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-types/src/param.rs` | 19 | `ADAPT` | `beltane-application + beltane-ports` | `P2-P6` | Reducer/contract semantics map well but command and effect contracts will be renamed. |
| `imbolc-types/src/state/instrument/effect.rs` | 18 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-core/src/state/persistence/tests/decoders.rs` | 18 | `ADAPT` | `beltane-adapter-persistence-sqlite` | `P5` | Round-trip and schema scenarios should be kept with adapter-level APIs. |
| `imbolc-types/src/state/music.rs` | 17 | `KEEP` | `beltane-domain` | `P3-P6` | Mostly pure musical behavior with minimal infrastructure coupling. |
| `imbolc-types/src/state/generative.rs` | 17 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-ui/src/repl/parse.rs` | 16 | `ADAPT` | `beltane-gui command adapter` | `P7` | Keep command parsing scenarios only if a command palette/console is retained. |
| `imbolc-ui/src/panes/piano_roll_pane/mod.rs` | 16 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-types/src/state/instrument_state.rs` | 16 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-core/src/dispatch/bus.rs` | 16 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-core/src/dispatch/automation.rs` | 16 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-types/src/tuning/mod.rs` | 15 | `KEEP` | `beltane-domain::tuning` | `P3` | Pure math and music-theory invariants are portable. |
| `imbolc-types/src/tuning/chord_detect.rs` | 15 | `KEEP` | `beltane-domain::tuning` | `P3` | Pure math and music-theory invariants are portable. |
| `imbolc-ui/src/repl/tests.rs` | 14 | `ADAPT` | `beltane-gui command adapter` | `P7` | Keep command parsing scenarios only if a command palette/console is retained. |
| `imbolc-types/src/reduce/generative.rs` | 14 | `ADAPT` | `beltane-application + beltane-ports` | `P2-P6` | Reducer/contract semantics map well but command and effect contracts will be renamed. |
| `imbolc-core/src/dispatch/sequencer.rs` | 14 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-types/src/state/arrangement.rs` | 13 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-core/src/state/undo.rs` | 13 | `ADAPT` | `beltane-application` | `P3-P5` | Same behavioral goals but ownership and composition model will differ. |
| `imbolc-ui/src/midi_dispatch.rs` | 12 | `ADAPT` | `beltane-gui + beltane-adapter-midi` | `P6-P7` | Input-to-command semantics remain relevant but runtime hooks will differ. |
| `imbolc-types/src/state/instrument/filter.rs` | 12 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-types/src/state/drum_sequencer.rs` | 12 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-core/src/dispatch/mixer.rs` | 12 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-ui/src/repl/registry.rs` | 11 | `ADAPT` | `beltane-gui command adapter` | `P7` | Keep command parsing scenarios only if a command palette/console is retained. |
| `imbolc-types/src/tuning/ratios.rs` | 11 | `KEEP` | `beltane-domain::tuning` | `P3` | Pure math and music-theory invariants are portable. |
| `imbolc-types/src/state/session.rs` | 11 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-types/src/action.rs` | 11 | `ADAPT` | `beltane-application + beltane-ports` | `P2-P6` | Reducer/contract semantics map well but command and effect contracts will be renamed. |
| `imbolc-ui/src/panes/mixer_pane/mod.rs` | 10 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/panes/instrument_pane.rs` | 10 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/panes/generative_pane.rs` | 10 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-types/src/state/vst.rs` | 10 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-core/src/state/grid.rs` | 10 | `KEEP` | `beltane-domain` | `P3-P6` | Deterministic state math that should transfer almost directly. |
| `imbolc-core/src/dispatch/piano_roll.rs` | 9 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-audio/src/generative_tick.rs` | 9 | `ADAPT` | `beltane-adapter-audio-sc + beltane-runtime` | `P2-P6` | Realtime behavior must be preserved, but queueing and effect interfaces will change. |
| `imbolc-ui/src/ui/widgets/text_input.rs` | 8 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/ui/layer.rs` | 8 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/ui/action_id.rs` | 8 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/panes/tuner_pane.rs` | 8 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-types/src/state/instrument/source_type.rs` | 8 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-types/src/state/instrument/lfo.rs` | 8 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-types/src/state/arpeggiator.rs` | 8 | `KEEP` | `beltane-domain` | `P3-P6` | Mostly pure musical behavior with minimal infrastructure coupling. |
| `imbolc-types/src/reduce/tag.rs` | 8 | `ADAPT` | `beltane-application + beltane-ports` | `P2-P6` | Reducer/contract semantics map well but command and effect contracts will be renamed. |
| `imbolc-core/src/state/persistence/checkpoint.rs` | 8 | `ADAPT` | `beltane-adapter-persistence-sqlite` | `P5` | Round-trip and schema scenarios should be kept with adapter-level APIs. |
| `imbolc-core/src/state/arpeggiator.rs` | 8 | `KEEP` | `beltane-domain` | `P3-P6` | Deterministic state math that should transfer almost directly. |
| `imbolc-core/src/midi/mod.rs` | 8 | `ADAPT` | `beltane-adapter-midi + beltane-application` | `P6` | MIDI scenarios remain valid; split between port adapter and app command handling. |
| `imbolc-ui/src/ui/list_selector.rs` | 7 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/ui/keybindings.rs` | 7 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/ui/filterable_list.rs` | 7 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/panes/quit_prompt_pane.rs` | 7 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/panes/midi_settings_pane.rs` | 7 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-core/src/scd_parser.rs` | 7 | `ADAPT` | `beltane-adapter-audio-sc` | `P4` | Parsing behavior useful if SuperCollider path retained; move under audio adapter tests. |
| `imbolc-audio/src/playback.rs` | 7 | `ADAPT` | `beltane-adapter-audio-sc + beltane-runtime` | `P2-P6` | Realtime behavior must be preserved, but queueing and effect interfaces will change. |
| `imbolc-audio/src/event_log.rs` | 7 | `ADAPT` | `beltane-adapter-audio-sc + beltane-runtime` | `P2-P6` | Realtime behavior must be preserved, but queueing and effect interfaces will change. |
| `imbolc-types/src/tuning/adaptive.rs` | 6 | `KEEP` | `beltane-domain::tuning` | `P3` | Pure math and music-theory invariants are portable. |
| `imbolc-types/src/state/piano_roll.rs` | 6 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-core/src/state/persistence/tests/mixer.rs` | 6 | `ADAPT` | `beltane-adapter-persistence-sqlite` | `P5` | Round-trip and schema scenarios should be kept with adapter-level APIs. |
| `imbolc-audio/src/engine/voice_allocator.rs` | 6 | `ADAPT` | `beltane-adapter-audio-sc + beltane-runtime` | `P2-P6` | Realtime behavior must be preserved, but queueing and effect interfaces will change. |
| `imbolc-ui/src/panes/instrument_edit_pane/mod.rs` | 5 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-types/src/state/theme.rs` | 5 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-types/src/state/param_tag.rs` | 5 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-types/src/state/midi_recording.rs` | 5 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-types/src/state/groove.rs` | 5 | `KEEP` | `beltane-domain` | `P3-P6` | Mostly pure musical behavior with minimal infrastructure coupling. |
| `imbolc-types/src/reduce/instrument.rs` | 5 | `ADAPT` | `beltane-application + beltane-ports` | `P2-P6` | Reducer/contract semantics map well but command and effect contracts will be renamed. |
| `imbolc-net/src/framing.rs` | 5 | `ADAPT` | `beltane-adapter-net` | `P8` | Protocol and ownership scenarios are valuable once networking is reintroduced. |
| `imbolc-core/src/state/persistence/tests/instruments.rs` | 5 | `ADAPT` | `beltane-adapter-persistence-sqlite` | `P5` | Round-trip and schema scenarios should be kept with adapter-level APIs. |
| `imbolc-core/src/state/mod.rs` | 5 | `ADAPT` | `beltane-application` | `P3-P5` | Same behavioral goals but ownership and composition model will differ. |
| `imbolc-core/src/interaction_log.rs` | 5 | `ADAPT` | `beltane-runtime` | `P2-P4` | Event-log behavior is relevant but should be runtime-owned in hex model. |
| `imbolc-core/src/dispatch/instrument/lfo.rs` | 5 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-core/src/dispatch/instrument/layer.rs` | 5 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-core/src/dispatch/instrument/filter.rs` | 5 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-core/src/dispatch/instrument/crud.rs` | 5 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-audio/src/bus_allocator.rs` | 5 | `KEEP` | `beltane-adapter-audio-sc` | `P4` | Low-level deterministic utility behavior is directly portable. |
| `imbolc-ui/src/ui/keymap.rs` | 4 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/panes/frame_edit_pane.rs` | 4 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/panes/automation_pane/mod.rs` | 4 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-types/src/state/sampler.rs` | 4 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-types/src/state/custom_synthdef.rs` | 4 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-core/src/state/recent_projects.rs` | 4 | `ADAPT` | `beltane-application` | `P3-P5` | Same behavioral goals but ownership and composition model will differ. |
| `imbolc-core/src/dispatch/instrument/envelope.rs` | 4 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-core/src/dispatch/instrument/effects.rs` | 4 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-core/src/config.rs` | 4 | `ADAPT` | `beltane-runtime + config adapter` | `P2-P5` | Config semantics reusable, but config loading must become an outbound adapter. |
| `imbolc-audio/src/triple_buffer.rs` | 4 | `KEEP` | `beltane-adapter-audio-sc` | `P4` | Low-level deterministic utility behavior is directly portable. |
| `imbolc-audio/src/engine/node_registry.rs` | 4 | `ADAPT` | `beltane-adapter-audio-sc + beltane-runtime` | `P2-P6` | Realtime behavior must be preserved, but queueing and effect interfaces will change. |
| `imbolc-ui/src/ui/status_bar.rs` | 3 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/panes/sequencer_pane.rs` | 3 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/panes/docs_pane/rendering.rs` | 3 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-net/tests/reconnection.rs` | 3 | `ADAPT` | `beltane-adapter-net` | `P8` | Protocol and ownership scenarios are valuable once networking is reintroduced. |
| `imbolc-net/tests/handshake.rs` | 3 | `ADAPT` | `beltane-adapter-net` | `P8` | Protocol and ownership scenarios are valuable once networking is reintroduced. |
| `imbolc-core/src/state/persistence/tests/arrangement.rs` | 3 | `ADAPT` | `beltane-adapter-persistence-sqlite` | `P5` | Round-trip and schema scenarios should be kept with adapter-level APIs. |
| `imbolc-core/src/dispatch/vst_param.rs` | 3 | `ADAPT` | `beltane-application` | `P2-P7` | Dispatch intent maps directly to use-cases, but hex ports replace direct side effects. |
| `imbolc-types/src/state/instrument/envelope.rs` | 2 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-net/tests/privilege.rs` | 2 | `ADAPT` | `beltane-adapter-net` | `P8` | Protocol and ownership scenarios are valuable once networking is reintroduced. |
| `imbolc-net/tests/multi_client.rs` | 2 | `ADAPT` | `beltane-adapter-net` | `P8` | Protocol and ownership scenarios are valuable once networking is reintroduced. |
| `imbolc-core/src/state/persistence/tests/basic.rs` | 2 | `ADAPT` | `beltane-adapter-persistence-sqlite` | `P5` | Round-trip and schema scenarios should be kept with adapter-level APIs. |
| `imbolc-audio/src/telemetry.rs` | 2 | `ADAPT` | `beltane-adapter-audio-sc + beltane-runtime` | `P2-P6` | Realtime behavior must be preserved, but queueing and effect interfaces will change. |
| `imbolc-ui/src/panes/vst_param_pane/mod.rs` | 1 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/panes/file_browser_pane.rs` | 1 | `DROP` | `beltane-gui` | `P7` | TUI-specific rendering/navigation tests do not transfer to the new GUI stack. |
| `imbolc-ui/src/network.rs` | 1 | `ADAPT` | `beltane-gui + beltane-adapter-net` | `P8` | UI network behaviors can inform future integration tests after net adapter exists. |
| `imbolc-types/src/state/recording.rs` | 1 | `ADAPT` | `beltane-domain` | `P3-P6` | Core behavior valuable, but data model and module boundaries will change. |
| `imbolc-types/src/state/humanize.rs` | 1 | `KEEP` | `beltane-domain` | `P3-P6` | Mostly pure musical behavior with minimal infrastructure coupling. |
| `imbolc-net/tests/ownership.rs` | 1 | `ADAPT` | `beltane-adapter-net` | `P8` | Protocol and ownership scenarios are valuable once networking is reintroduced. |
| `imbolc-net/src/discovery.rs` | 1 | `DROP` | `beltane-adapter-net` | `P8` | Discovery implementation likely to be redesigned; write fresh tests around final adapter contract. |
| `imbolc-core/src/state/persistence/blob.rs` | 1 | `ADAPT` | `beltane-adapter-persistence-sqlite` | `P5` | Round-trip and schema scenarios should be kept with adapter-level APIs. |

## Decision Totals (By File)

- KEEP: 12
- ADAPT: 76
- DROP: 24

## Notes

- ADAPT is the default for most valuable tests because Beltane intentionally changes architecture and contracts.
- DROP here means "rewrite against new GUI/runtime contracts," not "lose the behavior requirement."
- This matrix is intentionally phase-tagged to align with the Beltane roadmap (P2..P8).
