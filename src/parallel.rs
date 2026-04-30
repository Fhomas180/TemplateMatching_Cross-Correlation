use ndarray::Array3;
use rayon::prelude::*;
use std::sync::Arc;
use std::thread;
use crate::algorithm::{normalize_image, dot_product, find_best_match};

pub fn classify_parallel(
    templates: &[Vec<f32>],
    test_images: &Array3<f32>,
) -> Vec<usize> {
    let num_images = test_images.shape()[0];

    (0..num_images)
        .into_par_iter()          // rayon takes over here — splits work across threads
        .map(|i| {
            
            let img = test_images.slice(ndarray::s![i, .., ..]);
            let flat: Vec<f32> = img.iter().cloned().collect();

            
            let normalized = normalize_image(&flat);

            
            let scores: Vec<f32> = templates
                .iter()
                .map(|t| dot_product(&normalized, t))
                .collect();

            
            find_best_match(&scores)
        })
        .collect()
}
pub fn classify_parallel_threads(
    templates: &[Vec<f32>],
    test_images: &Array3<f32>,
    num_threads: usize,
) -> Vec<usize> {
    let num_images = test_images.shape()[0];
    let chunk_size = num_images / num_threads;
    let templates_arc = Arc::new(templates.to_vec());
    let mut all_images: Vec<Vec<f32>> = Vec::with_capacity(num_images);
    for i in 0..num_images {
        let img = test_images.slice(ndarray::s![i, .., ..]);
        all_images.push(img.iter().cloned().collect());
    }
    let images_arc = Arc::new(all_images);
    let mut handles = Vec::new();
    for t in 0..num_threads {
        let templates_clone = Arc::clone(&templates_arc);
        let images_clone = Arc::clone(&images_arc);
        let start = t * chunk_size;
        let end = if t == num_threads - 1 { num_images } else { start + chunk_size };
        let handle = thread::spawn(move || {
            let mut predictions = Vec::new();
            for i in start..end {
                let normalized = normalize_image(&images_clone[i]);
                let scores: Vec<f32> = templates_clone
                    .iter()
                    .map(|t| dot_product(&normalized, t))
                    .collect();
                predictions.push(find_best_match(&scores));
            }
            predictions
        });
        handles.push(handle);
    }
    let mut results = vec![0usize; num_images];
    let mut idx = 0;
    for handle in handles {
        let chunk = handle.join().unwrap();
        for pred in chunk {
            results[idx] = pred;
            idx += 1;
        }
    }
    results
}