use textplots::{Chart, Plot, Shape};
use std::time::Instant;

pub struct BenchmarkResults{
    pub sequential_time: f64,
    pub parallel_time: f64,
    pub speedup: f64,
    pub efficiency: f64,
    pub accuracy: f64,
}

pub fn time_sequential<F>(f: F) -> f64
where F: Fn(){
    let start = Instant::now();
    f();
    start.elapsed().as_secs_f64()
}

pub fn time_parallel<F>(f: F) -> f64
where F: Fn(){
    let start = Instant::now();
    f();
    start.elapsed().as_secs_f64()
}

pub fn calculate_speedup(seq: f64, par: f64) -> f64{
    seq / par
}

pub fn calculate_efficiency(speedup:f64, num_threads: usize) -> f64{
    speedup / num_threads as f64
}

pub fn print_results(results: &BenchmarkResults){
    println!(" BenchMark Results");
    println!("Sequential:  {:.3}s", results.sequential_time);
    println!("Parallel:    {:.3}s", results.parallel_time);
    println!("Speedup:     {:.2}x", results.speedup);
    println!("Efficiency:  {:.2}%", results.efficiency * 100.0);
    println!("Accuracy:    {:.2}%", results.accuracy);
}

pub fn plot_speedup_by_threads(times: &[(usize, f64)]){
    let points: Vec<(f32, f32)> = times
        .iter()
        .map(|(t, s)| (*t as f32, *s as f32))
        .collect();

    println!("\nExecution Time by Thread Count:");
    println!("(x = threads, y = seconds)");
    Chart::new(120, 30, 0.0, 25.0)  
        .lineplot(&Shape::Lines(&points))
        .display();
}