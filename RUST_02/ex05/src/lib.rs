#[derive(PartialEq, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const WHITE: Self = Self {
        red: 255,
        green: 255,
        blue: 255,
    };

    pub const RED: Self = Self {
        red: 255,
        green: 0,
        blue: 0,
    };

    pub const GREEN: Self = Self {
        red: 0,
        green: 255,
        blue: 0,
    };

    pub const BLUE: Self = Self {
        red: 0,
        green: 0,
        blue: 255,
    };

    pub fn eq(&self, other: &Self) -> bool {
        let dr = self.red - other.red;
        let dg = self.green - other.green;
        let db = self.blue - other.blue;
        let distance = dr * dr + dg * dg + db * db;

        return distance == 0;
    }

    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        let color = Self { red, green, blue };

        return color;
    }

    // pub fn closest_mix(self, palette: &[(Self, u8)], max: u32) -> Self {
    //     let result = Self::new(255, 255, 255);
    //
    //     for color in palette {}
    //
    //     return result;
    // }
}

// #[test]
// fn test_color_mix() {
//     assert_eq!(Color::RED.closest_mix(&[], 100), Color::WHITE);
//     assert_eq!(
//         Color::RED.closest_mix(&[(Color::RED, 255)], 0),
//         Color::WHITE
//     );
//
//     let palette = [(Color::RED, 100), (Color::GREEN, 100), (Color::BLUE, 100)];
//     assert_eq!(
//         Color::new(254, 23, 102).closest_mix(&palette, 5),
//         Color::new(218, 20, 57),
//     );
// }
