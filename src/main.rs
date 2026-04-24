mod benchmarks;
mod sequential;
mod parallel;
mod algorithm;



fn main(){
    let seq_time = benchmarks::time_sequential(|| {
        sequential::classify_sequential(&templates, &test_images);
    });
    let par_time = benchmarks::time_parallel(|| {
        parallel::classify_parallel(&templates, &test_images);

    });
    
}