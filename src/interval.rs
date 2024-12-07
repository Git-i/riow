#[derive(Clone)]
pub struct Interval {
    pub upper: f64,
    pub lower: f64
}

impl Interval {
    pub const UNIVERSE: Interval = Interval{ lower: f64::NEG_INFINITY, upper: f64::INFINITY };
    pub fn contains(&self, x: f64) -> bool {
        self.upper >= x && self.lower <= x
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.upper > x && self.lower < x
    }
    pub fn right_rebind(self, new_r: f64) -> Interval {
        Interval{ upper: new_r, ..self }
    }
    pub fn left_rebind(self, new_l: f64) -> Interval {
        Interval{ lower: new_l, ..self }
    }
}