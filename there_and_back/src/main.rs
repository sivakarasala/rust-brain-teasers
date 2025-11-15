use std::f32::consts::PI;

pub struct Degrees(pub f32);
pub struct Radians(pub f32);

impl Degrees {
    pub fn new(angle: f32) -> Self {
        Self(angle)
    }
}

impl From<Degrees> for Radians {
    fn from(item: Degrees) -> Self {
        Self(item.0 * PI / 180.0)
    }
}

fn main() {
    let one_eighty_degrees = Degrees::new(180.0);
    let one_eighty_radians: Radians = one_eighty_degrees.into();

    println!("180 Degrees in Radians = {}", one_eighty_radians.0);
}

use std::convert::TryFrom;

struct ZeroToTen(i32);

impl TryFrom<i32> for ZeroToTen {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if !(0..=10).contains(&value) {
            Err("Value must be between 0 and 10")
        } else {
            Ok(Self(value))
        }
    }
}