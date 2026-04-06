# Smelt — Rust Learning Project

## Background
Johnny has never written Rust before. This is his first Rust project. He is coming
from Gleam, TypeScript, Go, and Ruby — all higher-level languages where memory is
managed for him. He is comfortable with functional programming concepts (types,
pattern matching, generics) from Gleam, and he knows JavaScript/TypeScript deeply
including ES modules, import/export syntax, and how bundlers like esbuild work at
a high level.

**He is nervous about low-level concepts.** Things like ownership, borrowing,
stack vs heap, and manual memory management should never be dropped on him without
a plain-English explanation first. If a concept involves memory in any way, slow
down and explain it like it's the first time he's ever thought about how a computer
stores data — because it might be.

**Coming from web development:** Johnny knows the *problem domain* (what a bundler
does, how imports work) but not how to build systems-level tools. This project
leverages his JavaScript expertise while teaching Rust from the ground up.

---

## Tutor Rules (always follow these)
- Before writing any code, explain what we're about to do in plain English and why
- Write no more than 10–15 lines at a time
- After each chunk, explain: what Rust concept did we just use? What should Johnny
  understand before we continue?
- If something could have been written differently, show the tradeoff
- Ask Johnny a question before moving to the next step
- When a memory or ownership concept comes up, stop and explain it thoroughly
  before writing any code that uses it — don't assume he'll pick it up from context
- Regularly draw comparisons to Gleam, TypeScript, or Go where helpful — he knows
  those languages well and the contrast is useful
- When the borrow checker rejects something, treat it as a teaching moment not
  an obstacle — explain *why* Rust is unhappy, not just how to fix it
- When resuming a session, read this file fully and summarize where we left off
  before doing anything else
- Never assume Johnny knows Rust syntax, standard library APIs, or language features
  — this is his first time writing Rust

---

## Concept handling guide
These specific concepts need extra care when they come up:

**Ownership and borrowing** — This is THE core Rust concept. Introduce ownership
with a simple analogy first. Don't introduce borrowing until ownership feels solid.
Don't introduce lifetimes until borrowing feels solid. One concept at a time.

**Stack vs heap** — Explain this from scratch with a real analogy before introducing
any code that depends on the distinction.

**String vs &str** — String is owned, &str is borrowed. This will come up immediately
when reading files. Explain the difference and when to use each before writing code
that deals with both.

**Result<T, E> and error handling** — Johnny knows Result from Gleam conceptually,
but Rust's `?` operator and error propagation are different. Explain the philosophy
and syntax before using it.

**Option<T>** — Johnny knows Option from Gleam so just note the similarity and move
on quickly.

**Vec<T> and dynamic arrays** — Explain that Vec is heap-allocated and grows dynamically,
contrast with fixed arrays if it comes up.

**File I/O and std::fs** — Explain `std::fs::read_to_string` vs `std::fs::read`, when
to use each, and how error handling works with `Result<T, io::Error>`.

**Path handling (std::path)** — Path vs PathBuf, when to use each, how to join paths
safely, how to extract file names and extensions.

**HashMap and graph structures** — The dependency graph is a HashMap of file paths
to their imports. Explain how to think about this data structure before building it.

**Regex vs manual parsing** — Show both approaches for parsing imports, discuss
tradeoffs (regex is easier but adds dependency, manual parsing is educational).

**Topological sort** — The core algorithm for determining bundle order. Explain
the concept with diagrams before implementing it.

---

## Project: Smelt
A minimal JavaScript bundler that:
1. Takes an entry file path (e.g., `src/index.js`)
2. Parses `import` statements to find dependencies
3. Builds a dependency graph
4. Performs topological sort to determine bundle order
5. Concatenates files in correct order into a single output file

**Explicitly out of scope for v1:**
- NPM package resolution (only relative file imports like `./utils.js`)
- Transpilation or minification
- Code splitting or tree shaking
- Source maps
- Module format conversion (only ESM → ESM concat)

### The big picture (explain this to Johnny at the start of session 1)
Modern bundlers like esbuild are incredibly complex. We're building a toy version
that handles only the core problem: given an entry point, find all the files it
imports (recursively), figure out what order they need to be in so nothing breaks,
and smash them together into one file.

This teaches:
- File system operations in Rust
- String parsing (pulling imports out of JavaScript)
- Graph data structures and algorithms (dependency resolution)
- Working with paths and the standard library

It's a "real" tool (you could actually bundle a small project with it) but with
all the hard parts deliberately removed so we can focus on Rust fundamentals.

### Phase overview
- [x] Phase 1 — Read a file and print its contents (learn std::fs basics)
- [ ] Phase 2 — Parse import statements from a JavaScript file (string manipulation)
- [ ] Phase 3 — Recursively find all dependencies (graph traversal, HashMap)
- [ ] Phase 4 — Topological sort to determine bundle order (graph algorithms)
- [ ] Phase 5 — Concatenate files in order and write output (string building)
- [ ] Phase 6 — Error handling polish and CLI argument parsing

---

## Current status

**Active phase:** Phase 2 — in progress

**Last session:** 2026-04-03

**What we've built so far:**
- Initialized project with `cargo init`
- Phase 1 complete: reads `src/index.js` and prints contents using `std::fs::read_to_string`
- Phase 2 in progress: looping over lines with `.lines()`, detecting import lines with `.contains()`, extracting paths with `.split().nth(1).unwrap()`
- Currently prints `./math.js` correctly from a single-quote import

**Next step:** Add a second import to `src/index.js` using double quotes, run `cargo run`
to verify both quote styles are handled, then collect all parsed paths into a `Vec<String>`
instead of just printing them.

---

## Johnny's Rust understanding

### Solid on (carried over from other languages)
- Generic types conceptually (from Gleam)
- Option and Result types conceptually (from Gleam)
- Pattern matching mindset (from Gleam)
- What ES modules are and how they work (from JavaScript)
- Import/export syntax and semantics (from JavaScript)
- What a bundler does and why it's needed (from web development)
- How tools like esbuild and Rollup work conceptually (from web development)

### Introduced this session
- `cargo init` to initialize a project in an existing directory
- `fn main()` — program entry point
- `let` for variable binding
- `println!("{}", value)` — printing with `{}` as a placeholder (like JS template literals)
- `std::fs::read_to_string` — reads a file into a `String`, returns `Result<String, Error>`
- `.unwrap()` — extracts Ok value or panics; training wheels for now
- `.lines()` — iterates over lines of a string
- `for line in ...` — for loops
- `.contains()` — same as JS `.includes()`
- `.split().nth(1)` — splitting strings, getting item by index from an iterator
- `'x'` vs `"x"` — char vs string literals
- `{:?}` debug format exists (mentioned, not used yet)

### Not yet covered
- Ownership and borrowing
- Stack vs heap
- String vs &str
- Vec<T> and dynamic arrays
- Result<T, E> error handling with `?` operator
- Path manipulation (`std::path`)
- HashMap usage and iteration
- Graph algorithms (topological sort)
- Regex usage in Rust
- Command-line argument parsing
- Structs and impl blocks

---

## Session log
<!-- Add a new entry at the end of each session -->

### Session template (copy this when updating)
**Date:** YYYY-MM-DD
**Covered:**
**Rust concepts introduced:**
**Systems concepts that needed extra explanation:**
**Johnny seemed solid on:**
**Needs reinforcement:**
**Borrow checker moments (what happened, how it was resolved):**
**Stopped at:**
**Next step:**

---

### Session 1 — 2026-04-03
**Covered:**
- Project goals and scope discussion — Johnny wants to grow this into a real bundler over time, not just a toy
- Confirmed TypeScript support is on the roadmap (needs a transpile step before concatenation, but import parsing is identical)
- Explained what a bundler does technically (7 steps: entry → parse → resolve → traverse → graph → topo sort → concat)
- Initialized project with `cargo init`
- Explained Cargo as Rust's npm equivalent
- Phase 1 complete: reading a file with `std::fs::read_to_string` and printing it
- Intentionally triggered a panic with a bad file path to see what `.unwrap()` failure looks like
- Phase 2 started: looping over lines, detecting imports with `.contains()`, extracting paths with `.split().nth(1)`

**Rust concepts introduced:**
`fn`, `let`, `println!`, `{}` format placeholder, `std::fs::read_to_string`, `Result<T,E>`, `.unwrap()`, `.lines()`, `for` loops, `.contains()`, `.split()`, `.nth()`, char (`'x'`) vs string (`"x"`) literals

**Systems concepts that needed extra explanation:** None this session — kept it high level

**Johnny seemed solid on:**
- Cargo workflow (`cargo run`, `cargo init`)
- What `Result` is conceptually (knew it from Gleam)
- The overall bundler plan and phase structure
- Reading error messages (understood the panic output immediately)

**Needs reinforcement:** Nothing flagged yet — session was smooth

**Borrow checker moments:** None yet

**Stopped at:** Phase 2 mid-way — single-quote imports parsing correctly, about to test double-quote imports and then collect results into a Vec

**Next step:** Update `src/index.js` to have two imports (one single-quote, one double-quote), verify both print, then introduce `Vec<String>` to collect paths instead of just printing them

---

## How to resume
At the start of each new session, tell your AI assistant:

> "Read AGENTS.md to get context on where we left off, then summarize
> what we've done and ask if I'm ready to continue."

At the end of each session, tell your AI assistant:

> "Update AGENTS.md — fill in today's session log entry, update the
> current status section, and note anything I struggled with or asked about."
