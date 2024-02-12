use crate::{Unit, PI};

create_unit_type!(Radians, 0.0, String::from("Angle"));
create_unit_type!(Degrees, PI / 180.0, String::from("Angle"));
create_unit_type!(Rotations, PI * 2.0, String::from("Angle"));
