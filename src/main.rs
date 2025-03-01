use clap::{Arg, Command};
use image::{GrayImage, Luma};
use imageproc::gradients::sobel_gradients;

fn main() {
    let matches = Command::new("Edge Detection CLI")
        .version("1.0")
        .about("Applies edge detection to an image")
        .arg(
            Arg::new("input")
                .help("Path to the input image")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Path to save the output image")
                .required(true)
                .index(2),
        )
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();

    // 画像をロード
    let img = image::open(input_path).expect("Failed to open image");

    // グレースケール変換
    let gray_img: GrayImage = img.to_luma8();

    // Sobelフィルターを適用（エッジ検出、結果は u16 型）
    let edge_img_u16 = sobel_gradients(&gray_img);

    // u16 を u8 に変換
    let edge_img_u8 = GrayImage::from_fn(edge_img_u16.width(), edge_img_u16.height(), |x, y| {
        let pixel = edge_img_u16.get_pixel(x, y)[0];
        let scaled_pixel = (pixel as f32 / u16::MAX as f32 * 255.0) as u8;
        Luma([scaled_pixel])
    });

    // 画像を保存
    edge_img_u8
        .save(output_path)
        .expect("Failed to save processed image");

    println!("Edge detection completed. Saved to {}", output_path);
}
