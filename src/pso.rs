use crate::particle::Particle;
use rand::Rng;
use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>;

pub struct Pso {
    pub particles: Vec<Particle>,
    pub global_best: Vec<f64>,
    pub w: f64,
    pub c1: f64,
    pub c2: f64,
    pub eval_func: fn(&[f64]) -> f64,
    pub neighborhood_graph: Graph,
    pub neighborhood_depth: usize,
}

impl Pso {
    #[allow(clippy::too_many_arguments, clippy::same_item_push)]
    pub fn new(
        w: f64,
        c1: f64,
        c2: f64,
        min: f64,
        max: f64,
        dim: usize,
        eval_func: fn(&[f64]) -> f64,
        neighborhood_graph: &Graph,
        neighborhood_depth: usize,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Pso {
        let n = neighborhood_graph.len();
        if n == 0 {
            panic!("neighborhood_graph must not be empty");
        }
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

        let neighbor = neighborhood_graph.clone();
        Pso {
            particles,
            global_best,
            w,
            c1,
            c2,
            neighborhood_graph: neighbor,
            neighborhood_depth,
            eval_func,
        }
    }

    fn update(&mut self, rng: &mut rand::rngs::ThreadRng) {
        for i in 0..self.particles.len() {
            let mut new_position = Vec::new();
            let mut new_velocity = Vec::new();

            let leader_idx = self.local_best_idx(i);
            for j in 0..self.particles[i].position.len() {
                let r1 = rng.gen_range(0.0..=1.0);
                let r2 = rng.gen_range(0.0..=1.0);
                let new_velocity_j = self.w * self.particles[i].velocity[j]
                    + self.c1
                        * r1
                        * (self.particles[i].personal_best[j] - self.particles[i].position[j])
                    + self.c2
                        * r2
                        * (self.particles[leader_idx].position[j] - self.particles[i].position[j]);
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

    fn local_best_idx(&self, particle_idx: usize) -> usize {
        let mut particles = vec![(self.particles[particle_idx].clone(), particle_idx)];

        // bfs
        let mut deq = VecDeque::new();
        let mut seen = vec![false; self.particles.len()];
        deq.push_back((particle_idx, 0));
        while !deq.is_empty() {
            let (v, d) = deq.pop_front().unwrap();
            seen[v] = true;
            particles.push((self.particles[v].clone(), v));
            if d == self.neighborhood_depth {
                continue;
            }
            for &vv in self.neighborhood_graph[v].iter() {
                if !seen[vv] {
                    deq.push_back((vv, d + 1));
                }
            }
        }

        particles
            .iter()
            .min_by(|a, b| {
                (self.eval_func)(&a.0.position)
                    .partial_cmp(&(self.eval_func)(&b.0.position))
                    .unwrap()
            })
            .unwrap()
            .1
    }

    pub fn run(&mut self, rng: &mut rand::rngs::ThreadRng, max_iter: usize, print_status: bool) {
        for _ in 0..max_iter {
            self.update(rng);
            if print_status {
                println!(
                    "global best: {:?}\nbest score: {:?}",
                    self.global_best,
                    (self.eval_func)(&self.global_best)
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // テスト用の評価関数
    fn eval_func(position: &[f64]) -> f64 {
        position.iter().sum()
    }

    //テスト用のグラフ
    fn test_graph() -> Graph {
        vec![
            vec![1, 2],
            vec![0, 2],
            vec![0, 1, 3],
            vec![2, 4],
            vec![3, 5],
            vec![4, 6],
            vec![5, 7],
            vec![6, 8],
            vec![7, 9],
            vec![8],
        ]
    }

    #[test]
    fn test_pso_new() {
        let w = 0.5;
        let c1 = 1.0;
        let c2 = 2.0;
        let min = 0.0;
        let max = 1.0;
        let dim = 2;
        let neighborhood_graph = test_graph();
        let neighborhood_depth = 1;
        let mut rng = rand::thread_rng();

        let pso = Pso::new(
            w,
            c1,
            c2,
            min,
            max,
            dim,
            eval_func,
            &neighborhood_graph,
            neighborhood_depth,
            &mut rng,
        );

        // Pso構造体が正しく初期化されているかを確認
        assert_eq!(pso.particles.len(), neighborhood_graph.len());
        assert_eq!(pso.global_best.len(), dim);
    }

    #[test]
    fn test_pso_update() {
        let w = 0.5;
        let c1 = 1.0;
        let c2 = 2.0;
        let min = 0.0;
        let max = 1.0;
        let dim = 2;
        let neighborhood_graph = test_graph();
        let neighborhood_depth = 1;
        let mut rng = rand::thread_rng();

        let mut pso = Pso::new(
            w,
            c1,
            c2,
            min,
            max,
            dim,
            eval_func,
            &neighborhood_graph,
            neighborhood_depth,
            &mut rng,
        );

        // updateメソッドを実行し、エラーが発生しないことを確認
        pso.update(&mut rng);
    }

    #[test]
    fn test_pso_local_best_idx() {
        let w = 0.5;
        let c1 = 1.0;
        let c2 = 2.0;
        let min = 0.0;
        let max = 1.0;
        let dim = 2;
        let neighborhood_graph = test_graph();
        let neighborhood_depth = 1;
        let mut rng = rand::thread_rng();

        let pso = Pso::new(
            w,
            c1,
            c2,
            min,
            max,
            dim,
            eval_func,
            &neighborhood_graph,
            neighborhood_depth,
            &mut rng,
        );

        // local_best_idxメソッドを実行し、エラーが発生しないことを確認
        let _ = pso.local_best_idx(0);
    }

    #[test]
    fn test_pso_run() {
        let w = 0.5;
        let c1 = 1.0;
        let c2 = 2.0;
        let min = 0.0;
        let max = 1.0;
        let dim = 2;
        let neighborhood_graph = test_graph();
        let neighborhood_depth = 1;
        let mut rng = rand::thread_rng();

        let mut pso = Pso::new(
            w,
            c1,
            c2,
            min,
            max,
            dim,
            eval_func,
            &neighborhood_graph,
            neighborhood_depth,
            &mut rng,
        );

        // runメソッドを実行し、エラーが発生しないことを確認
        pso.run(&mut rng, 10, false);
    }
}
