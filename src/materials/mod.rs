mod lambertian;
mod material;
mod metal;

pub use self::lambertian::Lambertian;
pub use self::material::{Material, ScatterData};
pub use self::metal::Metal;
