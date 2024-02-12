/// The Unit trait for creating all unit types from.
pub trait Unit<U> {
    /// The conversion factor to the SI unit.
    fn conversion_factor() -> f64;
    /// Returns the value of the unit as the SI value.
    fn get_as_si(&self) -> f64;
    /// Converts the unit to the specified Unit. Panics if the types are not the same.
    fn convert<T: Unit<U>>(&self) -> T {
        T::new(self.get_as_si() * T::conversion_factor())
    }
    /// Creates a new instance of the Unit.
    fn new(value: f64) -> Self;
}

#[macro_export]
/// Creates the specified Unit type. Takes in the name, conversion factor (a f64) and the type (a
/// String)
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
