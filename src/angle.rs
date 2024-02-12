use crate::Unit;
use core::f64::consts::PI;

create_unit_type!(Radians, 1.0, String::from("Angle"));
create_unit_type!(Degrees, 180.0 / PI, String::from("Angle"));
create_unit_type!(Rotations, 1.0 / (PI * 2.0), String::from("Angle"));

#[cfg(test)]
mod tests {
    mod radians {
        use crate::angle::{Unit, Radians, Rotations, Degrees};
        use core::f64::consts::PI;

        #[test]
        fn as_si_test() {
            let rads = Radians::new(16.5);
            assert_eq!(rads.value, rads.get_as_si());
        }

        #[test]
        fn rotation_conversion_test() {
            let rads = Radians::new(PI);
            assert_eq!(0.5, rads.convert::<Rotations>().value);
        }

        #[test]
        fn degrees_conversion_test() {
            let rads = Radians::new(PI);
            assert_eq!(180.0, rads.convert::<Degrees>().value);
        }
    }
}
