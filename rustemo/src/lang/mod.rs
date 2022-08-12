//! If we start Cargo with bootstrap feature we will load parser code checked
//! out from the git `main` branch.
//!
//! In regular builds parser code from the source tree will be used.
#[rustfmt::skip]
#[cfg(not(feature="bootstrap"))]
pub(crate) mod rustemo;

#[allow(non_camel_case_types)]
#[cfg(not(feature="bootstrap"))]
pub(crate) mod rustemo_actions;


#[cfg(feature="bootstrap")]
rustemo_mod!{rustemo}

#[cfg(feature="bootstrap")]
rustemo_mod!{rustemo_actions}

#[cfg(test)]
mod tests;
