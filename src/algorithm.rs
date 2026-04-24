use image::{GrayImage, ImageBuffer, Luma};
use imageproc::template_matching::{match_template, MatchTemplateMethod};

fn templateMatching(){
    let img_raw: Vec<u8> = vec![0; 100 * 100];
    let temp_raw: Vec<u8> = vec![25u8,10 * 10];

    let img: GrayImage = ImageBuffer::from_raw(100,100,img_raw).unwrap();
    let template: GrayImage = ImageBuffer::from_raw(10,10, temp_raw).unwrap();

    let result = match_template(
        &img,
        &template,
        MatchTemplateMethod::SumofSquaredErrorsNormalized
    );

    let threshold = 10.0;
    for(x,y, score)in result.enumerate_pixels(){
        if score.0 < threshold {
            println!("Match found at ({}, {}) with score {}", x, y, score.0);
        }
    }

    
}
fn normalize_image(img: &[f32]) -> Vec<f32> {

}

fn dot_product(a: &[f32], b: &[f32]) -> f32{

}

fn find_best_match(scores: & [f32]) -> usize{
    
}