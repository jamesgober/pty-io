//! # pty_io
//!
//! Pseudo-terminals for every OS: ConPTY on Windows, posix_openpt on Unix, with child process spawn, resize, and clean teardown.
//!
//! Scaffold release (v0.1.0). The public surface is designed across the 0.x
//! series and frozen at v1.0. See `docs/API.md` and `dev/ROADMAP.md` for the
//! current phase scope.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![deny(unused_results)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]
#![deny(clippy::dbg_macro)]

#[cfg(test)]
mod tests {
    #[test]
    fn scaffold_compiles() {
        assert_eq!(env!("CARGO_PKG_NAME"), "pty-io");
    }
}
