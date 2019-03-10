//! `uom`, Units of measurement, is a crate that does automatic type-safe zero-cost [dimensional
//! analysis][analysis]. You can create your own systems or use the pre-built [International System
//! of Units][si] (SI) which is based on the [International System of Quantities][isq] (ISQ) and
//! includes numerous [quantities][quantity] (length, mass, time, ...) with conversion factors for
//! even more numerous [measurement units][measurement] (meter, kilometer, foot, mile, ...). No
//! more crashing your [climate orbiter][orbiter]!
//!
//! [analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [isq]: http://jcgm.bipm.org/vim/en/1.6.html
//! [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
//! [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
//! [orbiter]: https://en.wikipedia.org/wiki/Mars_Climate_Orbiter
//!
//! ## Usage
//! `uom` requires `rustc` 1.31.0 or later. Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! uom = "0.21.1"
//! ```
//!
//! The simple example below shows how to use quantities and units as well as how `uom` stops
//! invalid operations:
//!
//! ```rust
//! //use uom::si::f32::*;
//! //use uom::si::length::kilometer;
//! //use uom::si::time::second;
//!
//! //fn main() {
//! //    let length = Length::new::<kilometer>(5.0);
//! //    let time = Time::new::<second>(15.0);
//! //    let velocity/*: Velocity*/ = length / time;
//! //    let _acceleration = calc_acceleration(velocity, time);
//! //    //let error = length + time; // error[E0308]: mismatched types
//! //}
//!
//! //fn calc_acceleration(velocity: Velocity, time: Time) -> Acceleration {
//! //    velocity / time
//! //}
//! ```
//!
//! See the [examples](examples) directory for more advanced usage:
//!
//! ## Features
//! `uom` has multiple `Cargo` features for controlling available underlying storage types, the
//! inclusion of the pre-built [International System of Units][si] (SI), support for [Serde][serde],
//! and `no_std` functionality. The features are described below. `f32`, `f64`, `std`, and `si` are
//! enabled by default. Features can be cherry-picked by using the `--no-default-features` and
//! `--features "..."` flags when compiling `uom` or specifying features in Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! uom = {
//!     version = "0.21.1",
//!     default-features = false,
//!     features = [
//!         "autoconvert", # automatic base unit conversion.
//!         "usize", "u8", "u16", "u32", "u64", "u128", # Unsigned integer storage types.
//!         "isize", "i8", "i16", "i32", "i64", "i128", # Signed interger storage types.
//!         "bigint", "biguint", # Arbitrary width integer storage types.
//!         "rational", "rational32", "rational64", "bigrational", # Integer ratio storage types.
//!         "f32", "f64", # Floating point storage types.
//!         "si", "std", # Built-in SI system and std library support.
//!         "use_serde", # Serde support.
//!     ]
//! }
//! ```
//!
//!  * `usize`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `i8`, `i16`, `i32`, `i64`, `i128`,
//!    `bigint`, `biguint`, `rational`, `rational32`, `rational64`, `bigrational`, `f32`, `f64`
//!    -- Features to enable underlying storage types. At least one of these features must be
//!    enabled. `f32` and `f64` are enabled by default. See the [Design](#design) section for
//!    implications of choosing different underlying storage types.
//!  * `si` -- Feature to include the pre-built [International System of Units][si] (SI). Enabled by
//!    default.
//!  * `std` -- Feature to compile with standard library support. Disabling this feature compiles
//!    `uom` with `no_std`. Enabled by default.
//!  * `use_serde` -- Feature to enable support for serialization and deserialization of quantities
//!    with the [Serde][serde] crate. Disabled by default.
//!
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [serde]: https://serde.rs/
//!
//! ## Design
//!
//! ## Contributing
//! Contributions are welcome from everyone. Submit a pull request, an issue, or just add comments
//! to an existing item. The [International Bureau of Weights and Measures][BIPM] is an
//! international standards organization that publishes the [SI Brochure][brochure]. This document
//! defines the [SI] and can be used as a comprehensive reference for changes to `uom`. Conversion
//! factors for non-SI units can be found in NIST [Special Publication 811][nist811].
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
//! in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as below,
//! without any additional terms or conditions.
//!
//! ### License
//! Licensed under either of
//!
//!  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
//!    <http://www.apache.org/licenses/LICENSE-2.0>)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! [BIPM]: http://www.bipm.org/en/about-us/
//! [brochure]: http://www.bipm.org/en/publications/si-brochure/
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [nist811]: https://www.nist.gov/pml/nist-guide-si-appendix-b9-factors-units-listed-kind-quantity-or-field-science

// Compile with `no_std` when the `std` feature is not specified.
#![cfg_attr(not(feature = "std"), no_std)]
// Rustc lints.
#![forbid(unsafe_code)]
#![warn(bare_trait_objects)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

// Fail to compile if no underlying storage type features are specified.
#[rustfmt::skip]
#[cfg(not(any(
    feature = "usize", feature = "u8", feature = "u16", feature = "u32", feature = "u64",
    feature = "u128",
    feature = "isize", feature = "i8", feature = "i16", feature = "i32", feature = "i64",
    feature = "i128",
    feature = "bigint", feature = "biguint",
    feature = "rational", feature = "rational32", feature = "rational64", feature = "bigrational",
    feature = "f32", feature = "f64")))]
#[rustfmt::skip]
compile_error!("A least one underlying storage type must be enabled. See the features section of \
    uom documentation for available underlying storage type options.");

use num_traits as num;
use std::marker::PhantomData;

/// Primitive traits and types representing basic properties of types.
pub mod marker;

// #[cfg(feature = "si")]
// pub mod si;

/// TODO: Document and implement Debug.
#[derive(Debug)]
pub struct Quantity<U, V> {
    unit: PhantomData<U>,
    value: V,
}

/// TODO: Document
pub trait Unit {
    /// TODO
    type Dimension;
    /// TODO
    type Kind;

    /// Unit abbreviation.
    fn abbreviation() -> &'static str;

    /// Unit singular description.
    fn singular() -> &'static str;

    /// Unit plural description.
    fn plural() -> &'static str;
}

/// Trait to identify [units][units] which have a [conversion factor][factor].
///
/// [units]: http://jcgm.bipm.org/vim/en/1.13.html
/// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
pub trait Conversion<V> {
    /// Conversion factor type specific to the underlying storage type.
    type T: ConversionFactor<V>;

    /// Coefficient portion of [conversion factor][factor] for converting the given unit to the
    /// base unit for the quantity: `(value * coefficient()) + constant()`.
    ///
    /// Default implementation returns `Self::T::one()`.
    ///
    /// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
    #[inline(always)]
    fn coefficient() -> Self::T {
        <Self::T as num::One>::one()
    }

    /// Constant portion of [conversion factor][factor] for converting the given unit to the base
    /// unit for the quantity: `(value * coefficient()) + constant()`.
    ///
    /// Default implementation returns `Self::T::zero()`.
    ///
    /// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
    #[inline(always)]
    fn constant() -> Self::T {
        <Self::T as num::Zero>::zero()
    }

    /// Instance [conversion factor][factor].
    ///
    /// Default implementation returns the coefficient: `Self::coefficient()`.
    ///
    /// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
    #[inline(always)]
    fn into_conversion(&self) -> Self::T {
        Self::coefficient()
    }
}

/// Trait representing a [conversion factor][factor].
///
/// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
pub trait ConversionFactor<V>:
    std::ops::Add<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + num::Zero
    + num::One
{
    /// Raises a `ConversionFactor<V>` to an integer power.
    fn powi(self, e: i32) -> Self;

    /// Converts a `ConversionFactor<V>` into its underlying storage type.
    fn value(self) -> V;
}

/*
pub trait Dimension: std::fmt::Debug {
    type L: Integer;
    type M: Integer;
}

pub trait Unit {
    type D: Dimension + ?Sized;
    type K: ?Sized;
}

pub trait Conversion<V>: Unit {
    fn conversion() -> V;
}

#[derive(Debug)]
pub struct Quantity<U, V>
where
    //U: Conversion<V> + ?Sized,
    U: Unit + ?Sized
{
    unit: PhantomData<U>,
    value: V,
}

impl<U, V> Quantity<U, V>
where
    //U: Conversion<V> + ?Sized,
    U: Unit + ?Sized
{
    pub fn new(value: V) -> Self {
        Self { unit: PhantomData, value, }
    }
}

impl<Ul, Ur, V> Div<Quantity<Ur, V>> for Quantity<Ul, V>
where
    //Ul: Conversion<V> + ?Sized,
    Ul: Unit + ?Sized,
    //Ur: Conversion<V> + ?Sized,
    Ur: Unit + ?Sized,
    //Ul::Dimension: Dimension + ?Sized,
    //Ur::Dimension: Dimension + ?Sized,
    //Ul::D::L: Sub<Ur::D::L>,
    //Ul::Dimension::M: Sub<Ur::Dimension::M>,
    //<<Ul as Unit>::D as Dimension>::L: Sub<<<Ur as Unit>::Dimension as Dimension>::L>,
    //<<Ul as Unit>::Dimension as Dimension>::L: Sub<<<Ur as Unit>::Dimension as Dimension>::L>,
    V: One,
{
    type Output = Quantity<
        BaseUnit<
            Dimension<
                L = <Ul::D::L as Sub<Ul::D::L>>::Output,
                //L = <<<Ur as Unit>::D as Dimension>::L as Sub<<<Ur as Unit>::D as Dimension>::L>>::Output,
                M = <<<Ur as Unit>::D as Dimension>::M as Sub<<<Ur as Unit>::D as Dimension>::M>>::Output>,
            Kind>,
        V>;

    fn div(self, rhs: Quantity<Ur, V>) -> Self::Output {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct BaseUnit<D, K>
where
    D: Dimension + ?Sized,
    K: ?Sized,
{
    dimension: PhantomData<D>,
    kind: PhantomData<K>,
}

impl<D, K> Unit for BaseUnit<D, K>
where
    D: Dimension + ?Sized,
    K: ?Sized,
{
    type D = D;
    type K = K;
}

impl<D, K, V> Conversion<V> for BaseUnit<D, K>
where
    D: Dimension + ?Sized,
    K: ?Sized,
    V: One,
{
    fn conversion() -> V { V::one() }
}

pub trait Kind: std::fmt::Debug {}

// mod length {
//     pub type Dimension = crate::Dimension<
//         L = crate::P1,
//         M = crate::Z0>;
//     pub type Kind = crate::Kind;

//     mod units {
//         #[derive(Debug)]
//         pub struct Meter;

//         impl crate::Unit for Meter {
//             type Dimension = super::Dimension;
//             type Kind = super::Kind;
//         }

//         impl<V> crate::Conversion<V> for Meter
//         where
//             V: crate::One,
//         {
//             fn conversion() -> V { V::one() }
//         }
//     }

//     pub type Length<V> = crate::Quantity<crate::BaseUnit<Dimension, Kind>, V>;
//     pub type Meter<V> = crate::Quantity<units::Meter, V>;
// }

fn main() {
    // let m = length::Meter::new(1.0);
    // let l = length::Length::new(2.0);
    // let k: Quantity<BaseUnit<Dimension<L = Z0, M = P1>, Kind>, f32> =
    //     Quantity { unit: PhantomData, value: 3.0, };

    // println!("{:?}", m);
    // println!("{:?}", l);
    // println!("{:?}", k);
    // println!("{:?}", k / m);
}

*/
