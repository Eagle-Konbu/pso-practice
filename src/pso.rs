use crate::particle::Particle;
use rand::Rng;

pub struct Pso {
    pub particles: Vec<Particle>,
    pub global_best: Vec<f64>,
    pub w: f64,
    pub c1: f64,
    pub c2: f64,
    pub eval_func: fn(&[f64]) -> f64,
}

impl Pso {
    #[allow(clippy::too_many_arguments, clippy::same_item_push)]
    pub fn new(
        w: f64,
        c1: f64,
        c2: f64,
        min: f64,
        max: f64,
        n: usize,
        dim: usize,
        eval_func: fn(&[f64]) -> f64,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Pso {
        let mut particles = Vec::new();
        for _ in 0..n {
            let mut position = Vec::new();
            for _ in 0..dim {
                position.push(rng.gen_range(min..=max));
            }
            let mut velocity = Vec::new();
            for _ in 0..dim {
                velocity.push(0.0);
            }
            let personal_best = position.clone();
            particles.push(Particle {
                position,
                velocity,
                personal_best,
            });
        }

        let global_best = particles
            .iter()
            .min_by(|a, b| {
                eval_func(&a.position)
                    .partial_cmp(&eval_func(&b.position))
                    .unwrap()
            })
            .unwrap()
            .position
            .clone();
        Pso {
            particles,
            global_best,
            w,
            c1,
            c2,
            eval_func,
        }
    }

    fn update(&mut self, rng: &mut rand::rngs::ThreadRng) {
        // let mut new_particles = Vec::new();
        for i in 0..self.particles.len() {
            let mut new_position = Vec::new();
            let mut new_velocity = Vec::new();
            for j in 0..self.particles[i].position.len() {
                let r1 = rng.gen_range(0.0..=1.0);
                let r2 = rng.gen_range(0.0..=1.0);
                let new_velocity_j = self.w * self.particles[i].velocity[j]
                    + self.c1
                        * r1
                        * (self.particles[i].personal_best[j] - self.particles[i].position[j])
                    + self.c2 * r2 * (self.global_best[j] - self.particles[i].position[j]);
                new_velocity.push(new_velocity_j);
                let new_position_j = self.particles[i].position[j] + new_velocity_j;
                new_position.push(new_position_j);
            }

            if (self.eval_func)(&new_position) < (self.eval_func)(&self.particles[i].personal_best)
            {
                self.particles[i].personal_best = new_position.clone();
            }
            self.particles[i].position = new_position;
            self.particles[i].velocity = new_velocity;

            if (self.eval_func)(&self.particles[i].personal_best)
                < (self.eval_func)(&self.global_best)
            {
                self.global_best = self.particles[i].personal_best.clone();
            }
        }
    }

    pub fn run(&mut self, rng: &mut rand::rngs::ThreadRng, max_iter: usize) {
        for _ in 0..max_iter {
            self.update(rng);
            println!(
                "global best: {:?}\nbest score: {:?}",
                self.global_best,
                (self.eval_func)(&self.global_best)
            );
        }
    }
}
