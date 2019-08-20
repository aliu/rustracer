pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn to_rgb(self) -> [u8; 3] {
        fn inner(x: f64) -> u8 {
            (x * 256.0) as u8
        }
        [inner(self.x), inner(self.y), inner(self.z)]
    }
}

impl Into<[f64; 3]> for Vec3 {
    fn into(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}
