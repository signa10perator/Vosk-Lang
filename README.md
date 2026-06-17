# VØSK

A compiled language where the primitive unit is a context, not a variable.
Values hold epistemic state: resolved, unknown, decaying, corrupted.
Programs are negotiations between resolution and entropy.

---

## Philosophy

Most languages describe what a machine should do.
VØSK describes what is known, what must be true, and what happens when it isn't.

Values are not typed by what they *are* — they are typed by what is *known about them*.
A value is not an integer. It is resolved, unknown, decaying, or corrupted.
The runtime's job is to move values toward resolution.
Constraints force it. Decay fights it. Corruption ends it.

---

## Epistemic States

| State | Symbol | Meaning |
|-------|--------|---------|
| Unknown | `?` | exists, but unresolved |
| Resolved | `+` | known and stable |
| Decaying | `%` | known, but degrading |
| Constrained | `!` | must resolve |
| Contextual | `~` | only valid inside a context |
| Corrupted | `x` | was known, now invalid |

---

## Core Constructs

### Context `~`
The fundamental unit. A named field of related values and constraints.
A context has state. It can resolve, decay, or corrupt.

```vsk
~ anomaly {
    src  :: ?
    freq :: % 40
    signal :: ! +
}
```

### Constraint `!`
A truth that must hold. If it breaks, the context corrupts.
The runtime is always trying to satisfy constraints.

```vsk
! signal -> resolved
! freq :: + => emit "STABLE"
```

### Observation `@`
Watches a relationship between values. Passive — does not force resolution.
Responds when state changes.

```vsk
@ freq :: x ~> ^ "SIGNAL_LOST"
```

### Transmission `~>`
Values do not return. They transmit.
Outward to related contexts, upward to parents, or emitted externally.

```vsk
! src :: + ~> * "ORIGIN_FOUND"
```

---

## Example

```vsk
~ signal {
    src    :: ?
    freq   :: % 40
    noise  :: + 1
    stable :: ! +

    ! noise -> ?     ~> % freq
    @ freq  :: x     ~> ^ "SIGNAL_LOST"
    ! src   :: +     ~> * "ORIGIN_CONFIRMED"
    ! stable         ~> emit "TRANSMISSION_STABLE"
}
```

Reading it:

- A signal exists. Its source is unknown. Its frequency is decaying from 40. Noise is present.
- When noise resolves to unknown, frequency begins decaying.
- If frequency corrupts, escalate `SIGNAL_LOST` to the parent context.
- When source resolves, propagate `ORIGIN_CONFIRMED` to all related contexts.
- When stable resolves, emit `TRANSMISSION_STABLE` outward.

---

## File Extension

`.vsk`

## Binary

`vosk`

```bash
vosk run main.vsk
vosk build main.vsk
vosk check main.vsk
```

---

## Status

Early design and compiler development.
The language specification is being written alongside the implementation.
Contributions are not open yet — the core architecture is still being established.

---

## License

MIT
