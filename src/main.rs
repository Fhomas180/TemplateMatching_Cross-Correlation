mod benchmarks;
mod sequential;
mod parallel;
mod algorithm;
use mnist::*;
use ndarray::Array3;
use std::time::Instant;

fn main(){
    let Mnist {
        trn_img,
        trn_lbl,
        tst_img,
        tst_lbl,
        ..
    } = MnistBuilder::new()
    .base_path("data/")
        .label_format_digit()
        .training_set_length(50_000)
        .validation_set_length(10_000)
        .test_set_length(10_000)
        .finalize();


    let train_data = Array3::from_shape_vec((50_000, 28, 28), trn_img)
        .expect("Error converting training images")
        .map(|x| *x as f32 / 256.0);

    let test_data = Array3::from_shape_vec((10_000, 28, 28), tst_img)
        .expect("Error converting test images")
        .map(|x| *x as f32 / 256.0);

    
    let templates = sequential::build_templates(&train_data, &trn_lbl);

    
    let seq_predictions = sequential::classify_sequential(&templates, &test_data);
    let par_predictions = parallel::classify_parallel(&templates, &test_data);
    println!("Building templates...");
    let templates = sequential::build_templates(&train_data, &trn_lbl);
    println!("Templates built!");


    println!("Running sequential classification...");
    let seq_start = Instant::now();
    let seq_predictions = sequential::classify_sequential(&templates, &test_data);
    let seq_time = seq_start.elapsed().as_secs_f64();
    println!("Sequential done in {:.3}s", seq_time);

    println!("Running parallel classification...");
    let par_start = Instant::now();
    let par_predictions = parallel::classify_parallel(&templates, &test_data);
    let par_time = par_start.elapsed().as_secs_f64();
    println!("Parallel done in {:.3}s", par_time);
println!("\n Running the thread scaling test...");
let thread_counts = [1,2,4,8,16,24];
let mut thread_times: Vec<(usize, f64)> = Vec::new();
thread_times.push((1,seq_time));
for &num_threads in &thread_counts {
     let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .unwrap();

        let mut total = 0.0;
        for _ in 0..3 {
            let start = Instant::now();
            pool.install(|| {
                let _ = parallel::classify_parallel(&templates, &test_data);
            });
            total += start.elapsed().as_secs_f64();
        }
        let avg = total / 3.0;

        println!("Threads: {:2} | Time: {:.3}s | Speedup: {:.2}x",
            num_threads, avg, seq_time / avg);
        thread_times.push((num_threads, avg));
}
    
    let seq_correct = seq_predictions.iter()
        .zip(tst_lbl.iter())
        .filter(|(pred, label)| **pred == **label as usize)
        .count();
    let accuracy = seq_correct as f64 / seq_predictions.len() as f64 * 100.0;

    let speedup = seq_time / par_time;
    let efficiency = speedup / rayon::current_num_threads() as f64;


    benchmarks::print_results(&benchmarks::BenchmarkResults {
        sequential_time: seq_time,
        parallel_time: par_time,
        speedup,
        efficiency,
        accuracy,
    });


    benchmarks::plot_speedup_by_threads(&thread_times);
     let _ = par_predictions;
}