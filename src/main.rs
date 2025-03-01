use clap::{Parser, ValueEnum};
use image::{GrayImage, ImageBuffer, Luma};
use imageproc::gradients::sobel_gradients;
use imageproc::edges::canny;
use std::error::Error;

/// CLI Arguments for the Edge Detection tool
#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Applies edge detection to an image")]
struct Args {
    /// Path to the input image
    input: String,

    /// Path to save the output image
    output: String,

    /// Edge detection method: Sobel or Canny
    #[arg(short, long, value_enum, default_value = "sobel")]
    method: EdgeDetectionMethod,
}

/// Edge detection methods
#[derive(Debug, Clone, ValueEnum)]
enum EdgeDetectionMethod {
    Sobel,
    Canny,
}

fn main() {
    let args = Args::parse();
    println!("method: {:?}", args.method);
    match process_image(&args.input, &args.output, args.method) {
        Ok(_) => println!("Edge detection complete!!! Saved to {}", &args.output),
        Err(error) => println!("Error: {}", error),
    }
}

fn process_image(input_path: &str, output_path: &str, method: EdgeDetectionMethod) -> Result<(), Box<dyn Error>> {
    let img = image::open(input_path)?;
    let gray_img = img.to_luma8();

    let edge_img_u8 = match method {
        EdgeDetectionMethod::Sobel => {
            let edge_img_u16 = sobel_gradients(&gray_img);
            normalize_u16_to_u8(&edge_img_u16)
        }
        EdgeDetectionMethod::Canny => canny(&gray_img, 50.0, 100.0),
    };

    edge_img_u8.save(output_path)?;
    Ok(())
}

fn normalize_u16_to_u8(img: &ImageBuffer<Luma<u16>, Vec<u16>>) -> GrayImage {
    let (min, max) = img.pixels().fold((u16::MAX, 0), |(min, max), p| {
        let v = p[0];
        (min.min(v), max.max(v))
    });

    // min == max の場合、すべて同じ値なので全黒
    if min == max {
        println!("Warning: No edge detected (min == max). The output image may be blank.");
    }

    GrayImage::from_fn(img.width(), img.height(), |x, y| {
        let pixel = img.get_pixel(x, y)[0];
        let scaled_pixel = if max > min {
            ((pixel - min) as f32 / (max - min) as f32 * 255.0) as u8
        } else {
            0
        };
        Luma([scaled_pixel])
    })
}
