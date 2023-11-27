/*
    Process video for edge detection
    Each frame is processed individually
    and then written to a new video file
 */

extern crate ffmpeg_next as ffmpeg;

use ffmpeg::{codec, format, frame, media, software::scaling, Packet, Rational};
use ffmpeg_next::codec::traits::Decoder;
use crate::canny;

pub(crate) fn process_video(input_path: &str, output_path: &str) -> Result<(), ffmpeg_next::Error> {
    ffmpeg_next::init().unwrap();

    let mut input_format_context = format::input(&input_path).unwrap();

    let input_stream = input_format_context
        .streams()
        .best(media::Type::Video)
        .unwrap();
    let input_stream_index = input_stream.index();

    let codec_param = input_stream.parameters();

    let codec = codec::decoder::find(codec_param.codec_id()).unwrap();

    let mut decoder = codec.decoder().video().unwrap();
    decoder.set_parameters(codec_param).unwrap();

    let mut output_format_context = format::output(&output_path).unwrap();
    let global_header = output_format_context.format().flags().contains(format::Flags::GLOBAL_HEADER);

    let mut output_stream = output_format_context.add_stream(decoder.parameters()).unwrap();
    if global_header {
        output_stream.set_codec_tag(0);
    }

    output_stream.write_header().unwrap();

    for (stream, packet) in input_format_context.packets() {
        if stream.index() == input_stream_index {
            decoder.send_packet(&packet).unwrap();
            let mut decoded = frame::Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let input_image = decoded.as_rgb().unwrap();
                let canny_image = canny::canny(&input_image);
                let mut output_image = frame::Video::new(
                    format::Pixel::RGB24,
                    canny_image.width() as u32,
                    canny_image.height() as u32,
                );
                output_image.planes()[0].copy_from_slice(canny_image.as_raw());
                output_image.set_pts(decoded.pts());
                let mut scaler = scaling::Context::get(
                    format::Pixel::RGB24,
                    canny_image.width(),
                    canny_image.height(),
                    format::Pixel::YUV420P,
                    canny_image.width(),
                    canny_image.height(),
                    scaling::Flags::BILINEAR,
                ).unwrap();
                scaler.run(&output_image, &mut decoded).unwrap();
                output_stream.send_frame(&decoded).unwrap();
            }
        }
    }

    output_stream.write_trailer().unwrap();

    Ok(())
}