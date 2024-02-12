use crate::Unit;

create_unit_type!(Kilometres, 0.001, String::from("Length"));
create_unit_type!(Metres, 1.0, String::from("Length"));
create_unit_type!(Decimetres, 10.0, String::from("Length"));
create_unit_type!(Centimetres, 100.0, String::from("Length"));
create_unit_type!(Millimetres, 1000.0, String::from("Length"));

#[cfg(test)]
mod tests {
    mod metres {
        use crate::length::*;

        #[test]
        fn as_si_test() {
            let length = Metres::new(1.0);
            assert_eq!(1.0, length.get_as_si());
        }

        #[test]
        fn convert_to_kilometres() {
            let length = Metres::new(1.0);
            assert_eq!(0.001, length.convert::<Kilometres>().value);
        }

        #[test]
        fn convert_to_decimetres() {
            let length = Metres::new(1.0);
            assert_eq!(10.0, length.convert::<Decimetres>().value);
        }

        #[test]
        fn convert_to_centimetres() {
            let length = Metres::new(1.0);
            assert_eq!(100.0, length.convert::<Centimetres>().value);
        }

        #[test]
        fn convert_to_millimetres() {
            let length = Metres::new(1.0);
            assert_eq!(1000.0, length.convert::<Millimetres>().value);
        }
    }

    mod kilometres {
        use crate::length::*;

        #[test]
        fn as_si_test() {
            let length = Kilometres::new(1.0);
            assert_eq!(1000.0, length.get_as_si());
        }

        #[test]
        fn convert_to_metres() {
            let length = Metres::new(1.0);
            assert_eq!(10.0, length.convert::<Decimetres>().value);
        }

        #[test]
        fn convert_to_decimetres() {
            let length = Kilometres::new(1.0);
            assert_eq!(10000.0, length.convert::<Decimetres>().value);
        }

        #[test]
        fn convert_to_centimetres() {
            let length = Kilometres::new(1.0);
            assert_eq!(100000.0, length.convert::<Centimetres>().value);
        }

        #[test]
        fn convert_to_millimetres() {
            let length = Kilometres::new(1.0);
            assert_eq!(1000000.0, length.convert::<Millimetres>().value);
        }
    }

    mod decimetres {
        use crate::length::*;

        #[test]
        fn as_si_test() {
            let length = Decimetres::new(1.0);
            assert_eq!(0.1, length.get_as_si());
        }

        #[test]
        fn convert_to_kilometres() {
            let length = Decimetres::new(1.0);
            assert_eq!(0.0001, length.convert::<Kilometres>().value);
        }

        #[test]
        fn convert_to_centimetres() {
            let length = Decimetres::new(1.0);
            assert_eq!(10.0, length.convert::<Centimetres>().value);
        }

        #[test]
        fn convert_to_millimetres() {
            let length = Decimetres::new(1.0);
            assert_eq!(100.0, length.convert::<Millimetres>().value);
        }
    }

    mod centimetres {
        use crate::length::*;

        #[test]
        fn as_si_test() {
            let length = Centimetres::new(1.0);
            assert_eq!(0.01, length.get_as_si());
        }

        #[test]
        fn convert_to_kilometres() {
            let length = Centimetres::new(1.0);
            assert_eq!(0.00001, length.convert::<Kilometres>().value);
        }

        #[test]
        fn convert_to_decimetres() {
            let length = Centimetres::new(1.0);
            assert_eq!(0.1, length.convert::<Decimetres>().value);
        }

        #[test]
        fn convert_to_millimetres() {
            let length = Centimetres::new(1.0);
            assert_eq!(10.0, length.convert::<Millimetres>().value);
        }
    }

    mod millimetres {
        use crate::length::*;

        #[test]
        fn as_si_test() {
            let length = Millimetres::new(1.0);
            assert_eq!(0.001, length.get_as_si());
        }

        #[test]
        fn convert_to_kilometres() {
            let length = Millimetres::new(1.0);
            assert_eq!(0.000001, length.convert::<Kilometres>().value);
        }

        #[test]
        fn convert_to_decimetres() {
            let length = Millimetres::new(1.0);
            assert_eq!(0.01, length.convert::<Decimetres>().value);
        }

        #[test]
        fn convert_to_centimetres() {
            let length = Millimetres::new(1.0);
            assert_eq!(0.1, length.convert::<Centimetres>().value);
        }
    }
}
