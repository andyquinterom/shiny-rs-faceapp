use rustface::{Detector, FaceInfo, ImageData, Rectangle};
use image::{DynamicImage, GrayImage, Rgb, RgbImage};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use std::path::Path;

fn draw_face(rgb: &mut RgbImage, bbox: &Rectangle, weight: i32) {
    for i in 0..weight {
        let rect = Rect::at(bbox.x() + i, bbox.y() + i)
            .of_size(bbox.width() - i as u32 * 2, bbox.height() - i as u32 * 2);
        draw_hollow_rect_mut(rgb, rect, Rgb([255, 0, 0]));
    }
}

pub fn run_model(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_name = Path::new(&path).file_name().unwrap().to_os_string().into_string().unwrap();
    let save_path = format!("./static/results/{}.png", file_name);
    if Path::new(&save_path).exists() {
        return Ok(save_path);
    }
    let mut detector = rustface::create_detector("./model.bin").unwrap();
    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);

    let image: DynamicImage = image::open(path)?;

    let mut rgb = image.to_rgb8();

    let faces = detect_faces(&mut *detector, &image.to_luma8());

    for face in faces {
        let bbox = face.bbox();
        draw_face(&mut rgb, bbox, 4);
    }


    rgb.save(save_path.clone())?;

   Ok(save_path)

}



fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
    let (width, height) = gray.dimensions();
    let mut image = ImageData::new(gray, width, height);
    let faces = detector.detect(&mut image);
    faces
}
