pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const WHITE: Self = Self {
        red: 0xff,
        green: 0xff,
        blue: 0xff,
    };

    pub const RED: Self = Self {
        red: 0xff,
        green: 0x00,
        blue: 0x00,
    };

    pub const GREEN: Self = Self {
        red: 0x00,
        green: 0xff,
        blue: 0x00,
    };

    pub const BLUE: Self = Self {
        red: 0x00,
        green: 0x00,
        blue: 0xff,
    };

    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        let color = Self {
            red: red,
            green: green,
            blue: blue,
        };

        return color;
    }

    pub fn closest_mix(self, palette: &[(Self, u8)], max: u32) -> Self {
        let color = Self::new(0, 0, 0);

        return color;
    }
}
