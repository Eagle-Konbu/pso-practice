use crate::particle::Particle;
use rand::Rng;

type Graph = Vec<Vec<usize>>;

pub struct Mopso {
    pub particles: Vec<Particle>,
    pub w: f64,
    pub c1: f64,
    pub c2: f64,
    pub eval_funcs: Vec<fn(&[f64]) -> f64>,
    pub external_archive: Vec<Particle>,
}

impl Mopso {
    #[allow(clippy::too_many_arguments, clippy::same_item_push)]
    fn new(
        w: f64,
        c1: f64,
        c2: f64,
        min: f64,
        max: f64,
        n: usize,
        dim: usize,
        eval_funcs: Vec<fn(&[f64]) -> f64>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Mopso {
        let particles = {
            let mut x = vec![];
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
                x.push(Particle {
                    position,
                    velocity,
                    personal_best,
                });
            }
            x
        };

        let mut mopso = Mopso {
            particles,
            w,
            c1,
            c2,
            eval_funcs,
            external_archive: vec![],
        };

        mopso.external_archive = mopso.nondominated_particles();
        mopso
    }

    fn update(&mut self, rng: &mut rand::rngs::ThreadRng) {
        for i in 0..self.particles.len() {}
    }

    fn nondominated_particles(&self) -> Vec<Particle> {
        let results = self
            .particles
            .iter()
            .map(|p| {
                self.eval_funcs
                    .iter()
                    .map(|&f| f(&p.position))
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>();

        (0..results.len())
            .filter(|&i| !(0..results.len()).any(|j| i != j && dominates(&results[j], &results[i])))
            .map(|i| self.particles[i].clone())
            .collect::<Vec<Particle>>()
    }

    fn leader_idx(&self, particle_idx: usize, k: usize) -> usize {
        let other_particles = self
            .particles
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != particle_idx)
            .collect::<Vec<(usize, &Particle)>>();
        0
    }

    fn quality(&self) -> f64 {
        0.0
    }
}

fn dominates<T: std::cmp::PartialOrd>(a: &[T], b: &[T]) -> bool {
    if a == b {
        return false;
    }
    for i in 0..a.len() {
        if a[i] > b[i] {
            return false;
        }
    }
    true
}
