use pso::pso::Pso;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    let mut pso = Pso::new(0.5, 0.5, 0.5, 0.0, 10.0, 100, 5, f, &mut rng);
    pso.run(&mut rng, 100);

    println!(
        "global best: {:?}\nbest score: {:?}",
        pso.global_best,
        f(&pso.global_best)
    );
}

fn f(x: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.len() {
        sum += x[i].powi(2);
    }
    sum
}
