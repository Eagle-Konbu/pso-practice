#[derive(Clone)]
pub struct Particle {
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub personal_best: Vec<f64>,
}