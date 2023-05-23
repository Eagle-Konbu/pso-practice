use crate::particle::Particle;
use rand::Rng;

type Graph = Vec<Vec<usize>>;

pub struct Mopso {
    pub particles: Vec<Particle>,
    pub w: f64,
    pub c1: f64,
    pub c2: f64,
    pub eval_funcs: Vec<fn(&[f64]) -> f64>,
    pub neighborhood_graph: Graph,
    pub neighborhood_depth: usize,
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
        dim: usize,
        eval_funcs: Vec<fn(&[f64]) -> f64>,
        neighborhood_graph: &Graph,
        neighborhood_depth: usize,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Mopso {
        let n = neighborhood_graph.len();
        if n == 0 {
            panic!("neighborhood_graph must not be empty");
        }
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

        let neighbor = neighborhood_graph.clone();
        let mut mopso = Mopso {
            particles,
            w,
            c1,
            c2,
            neighborhood_graph: neighbor,
            neighborhood_depth,
            eval_funcs,
            external_archive: vec![],
        };

        mopso.external_archive = mopso.nondominated_particles();
        mopso
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

        let mut nondominated_idxs = vec![];

        for i in 0..results.len() {
            let mut dominated = false;
            for j in 0..results.len() {
                if i == j {
                    continue;
                }
                if dominates(&results[j], &results[i]) {
                    dominated = true;
                    break;
                }
            }
            if !dominated {
                nondominated_idxs.push(i);
            }
        }

        let nondominated_particles = nondominated_idxs
            .iter()
            .map(|&i| self.particles[i].clone())
            .collect::<Vec<Particle>>();

        nondominated_particles
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
