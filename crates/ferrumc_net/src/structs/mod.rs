pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Position {
    pub fn from_bytes(bytes: &[u8; 8]) -> Self {
        let val = i64::from_be_bytes(*bytes);

        let x = (val >> 38) as i32;
        let y = (val << 52 >> 52) as i16;
        let z = (val << 26 >> 38) as i32;

        // Handle sign extension
        let x = if x >= 1 << 25 { x - (1 << 26) } else { x };
        let y = if y >= 1 << 11 { y - (1 << 12) } else { y };
        let z = if z >= 1 << 25 { z - (1 << 26) } else { z };

        Position { x, y, z }
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let val = ((self.x as i64 & 0x3FFFFFF) << 38)
            | ((self.z as i64 & 0x3FFFFFF) << 12)
            | (self.y as i64 & 0xFFF);

        val.to_be_bytes()
    }
}

pub struct Angle {
    pub rotation: f32,
}

impl Angle {
    pub fn from_degrees(degrees: f32) -> Self {
        Angle {
            rotation: degrees / 360.0 * 256.0,
        }
    }
}

