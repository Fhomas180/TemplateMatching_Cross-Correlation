use ndarray::Array3;
use crate::algorithm::{normalize_image, dot_product, find_best_match};

pub fn build_templates(images: &Array3<f32>, labels: &[u8]) -> Vec<Vec<f32>>{
let num_classes = 10;
    let image_size = 28 * 28;
    let mut templates = vec![vec![0.0f32; image_size]; num_classes];
    let mut counts = vec![0usize; num_classes];

    // Step 1: sum up all images per class
    for (i, label) in labels.iter().enumerate() {
        let class = *label as usize;
        let img = images.slice(ndarray::s![i, .., ..]);
        let flat: Vec<f32> = img.iter().cloned().collect();

        for (j, val) in flat.iter().enumerate() {
            templates[class][j] += val;
        }
        counts[class] += 1;
    }

    // Step 2: average each template then normalize it
    for class in 0..num_classes {
        for val in templates[class].iter_mut() {
            *val /= counts[class] as f32;
        }
        templates[class] = normalize_image(&templates[class]);
    }

    templates
}

 pub fn classify_sequential(
    templates: &[Vec<f32>],
    test_images: &Array3<f32>,
) -> Vec<usize> {
    let num_images = test_images.shape()[0];
    let mut predictions = Vec::with_capacity(num_images);

    for i in 0..num_images {
        // Step 1: grab and flatten image
        let img = test_images.slice(ndarray::s![i, .., ..]);
        let flat: Vec<f32> = img.iter().cloned().collect();

        // Step 2: normalize it
        let normalized = normalize_image(&flat);

        // Step 3: score against all 10 templates
        let scores: Vec<f32> = templates
            .iter()
            .map(|t| dot_product(&normalized, t))
            .collect();

        // Step 4: pick the best match
        predictions.push(find_best_match(&scores));
    }

    predictions
}