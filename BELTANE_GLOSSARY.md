# Beltane Glossary and Naming Contract

Date: 2026-02-17

## Purpose

This document defines the canonical vocabulary for Beltane.
It exists to keep the rewrite modular, consistent, and easier to maintain across domain, application, runtime, adapters, and GUI.

Goals aligned with Beltane:

- Feature/philosophy parity with Imbolc.
- GUI-first product.
- Fully modular architecture.
- Strict hexagonal architecture boundaries.

## Scope

This glossary applies to:

- Domain commands and events.
- Application-level effects.
- Runtime and adapter interfaces.
- GUI adapter action mapping.

It does not require end-user labels to match API names exactly, but API names should follow this contract.

## Canonical Verb Set

Use these verbs in command and effect names.

### State and Value

- `SetX`: absolute assignment.
- `AdjustX`: relative delta.
- `ToggleX`: boolean flip only.
- `EnableX` / `DisableX`: explicit boolean set (when toggle is ambiguous).
- `ResetX`: restore default/neutral value.
- `CycleX`: move to next value in finite cycle.
- `NextX` / `PrevX`: directional step in ordered lists.

### Lifecycle and Collections

- `CreateX`: create a top-level entity.
- `DeleteX`: destroy a top-level entity.
- `AddX`: add member/entry to existing collection.
- `RemoveX`: remove member/entry from existing collection.
- `RenameX`: rename entity.
- `MoveX`: reposition entity.
- `DuplicateX`: clone entity.

### Transport and Playback

- `StartPlayback` / `StopPlayback`: explicit transport control.
- `TogglePlayback`: allowed in UI adapters, avoid in core where explicit is possible.
- `StartRecording` / `StopRecording`: explicit recording control.

### I/O and Integration

- `SaveX`: write state.
- `LoadX`: read structured state.
- `ImportX`: ingest external assets/plugins.
- `ExportX`: write derived artifacts (stems, bounce, reports).
- `RenderX`: offline render/synthesis operation.
- `ConnectX` / `DisconnectX`: connection lifecycle.
- `DiscoverX`: query/scan metadata.

### UI-only Verbs (adapter layer only)

- `OpenX`, `CloseX`, `FocusX`, `SwitchX`, `NavigateX`.

These are not domain commands.

## Canonical Noun Set

### Core Music Model

- `Session`
- `Project`
- `Transport`
- `Arrangement`
- `Track`
- `Clip`
- `ClipInstance` (preferred over ambiguous `Placement`)
- `PianoRoll`
- `DrumSequencer`
- `AutomationLane`
- `AutomationPoint`

### Sound and Routing

- `Instrument`
- `Sampler`
- `Effect`
- `Filter`
- `Envelope`
- `Lfo` (code may use `Lfo`; docs can say LFO)
- `Equalizer` (short `Eq` acceptable in UI labels)
- `Bus`
- `Mixer`
- `ChannelStrip`
- `LayerGroup` (or renamed `InstrumentGroup` if adopted globally)

### Timing and Expression

- `Swing`
- `Humanize`
- `Groove`
- `Arpeggiator`

### External Systems

- `Midi`
- `Plugin` (domain-preferred umbrella)
- `Vst` (adapter-specific; avoid as top-level domain noun)
- `Server`
- `Checkpoint`
- `Tag`

### Sample Editing

- `SampleSlice`
- `SampleSliceEditor` (domain-preferred over ambiguous `Chopper`)

## Naming Patterns by Layer

### Domain Commands

Pattern: `VerbNoun` in PascalCase.

Examples:

- `SetBpm`
- `AdjustPan`
- `ToggleMute`
- `CreateClip`
- `DeleteClip`
- `AddEffect`
- `RemoveEffect`

### Domain Events

Pattern: past tense or completed-state form.

Examples:

- `PlaybackStarted`
- `PlaybackStopped`
- `BpmSet`
- `EffectAdded`
- `ClipDeleted`
- `AutomationPointMoved`

### Application Effects

Pattern: imperative side effect with explicit target.

Examples:

- `SyncAudioTransport`
- `PersistProject`
- `LoadPluginMetadata`
- `PublishTransportView`

### Adapter/UI Action Mapping

Adapter actions may use UI wording, but must map to canonical domain commands.

Example:

- UI action `PlayStopPressed` -> domain `StartPlayback` or `StopPlayback` (or `TogglePlayback` if state-based dispatch is intentional).

## Rules (Enforced)

1. Do not use UI navigation verbs in domain commands.
2. Do not mix `Set` and `Adjust` semantics in one command.
3. Do not use `Toggle` for non-boolean states.
4. Prefer explicit start/stop for long-running operations.
5. Use one noun for one concept across modules.
6. Avoid introducing new abbreviations in domain APIs.
7. Keep snake_case only for keybinding/CLI string literals; keep PascalCase for Rust type/variant names.

## Deprecated -> Preferred Mapping

| Current / Legacy | Preferred in Beltane | Notes |
|---|---|---|
| `PlayStop` | `StartPlayback` / `StopPlayback` or `TogglePlayback` | Prefer explicit start/stop in core. |
| `CycleXReverse` | `PrevX` | Use directional pair `NextX` + `PrevX`. |
| `UpdateX` (generic) | `SetX` or `AdjustX` | Choose exact semantics. |
| `Open*` in domain actions | move to UI adapter layer | `Open` is UI intent, not domain mutation. |
| `Placement` | `ClipInstance` | Clearer arrangement noun. |
| `Chopper` (domain) | `SampleSliceEditor` / `SampleSlice` | Keep `Chopper` only as optional UI label. |
| `Vst*` top-level domain noun | `Plugin*` (domain), `Vst*` (adapter) | Keep protocol/backend details outside domain. |
| `RecordMaster` (toggle semantics) | `ToggleMasterRecording` or `StartMasterRecording`/`StopMasterRecording` | Avoid ambiguous lifecycle naming. |
| mixed `Arp`/`Arpeggiator` in domain surface | `Arpeggiator` | Keep abbreviations to UI shortcuts. |
| mixed `Eq`/`Equalizer` in domain surface | `Equalizer` | `Eq` acceptable for UI labels only. |

## Compatibility and Aliases

To preserve user ergonomics and parity:

- REPL/command palette may keep legacy aliases (`play-stop`, `cycle-*-rev`, etc.).
- Aliases must normalize to canonical Beltane commands before entering application logic.
- Internal tests should assert canonical commands/events/effects, not alias spellings.

## PR Checklist for New Names

Before adding a new command/event/effect name:

1. Does the verb match the canonical set?
2. Is the noun already used elsewhere for the same concept?
3. Is the command domain-level (not UI intent)?
4. Is the semantic explicit (`Set` vs `Adjust`, `Start` vs `Toggle`)?
5. Is there a migration alias needed for parity with Imbolc UX?

## Immediate Beltane Usage

For current scaffold and near-term slices:

- Keep existing `TogglePlay` as a bootstrap alias.
- Introduce canonical transport pair next: `StartPlayback`, `StopPlayback`.
- Add adapter normalization so UI inputs can still emit `TogglePlay` while domain converges on canonical verbs.
