pub type Float = f64;

#[derive(Debug)]
pub struct Vec3 {
    pub e: [Float; 3],
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            e: [0 as Float, 0 as Float, 0 as Float],
        }
    }

    pub fn with_elements(e0: Float, e1: Float, e2: Float) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> Float {
        self.e[0]
    }

    pub fn y(&self) -> Float {
        self.e[1]
    }

    pub fn z(&self) -> Float {
        self.e[2]
    }
}
