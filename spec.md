# VØSK Language Specification
### Version 1.0

---

## Preamble

Most languages describe what a machine should do.
VØSK describes what is known, what must remain true, and what happens when it doesn't.

This language was built for a specific problem: how do you write programs that reflect the actual state of knowledge rather than the assumed state of certainty? Traditional type systems tell you what a value *is*. VØSK tells you what is *known about it* — and what happens as that knowledge degrades.

Every value in VØSK exists somewhere on a spectrum between resolution and corruption. The runtime does not pretend otherwise. Programs are not instructions — they are negotiations between what must hold and what is falling apart.

This is the official specification for VØSK v1.0.

---

## Philosophy

**1. Knowledge is not binary.**
A value is not simply known or unknown. It may be known but degrading. It may have been known and is now corrupted. The type system must reflect this.

**2. Constraints are obligations, not checks.**
A constraint in VØSK is not a validation — it is a promise the runtime is responsible for keeping. If it breaks, the runtime responds. The program does not continue as if nothing happened.

**3. Contexts are sovereign.**
What happens inside a context stays inside a context unless explicitly transmitted outward. Corruption does not spread by accident. Transmission is always intentional.

**4. Decay is information.**
A decaying value is not an error. It is a signal. The rate of decay, the threshold of corruption, and the response to it are all first-class design decisions in VØSK.

**5. Transmission over return.**
VØSK programs do not return values. They transmit states. Output is directional — inward to a parent, outward to all related contexts, or emitted beyond the program entirely.

---

## File Format

VØSK source files use the `.vsk` extension.
The compiler binary is `vosk`.
Programs are UTF-8 encoded plaintext.

---

## CLI

```
vosk run <file.vsk>     execute a VØSK program
vosk check <file.vsk>   parse and validate without executing
```

---

## Comments

Comments begin with `#` and extend to the end of the line.
They are stripped by the lexer and have no effect on execution.

```vsk
# this is a comment
~ signal {
    src :: ?   # inline comment
}
```

---

## Epistemic State System

VØSK does not have traditional types. Values are typed by their **epistemic state** — what is known about them, not what they are.

| State | Symbol | Meaning |
|-------|--------|---------|
| Unknown | `?` | exists, but unresolved |
| Resolved | `+` | known and stable |
| Decaying | `%` | known, but degrading toward corruption |
| Corrupted | `x` | was known, now invalid |

### Resolved Values

A resolved state may carry a concrete value:

```vsk
name     :: + "ilya"
frequency :: + 40
signal    :: +
```

A binding with a string or numeric value is considered resolved. A bare `+` indicates resolved without a specific value.

### Decaying Values

A decaying state carries a numeric decay level between `0.0` and `1.0`.
Each execution tick subtracts `0.25` from the decay level.
When the decay level reaches `0.0`, the value transitions to `Corrupted`.

```vsk
coherence :: % 0.75   # three ticks before corruption
memory    :: % 0.50   # two ticks before corruption
certainty :: % 0.25   # one tick before corruption
```

If no decay level is specified, the default is `0.25` — one tick from corruption.

```vsk
signal :: %   # equivalent to signal :: % 0.25
```

---

## Core Constructs

### 1. Context `~`

The fundamental unit of VØSK. A context is a named, bounded field of related values, constraints, and observations.

```vsk
~ name {
    # bindings, constraints, observations
}
```

Contexts are sovereign. Bindings declared inside a context are scoped to that context and do not persist beyond it. Corruption inside a context does not propagate to other contexts unless explicitly transmitted.

A program may contain multiple contexts. They execute in declaration order.

```vsk
~ alpha {
    signal :: +
}

~ beta {
    signal :: ?
}
```

### 2. Binding `::`

A binding associates a name with an epistemic state.

```vsk
name :: state
name :: state value
```

Examples:

```vsk
src       :: ?
signal    :: +
label     :: + "active"
frequency :: + 40
coherence :: % 0.75
status    :: x
```

Bindings are evaluated in declaration order. A name may only be bound once per context.

### 3. Constraint `!`

A constraint declares a truth that must hold within a context.
If the constraint is violated, the context is marked as corrupted.

```vsk
! name :: state
```

Examples:

```vsk
! signal :: +       # signal must be resolved
! src    :: ?       # src must be unknown
```

Constraints are evaluated after bindings. A violated constraint does not halt execution — remaining statements are evaluated, then the context is marked `[CONTEXT CORRUPTED]`.

The following binding states satisfy a resolved constraint `! name :: +`:

- `+` (bare resolved)
- `+ "string"` (resolved with string value)
- `+ number` (resolved with numeric value)

### 4. Observation `@`

An observation watches a binding for a specific state condition.
It is passive — it does not force resolution or change state.
When the condition is met, it may fire a transmission.

```vsk
@ name :: state
@ name :: state ~> transmission
```

Examples:

```vsk
@ signal :: x                        # watch for corruption, no transmission
@ signal :: x ~> emit "LOST"        # watch and emit on corruption
@ signal :: x ~> ^ "ESCALATE"      # watch and escalate to parent
@ signal :: x ~> * "PROPAGATE"     # watch and propagate to all
```

Observations are evaluated twice per context execution:

1. During initial evaluation, against the declared binding states
2. After the decay tick, against post-decay states

### 5. Transmission `~>`

Transmission is VØSK's output primitive. Values do not return — they transmit.

```vsk
~> emit "message"    # emit outward beyond the program
~> ^ "message"       # escalate to parent context
~> * "message"       # propagate to all related contexts
~> "message"         # local transmission
```

Transmissions are fired by observations when their condition is met.
A transmission carries a string message and a directional scope.

| Scope | Symbol | Behavior |
|-------|--------|---------|
| Emit | `emit` | fires outward, visible as terminal output |
| Escalate | `^` | surfaces to the parent context |
| Propagate | `*` | spreads to all related contexts |
| Local | *(none)* | contained within current scope |

### 6. Receiver `^NAME`

A receiver binding declares that a context expects a transmission from another context.
Receivers are resolved implicitly — no explicit wiring required.
When a transmission matching the receiver name fires anywhere in the program,
the receiver binding transitions to `Resolved` before the context executes.

```vsk
~ beta {
    ^SIGNAL_LOST :: ?

    ! ^SIGNAL_LOST :: +
    @ ^SIGNAL_LOST :: + ~> emit "BETA_RECEIVED"
}
```

Receiver names are prefixed with `^` in the binding declaration.
The `^` prefix distinguishes receivers from regular bindings and prevents naming collisions.

**Receiver obligation:** a receiver that never receives a transmission remains in its
declared state. If a constraint requires it to be resolved and no transmission arrives,
the constraint is violated and the context corrupts.

Silence is not neutral. An unresolved receiver is a failed expectation.

**Transmission scope and receivers:**

| Scope | Resolves |
|-------|---------|
| `* "NAME"` | resolves `^NAME` in ALL other contexts |
| `^ "NAME"` | resolves `^NAME` in the NEXT context in declaration order |

---

## Runtime Behavior

### Execution Order

For each context, execution proceeds as follows:

1. Evaluate bindings in declaration order
2. Evaluate constraints against current binding states
3. Evaluate observations against current binding states
4. Execute decay tick — all decaying values subtract `0.25`
5. Check for newly corrupted values
6. Re-evaluate observations against post-decay states
7. Fire transmissions for triggered observations
8. If any constraint was violated, mark context `[CONTEXT CORRUPTED]`

### Decay Tick

Every decaying binding is decremented by `0.25` per execution cycle.

```
Decaying(0.75) → Decaying(0.50) → Decaying(0.25) → Corrupted
```

Corruption is terminal. A corrupted value does not recover.

### Context Isolation

Bindings are cleared between contexts. A value bound in `~ alpha` is not accessible in `~ beta`. Corruption in one context does not infect another.

The only cross-context communication is through transmission — and transmission carries messages, not state.

### Constraint Violation

A violated constraint marks the context as corrupted after full execution.
The program continues to the next context. Constraint violation is reported but does not halt the program.

---

## Error Model

### Parse Errors

Reported by `vosk check` and `vosk run` before execution begins.
Include line number and expected vs found token.

```
error in file.vsk: line 4: expected Bind but found Unknown
```

### Runtime Corruption

Not an error — an expected outcome. Contexts can and do corrupt.
Corruption is reported as `[CONTEXT CORRUPTED]` after context execution.

### Unknown Commands

The CLI reports unknown commands and suggests `help`.

---

## Symbol Reference

| Symbol | Name | Usage |
|--------|------|-------|
| `~` | Context | declare a context |
| `::` | Bind | bind a name to a state |
| `!` | Constraint | declare a required truth |
| `@` | Observe | watch a binding for a state |
| `~>` | Transmit | fire a transmission |
| `->` | Arrow | leads to (reserved) |
| `?` | Unknown | epistemic state |
| `+` | Resolved | epistemic state |
| `%` | Decaying | epistemic state |
| `x` | Corrupted | epistemic state |
| `^` | Escalate | transmission scope |
| `*` | Propagate | transmission scope |
| `#` | Comment | stripped by lexer |

---

## Reserved — Future

The following features are defined in this specification but not yet implemented in v1.0. They are reserved and may not be used as identifiers.

### Real-Time Decay
Decay will operate on wall-clock time rather than execution steps. A decaying value will degrade continuously while the program is running, enabling VØSK programs to function as living processes rather than scripts.

### Custom Tick Rates
Individual bindings will support custom decay intervals:

```vsk
coherence :: % 0.75 @ 500ms   # decay every 500 milliseconds
```

### Context Inheritance
Contexts will support inheritance from parent contexts, allowing shared constraints and observations to be declared once and applied to multiple child contexts.

### Import System
Programs will be able to import other `.vsk` files:

```vsk
import "signals.vsk"
```

### Standard Library
A set of built-in contexts covering common patterns — signal tracking, observer patterns, decay chains — will be distributed with the `vosk` binary.

### Bytecode Compilation
VØSK programs will compile to a portable bytecode format executable by the VØSK virtual machine, enabling distribution of compiled `.vskc` artifacts.

### LLVM Backend
A native compilation target via LLVM will produce optimized machine code from VØSK source, enabling VØSK programs to run without the interpreter.

---

## Versioning

This document describes **VØSK v1.0**.

The language is under active development. Features marked **Reserved** are planned and may appear in future versions. The core epistemic state system, context model, constraint system, observation model, and transmission model described in this document are stable and will not change without a major version increment.

---

## License

VØSK is released under the MIT License.
The language, compiler, and this specification are open source.

---

*a language where values decay and constraints must hold.*
