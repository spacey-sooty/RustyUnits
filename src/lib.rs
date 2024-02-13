//! A units library that provides length, time and angle. Also provides a powerful Unit trait and
//! macro for creating your own unit types. The pre provided types can be found under the relevant
//! modules, under the root module the Unit trait and create_unit_type macro are exposed.
//!
//! Here is a simple example for creating custom units using angles. Notice that you declare an
//! enum for Angle, this allows the traits generic bound to control what you convert into
//! restricting you to relevant units.
//! ```
//! use units::{Unit, create_unit_type};
//! use core::f64::consts::PI;
//!
//! pub enum Angle {}
//! create_unit_type!(Radians, 1.0, Angle);
//! create_unit_type!(Degrees, 180.0 / PI, Angle);
//! create_unit_type!(Rotations, 1.0 / (PI * 2.0), Angle);
//! ```
#![no_std]

/// The Unit trait for creating all unit types from.
pub trait Unit<U> {
    /// The conversion factor to the SI unit.
    fn conversion_factor() -> f64;
    /// Returns the value of the unit as the SI value.
    fn get_as_si(&self) -> f64;
    /// Converts the unit to the specified Unit.
    fn convert<T: Unit<U>>(&self) -> T {
        T::new(self.get_as_si() * T::conversion_factor())
    }
    /// Creates a new instance of the Unit.
    fn new(value: f64) -> Self;
}

#[macro_export]
/// Creates the specified Unit type. Takes in the name, conversion factor (a f64) and the type (a
/// struct or enum which is used to unify all types of a group eg. Length)
macro_rules! create_unit_type {
    ($name:ident, $conversion_factor:expr, $type:ty) => {
        pub struct $name {
            value: f64,
        }

        impl Unit<$type> for $name {
            fn conversion_factor() -> f64 {
                $conversion_factor
            }

            fn get_as_si(&self) -> f64 {
                self.value / $conversion_factor
            }

            fn new(value: f64) -> Self {
                Self { value }
            }
        }
    };
}

pub mod angle;
pub mod length;
pub mod time;
