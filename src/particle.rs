#[derive(Clone)]
pub struct Particle {
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub personal_best: Vec<f64>,
}

impl Particle {
    pub fn distance(&self, other: &Particle) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.position.len() {
            sum += (self.position[i] - other.position[i]).powi(2);
        }
        sum.sqrt()
    }
}