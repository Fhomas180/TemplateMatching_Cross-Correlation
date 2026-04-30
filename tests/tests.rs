use rustFinalProject::algorithm::{normalize_image, dot_product, find_best_match};

#[test]
fn test_dot_product() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![4.0, 5.0, 6.0];
    assert_eq!(dot_product(&a, &b), 32.0);
}

#[test]
fn test_find_best_match() {
    let scores = vec![0.1, 0.5, 0.9, 0.3];
    assert_eq!(find_best_match(&scores), 2);
}

#[test]
fn test_normalize_image() {
    let img = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = normalize_image(&img);
    let mean: f32 = result.iter().sum::<f32>() / result.len() as f32;
    assert!(mean.abs() < 0.001);
}