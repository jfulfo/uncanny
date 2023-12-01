
use video_rs::*;
use std::path::PathBuf;
use image::RgbImage;
use ndarray::{ArrayBase, OwnedRepr, Dim};
use crate::canny::canny;

const DEFAULT_HEIGHT: u32 = 480;
const DEFAULT_WIDTH: u32 = 720;

fn get_canny_frame(frame: ArrayBase<OwnedRepr<u8>, Dim<[usize; 3]>>) -> ArrayBase<OwnedRepr<u8>, Dim<[usize; 3]>> {
    // convert frame to RgbImage
    let height = frame.shape()[0];
    let width = frame.shape()[1];
    let mut image = RgbImage::new(width as u32, height as u32);
    image.copy_from_slice(&frame.into_raw_vec());
    let output_image = canny(&image);
    let output_frame = ArrayBase::from_shape_vec((DEFAULT_HEIGHT as usize, DEFAULT_WIDTH as usize, 3), output_image.to_vec()).unwrap();
    output_frame
}

/**
 * Iterate through each frame of the video and apply the canny algorithm
 * return a new video with the edges
 */
pub(crate) fn process_video(input_path: &str, output_path: &str) -> Result<(), Error> {
    video_rs::init().unwrap();
    let mut encoder = Encoder::new(
        &PathBuf::from(output_path).into(),
        EncoderSettings::for_h264_yuv420p(DEFAULT_WIDTH as usize, DEFAULT_HEIGHT as usize, true),
    ).unwrap();
    let mut decoder = Decoder::new_with_options_and_resize(
        &PathBuf::from(input_path).into(),
        &Options::new_h264(),
        Resize::Exact(DEFAULT_WIDTH, DEFAULT_HEIGHT),
    ).unwrap();

    let mut frame_count = 1;
    decoder.decode_iter()
        .take_while(Result::is_ok)
        .for_each(|frame| {
            let frame_tuple = frame.unwrap();
            let timestamp = frame_tuple.0;
            let input_frame = frame_tuple.1; 
            let output_frame = get_canny_frame(input_frame);
            encoder.encode(&output_frame, &timestamp).unwrap();
            println!("processed frame {}", frame_count);
            frame_count += 1;
        });
    encoder.finish()
}
