use ndarray::Array3;
use rayon::prelude::*;
use crate::algorithm::{normalize_image, dot_product, find_best_match};

pub fn classify_parallel(
    templates: &[Vec<f32>],
    test_images: &Array3<f32>,
) -> Vec<usize> {
    let num_images = test_images.shape()[0];

    (0..num_images)
        .into_par_iter()          // rayon takes over here — splits work across threads
        .map(|i| {
            // Step 1: grab image i and flatten it
            let img = test_images.slice(ndarray::s![i, .., ..]);
            let flat: Vec<f32> = img.iter().cloned().collect();

            // Step 2: normalize it
            let normalized = normalize_image(&flat);

            // Step 3: score against every template
            let scores: Vec<f32> = templates
                .iter()
                .map(|t| dot_product(&normalized, t))
                .collect();

            // Step 4: return the best matching digit
            find_best_match(&scores)
        })
        .collect()
}