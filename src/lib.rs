/// The PI constant used by the library.
pub const PI: f64 = 3.141592653589;

/// The Unit trait for creating all unit types from.
pub trait Unit {
    /// The conversion factor to the SI unit.
    fn conversion_factor() -> f64;
    /// The type of the unit eg, Length, Angle.
    fn get_type() -> String;
    /// Returns the value of the unit as the SI value.
    fn get_as_si(&self) -> f64;
    /// Converts the unit to the specified Unit. Panics if the types are not the same.
    fn convert<T: Unit>(&self) -> T {
        assert_eq!(Self::get_type(), T::get_type());
        T::new(self.get_as_si() * T::conversion_factor())
    }
    /// Creates a new instance of the Unit.
    fn new(value: f64) -> Self;
}

#[macro_export]
/// Creates the specified Unit type. Takes in the name, conversion factor (a f64) and the type (a
/// String)
macro_rules! create_unit_type {
    ($name:ident, $conversion_factor:expr, $type:expr) => {
        pub struct $name {
            value: f64,
        }

        impl Unit for $name {
            fn conversion_factor() -> f64 {
                $conversion_factor
            }

            fn get_type() -> String {
                $type
            }

            fn get_as_si(&self) -> f64 {
                self.value * $conversion_factor
            }

            fn new(value: f64) -> Self {
                Self {
                    value
                }
            }
        }
    };
}

pub mod angle;
pub mod length;
