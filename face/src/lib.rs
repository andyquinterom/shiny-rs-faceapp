use rustface::{Detector, FaceInfo, ImageData};
use image::{DynamicImage, GrayImage, Rgb};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use uuid::Uuid;

pub fn run_model(path: &str) -> Result<String, Box<dyn std::error::Error>> {
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
        let rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());
        draw_hollow_rect_mut(&mut rgb, rect, Rgb([255, 0, 0]));
    }

    let relative_path = format!("/results/{}.png", Uuid::new_v4());
    let save_path = format!("static{}", relative_path);

    rgb.save(save_path.clone())?;

   Ok(relative_path)

}



fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
    let (width, height) = gray.dimensions();
    let mut image = ImageData::new(gray, width, height);
    let faces = detector.detect(&mut image);
    faces
}
