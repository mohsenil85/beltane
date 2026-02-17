# Beltane: High-Level Hexagonal Architecture Plan

## Mission

Beltane is a from-scratch rewrite of Imbolc with four explicit goals:

1. Feature and philosophy parity with Imbolc.
2. A first-class GUI experience.
3. Fully modular architecture.
4. Strict hexagonal architecture boundaries.

"Feature and philosophy parity" means preserving Imbolc's product identity (keyboard-first power workflow, realtime responsiveness, stability-first engineering) while reimplementing the system with cleaner module boundaries.

## Product Posture

- Beltane is not a patch series on current Imbolc internals; it is a deliberate new architecture.
- Existing Imbolc remains the reference product while Beltane reaches parity.
- Legacy GUI (`imbolc-gui`) should be treated as non-blocking reference material, not the primary path.
- GUI in Beltane is required, but core logic must remain GUI-library-agnostic.

## Architecture Decision

Beltane adopts a strict hexagonal architecture:

- Core logic lives inside the hexagon.
- All I/O (GUI, audio backend, persistence, MIDI, network) lives in adapters outside the hexagon.
- The core communicates only through ports and typed contracts.

Primary rule: UI never mutates engine internals directly; it sends commands and reads projections.

## High-Level Crate Topology

Current scaffold aligns to this target:

- `beltane-domain`
  - Pure domain model and invariants.
  - No I/O, no GUI/audio/backend types.
- `beltane-ports`
  - Command/event/effect contracts.
  - Trait ports for outbound dependencies.
- `beltane-application`
  - Use-case orchestration over domain.
  - Handles command -> state transition -> events/effects/projections.
- `beltane-runtime`
  - Wiring/composition layer.
  - Routes effects to adapters; feeds results back as commands/events.
- `beltane-gui`
  - Inbound adapter boundary for GUI actions.
  - Maps GUI intents to application commands.

Planned next adapters:

- `beltane-adapter-audio-sc` (SuperCollider bridge)
- `beltane-adapter-persistence-sqlite`
- `beltane-adapter-midi`
- `beltane-adapter-net` (optional until parity target requires it)

## Dependency Rules (Non-Negotiable)

- `domain` depends on nothing project-specific.
- `application` depends on `domain` + `ports` contracts only.
- `runtime` depends on `application` + `ports`.
- Adapters depend on `ports` and external libraries.
- GUI depends on command/projection contracts; never on backend internals.
- No core crate may import adapter crates.

## Core Flow Contract

For every feature module:

1. Inbound adapter emits `Command`.
2. Application handles command using domain state.
3. Domain emits domain event(s).
4. Application maps to app event(s) + effect(s).
5. Runtime applies effects through outbound ports.
6. Runtime publishes projection/view model for GUI.

Current vertical slice already follows this (`TogglePlay`).

## Module Decomposition Target

Beltane domain/application should be organized by feature module, each with its own command/event/effect/projection surface:

1. Transport.
2. Timeline/arrangement.
3. Piano roll.
4. Drum sequencer.
5. Mixer and routing graph.
6. Instrument and FX chain.
7. Automation.
8. Project persistence and file operations.
9. Optional collaboration/networking.

Each module must be independently testable with fake ports.

## Realtime and Concurrency Rules

- Audio callback path must stay lock-light and allocation-minimal.
- No filesystem/network/UI work in callback-critical paths.
- Control-plane commands cross thread boundaries via explicit queues.
- Audio feedback returns as typed events, then becomes UI projection data.
- Keep "incremental projection first, full sync fallback" strategy from Imbolc philosophy.

## GUI Strategy

- Beltane must ship with GUI, but GUI library should remain swappable.
- Choose library early (e.g., `iced`, `egui`, `slint`) based on desired tradeoff:
  - `iced`: strongest architectural fit with message/update model.
  - `egui`: fastest iteration and tooling ergonomics.
  - `slint`: stronger product UI polish path.
- Regardless of choice, GUI acts as inbound adapter only.

## Feature/Philosophy Parity Scope

Parity scope should include (at minimum):

- Sequencing and arrangement workflows.
- Instrument + FX editing and routing.
- Mixer operations.
- Automation editing and playback.
- Save/load/export flows.
- Realtime feedback/monitoring loops.
- Stability-first behavior under load.

Philosophy parity means preserving:

- Keyboard-first power-user workflow.
- Predictable latency and timing discipline.
- Clear separation of state mutation and side effects.
- Robust undo/redo and persistence correctness.

## Phased Plan

### Phase 0: Architecture Guardrails

- Lock dependency rules and crate boundaries in writing.
- Add CI checks/lints where possible to prevent forbidden imports.
- Define command/event/effect naming conventions.

### Phase 1: Vertical Slice Template (Done/Started)

- Keep `TogglePlay` as canonical example.
- Ensure all new features follow same flow contract.

### Phase 2: Transport + Timing Foundations

- Add BPM/set-playhead/loop commands and validations.
- Define timing projection contract for GUI.
- Add fake audio adapter tests first, then real adapter wiring.

### Phase 3: Timeline/Piano Roll Core

- Build arrangement and note-editing domain models.
- Add projections optimized for GUI rendering.
- Validate undo/redo boundaries in application layer.

### Phase 4: Instruments/FX and Routing

- Introduce modular signal graph model.
- Add effects as domain modules, not ad-hoc engine calls.
- Keep backend-specific details in audio adapter.

### Phase 5: Persistence and Recovery

- Add persistence ports and sqlite adapter.
- Define versioned project schema strategy.
- Rebuild autosave/recovery behavior with port-based I/O.

### Phase 6: Automation and Advanced Editing

- Reintroduce automation lanes and playback semantics.
- Ensure module-level invariants and deterministic tests.

### Phase 7: GUI Depth and UX Parity

- Expand GUI panes/workflows to parity target.
- Keep projection layer explicit; no domain leakage.

### Phase 8: Optional Networking and Collaboration

- Reintroduce collaboration via dedicated adapter.
- Maintain local single-user quality as primary gate.

### Phase 9: Stabilization and Parity Sign-Off

- Regression test coverage for critical music workflows.
- Performance and latency benchmarks against Imbolc baseline.
- Feature parity checklist closure before migration messaging.

## Quality Gates per Phase

- `cargo test` green for Beltane workspace.
- Module-level tests for domain invariants.
- Application tests for command -> event/effect contracts.
- Runtime tests for effect routing correctness.
- No boundary violations across crates.

## Near-Term Next Steps

1. Add second vertical slice: `SetBpm` with domain validation and audio sync effect.
2. Define standard error model for command handling (`Result<HandleOutput, DomainError>` pattern).
3. Add adapter crate stubs for audio and persistence to lock interfaces early.
4. Select GUI library and create minimal event loop adapter using existing command mapping boundary.

## Working Principles

- Prefer explicit contracts over convenience coupling.
- Keep slices thin and demonstrably testable.
- Do not bypass ports "temporarily"; temporary shortcuts become architecture debt.
- Optimize for long-term maintainability over short-term feature velocity.
