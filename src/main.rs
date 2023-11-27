/*
    Uncanny: An edge detection runner
    using Canny edge detection operator
 */

mod args;
//mod video_processor;
mod canny;

use image;
use std::time::Instant;


fn main() {
    let args = args::parse_args();
    let is_video = args["is_video"].parse::<bool>().unwrap();
    let start = Instant::now();

    if is_video {
        //video_processor::process_video(&args["input_path"], &args["output_path"]);
        println!("Video processing not yet implemented")
    } else {
        let input_image = image::open(&args["input_path"]).unwrap().to_rgb8();
        let input_time = start.elapsed();
        println!("Image loaded: {:?}", start.elapsed());
        let output_image = canny::canny(&input_image);
        println!("Canny: {:?}", start.elapsed() - input_time);
        output_image.save(&args["output_path"]).unwrap();
    }
    let duration = start.elapsed();

    println!("Saved to {}", &args["output_path"]);
    println!("Took {:?}ms", duration.as_millis());
}
