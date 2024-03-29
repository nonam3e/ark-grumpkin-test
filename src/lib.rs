#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the prime-order curve Grumpkin, generated by
//! Zachary J. Williamson from Aztec protocol. The main feature of this
//! curve is that it forms a cycle with bn254, i.e. its scalar field and base
//! field respectively are the base field and scalar field of bn254.
//!
//!
//! Curve information:
//! Grumpkin:
//! * Base field: q =
//!   21888242871839275222246405745257275088548364400416034343698204186575808495617
//! * Scalar field: r =
//!   21888242871839275222246405745257275088696311157297823662689037894645226208583
//! * Curve equation: y^2 = x^3 - 17
//! * Valuation(q - 1, 2) = 28
//! * Valuation(r - 1, 2) = 1

#[cfg(feature = "r1cs")]
pub mod constraints;
mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
