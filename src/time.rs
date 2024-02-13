use crate::Unit;

pub enum Time {}
create_unit_type!(Second, 1.0, Time);
create_unit_type!(Millisecond, 100.0, Time);
create_unit_type!(Minute, 1.0 / 60.0, Time);
create_unit_type!(Hour, 1.0 / 60.0 / 60.0, Time);

#[cfg(test)]
mod tests {
    mod second {
        use crate::time::*;

        #[test]
        fn as_si() {
            let time = Second::new(1.0);
            assert_eq!(time.value, time.get_as_si());
        }

        #[test]
        fn convert_minutes() {
            let time = Second::new(1.0);
            assert_eq!(1.0 / 60.0, time.convert::<Minute>().value);
        }

        #[test]
        fn convert_hours() {
            let time = Second::new(1.0);
            assert_eq!(1.0 / 3600.0, time.convert::<Hour>().value)
        }

        #[test]
        fn convert_milliseconds() {
            let time = Second::new(1.0);
            assert_eq!(100.0, time.convert::<Millisecond>().value)
        }
    }

    mod milliseconds {
        use crate::time::*;

        #[test]
        fn as_si() {
            let time = Millisecond::new(1.0);
            assert_eq!(0.01, time.get_as_si());
        }

        #[test]
        fn convert_minutes() {
            let time = Millisecond::new(1.0);
            assert_eq!(1.0 / 60.0 / 100.0, time.convert::<Minute>().value);
        }

        #[test]
        fn convert_hours() {
            let time = Millisecond::new(1.0);
            assert_eq!(1.0 / 3600.0 / 100.0, time.convert::<Hour>().value)
        }
    }

    mod minutes {
        use crate::time::*;

        #[test]
        fn as_si() {
            let time = Minute::new(1.0);
            assert_eq!(60.0, time.get_as_si());
        }

        #[test]
        fn convert_hours() {
            let time = Minute::new(1.0);
            assert_eq!(1.0 / 60.0, time.convert::<Hour>().value)
        }

        #[test]
        fn convert_millisecond() {
            let time = Minute::new(1.0);
            assert_eq!(6000.0, time.convert::<Millisecond>().value)
        }
    }

    mod hours {
        use crate::time::*;

        #[test]
        fn as_si() {
            let time = Hour::new(1.0);
            assert_eq!(3600.0, time.get_as_si());
        }

        #[test]
        fn convert_minutes() {
            let time = Hour::new(1.0);
            assert_eq!(60.0, time.convert::<Minute>().value)
        }

        #[test]
        fn convert_millisecond() {
            let time = Hour::new(1.0);
            assert_eq!(360000.0, time.convert::<Millisecond>().value)
        }
    }
}
