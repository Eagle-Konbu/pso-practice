use pso::pso::Pso;

fn main() {
    let n = 100;
    let complete_graph = {
        let mut graph = Vec::new();
        for i in 0..n {
            let mut row = Vec::new();
            for j in 0..n {
                if i != j {
                    row.push(j);
                }
            }
            graph.push(row);
        }
        graph
    };
    let mut rng = rand::thread_rng();
    let mut pso = Pso::new(
        0.5,
        0.5,
        0.5,
        0.0,
        10.0,
        5,
        f,
        &complete_graph,
        1,
        &mut rng,
    );
    pso.run(&mut rng, 100, false);

    println!(
        "global best: {:?}\nbest score: {:?}",
        pso.global_best,
        f(&pso.global_best)
    );
}

fn f(x: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.len() {
        sum += x[i].powi(2);
    }
    sum
}
