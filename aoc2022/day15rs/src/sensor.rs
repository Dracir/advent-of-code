#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sensor {
    pub s_x: i64,
    pub s_y: i64,
    pub b_x: i64,
    pub b_y: i64,
    pub radius: i64,
}

impl Sensor {
    pub fn new(s_x: i64, s_y: i64, b_x: i64, b_y: i64) -> Sensor {
        Sensor {
            s_x,
            s_y,
            b_x,
            b_y,
            radius: manhattan_distance(s_x, s_y, b_x, b_y),
        }
    }

    pub fn distance_covered(&self) -> i64 {
        manhattan_distance(self.s_x, self.s_y, self.b_x, self.b_y)
    }
}
#[inline(always)]
fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn parse_sensor(input: &str) -> Sensor {
    let simpler = input.to_owned().replace("x=", "").replace("y=", "").replace(":", "");
    let mut parts = simpler.split_whitespace();
    let s_x = parts.nth(2).unwrap().trim_end_matches(',').parse().unwrap();
    let s_y = parts.nth(0).unwrap().parse().unwrap();
    let b_x = parts.nth(4).unwrap().trim_end_matches(',').parse().unwrap();
    let b_y = parts.nth(0).unwrap().parse().unwrap();
    Sensor::new(s_x, s_y, b_x, b_y)
}
