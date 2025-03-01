use clap::Parser;
use image::{GrayImage, Luma};
use imageproc::gradients::sobel_gradients;

/// CLI Arguments for the Edge Detection tool
#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Applies edge detection to an image")]
struct Args {
    /// Path to the input image
    input: String,

    /// Path to save the output image
    output: String,
}

fn main() {
    let args = Args::parse();

    // 画像をロード
    let img: image::DynamicImage = image::open(&args.input).expect("Failed to open image");

    // グレースケール変換
    let gray_img: GrayImage = img.to_luma8();

    // Sobelフィルターを適用（エッジ検出、結果は u16 型）
    let edge_img_u16 = sobel_gradients(&gray_img);

    // 最小値・最大値を求める
    let (min, max) = edge_img_u16.pixels().fold((u16::MAX, 0), |(min, max), p| {
        let v = p[0];
        (min.min(v), max.max(v))
    });

    // min == max の場合、すべて同じ値なので正規化できない（全黒になる）
    if min == max {
        println!("Warning: No edge detected (min == max). The output image may be blank.");
    }

    // u16 を u8 に正規化（コントラストを調整）
    let edge_img_u8 = GrayImage::from_fn(edge_img_u16.width(), edge_img_u16.height(), |x, y| {
        let pixel = edge_img_u16.get_pixel(x, y)[0];
        let scaled_pixel = if max > min {
            ((pixel - min) as f32 / (max - min) as f32 * 255.0) as u8
        } else {
            0 // min == max の場合、全ピクセルが同じ値になるので0にする
        };
        Luma([scaled_pixel])
    });

    // 画像を保存
    edge_img_u8
        .save(&args.output)
        .expect("Failed to save processed image");

    println!("Edge detection completed. Saved to {}", &args.output);
}
