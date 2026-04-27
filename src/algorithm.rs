pub fn normalize_image(img: &[f32]) -> Vec<f32> {
let mean: f32 = img.iter().sum::<f32>() / img.len() as f32;
let std: f32 = (img.iter().map(|x| (x - mean).powi(2))
        .sum::<f32>() / img.len() as f32)
        .sqrt();
    
    if std == 0.0 {
        return vec![0.0; img.len()];
    }
    
    img.iter().map(|x| (x - mean) / std).collect()
}

pub fn dot_product(a: &[f32], b: &[f32]) -> f32{
a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

pub fn find_best_match(scores: & [f32]) -> usize{
        scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0)
}