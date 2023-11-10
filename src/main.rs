/*
    Uncanny: An edge detection runner
    using Canny edge detection operator
 */

mod canny;
mod args;

use image;

fn main() {
    let args = args::parse_args();

    let input_image = image::open(&args["input_path"]).unwrap();
    let input_image = input_image.to_rgb8();
    let output_image = canny::canny(&input_image);
    output_image.save(&args["output_path"]).unwrap();
    println!("Saved to {}", &args["output_path"]);
}
