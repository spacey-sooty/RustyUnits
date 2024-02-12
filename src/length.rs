use crate::Unit;

create_unit_type!(Kilometres, 1000.0, String::from("Length"));
create_unit_type!(Metres, 1.0, String::from("Length"));
create_unit_type!(Decimetres, 0.1, String::from("Length"));
create_unit_type!(Centimetres, 0.01, String::from("Length"));
create_unit_type!(Millimetres, 0.001, String::from("Length"));

#[cfg(test)]
mod tests {}
