//! If we start Cargo with bootstrap feature we will load parser code checked
//! out from the git `main` branch.
//!
//! In regular builds parser code from the source tree will be used.
#[rustfmt::skip]
#[cfg(not(feature="bootstrap"))]
pub(crate) mod rustemo;

// Relax these checks as we have generated code from the grammar.
#[allow(non_camel_case_types, clippy::enum_variant_names)]
#[cfg(not(feature = "bootstrap"))]
pub(crate) mod rustemo_actions;

#[cfg(feature = "bootstrap")]
rustemo_mod! {pub(crate) rustemo, "/src/lang"}

#[cfg(feature = "bootstrap")]
rustemo_mod! {pub(crate) rustemo_actions, "/src/lang"}

#[cfg(test)]
mod tests;