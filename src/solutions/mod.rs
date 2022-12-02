use crate::{Answer};

mod y_2022;

pub fn get_year(year: u32) -> &'static [&'static dyn Answer] {
    match year {
        2022 => &y_2022::ALL,
        _ => &[],
    }
}
