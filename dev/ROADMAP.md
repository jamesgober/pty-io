# pty-io - Roadmap

> Path from scaffold to a stable 1.0. Hard parts are front-loaded; each phase has hard exit criteria.
> Master plan: C:/Dev/libraries/Rust/lang-collection/_lexersketch/ROADMAP.md and NEW-LIBS.md
>
> **Anti-deferral rule:** no listed hard task moves to a later phase unless this file records the move and the reason.

## v0.1.0 - Scaffold (DONE)
Compiles, CI green, structure correct, no domain logic.
- [x] Manifest, README, CHANGELOG, REPS, dual license, CI, deny, clippy, rustfmt, DIRECTIVES, ROADMAP.

## v0.2.0 - Foundation
- [ ] ConPTY and posix_openpt backends, spawn with env/cwd, read/write streams, resize, kill, reap; tests on all three OSes.

## v0.5.0 - Implementation
- [ ] Async adapter, process-group control, signal forwarding.

## v0.9.0 - Hardening
- [ ] Leak and teardown stress tests.

## v1.0.0 - Stable
- [ ] Frozen after Studio's terminal ships on it (D18).
