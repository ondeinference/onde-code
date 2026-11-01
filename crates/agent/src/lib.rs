//! Onde Code as an embeddable [`ed_acp::Profile`]: its prompt and tools ([`profile::OndeCode`]),
//! its Onde sign-in ([`auth::OndeAuth`]) and the browser flow behind it ([`browser_auth`]).
//! The `onde-code` binary serves it over stdio; an app can host it in process instead.

pub mod auth;
pub mod browser_auth;
pub mod profile;

pub use profile::{INFO, OndeCode};
