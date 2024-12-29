pub struct Point {
    x: f32,
    y: f32,
}

fn my_abs(nbr: f32) -> f32 {
    if nbr < 0.00 {
        return -nbr;
    }

    return nbr;
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        let point = Point { x: x, y: y };

        return point;
    }

    pub fn zero() -> Self {
        let point = Point { x: 0.00, y: 0.00 };

        return point;
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let dx = my_abs(self.x - other.x);
        let dy = my_abs(self.y - other.y);

        return dx.sqrt() + dy.sqrt();
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}
