#![feature(test)]

extern crate test;
extern crate oxipng;

use oxipng::png;
use test::Bencher;
use std::path::PathBuf;

#[bench]
fn bench_16_to_8_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgb_16_should_be_rgb_8.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_bit_depth();
    });
}

#[bench]
fn bench_8_to_4_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/palette_8_should_be_palette_4.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_bit_depth();
    });
}

#[bench]
fn bench_8_to_2_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/palette_8_should_be_palette_2.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_bit_depth();
    });
}

#[bench]
fn bench_8_to_1_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/palette_8_should_be_palette_1.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_bit_depth();
    });
}

#[bench]
fn bench_4_to_2_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/palette_4_should_be_palette_2.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_bit_depth();
    });
}

#[bench]
fn bench_4_to_1_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/palette_4_should_be_palette_1.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_bit_depth();
    });
}

#[bench]
fn bench_2_to_1_bits(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/palette_2_should_be_palette_1.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_bit_depth();
    });
}

#[bench]
fn bench_rgba_to_rgb_16(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgba_16_should_be_rgb_16.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_color_type();
    });
}

#[bench]
fn bench_rgba_to_rgb_8(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgba_8_should_be_rgb_8.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_color_type();
    });
}

#[bench]
fn bench_rgba_to_grayscale_alpha_16(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgba_16_should_be_grayscale_alpha_16.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_color_type();
    });
}

#[bench]
fn bench_rgba_to_grayscale_alpha_8(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgba_8_should_be_grayscale_alpha_8.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_color_type();
    });
}

#[bench]
fn bench_rgba_to_grayscale_16(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgba_16_should_be_grayscale_16.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_color_type();
    });
}

#[bench]
fn bench_rgba_to_grayscale_8(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgba_8_should_be_grayscale_8.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_color_type();
    });
}

#[bench]
fn bench_rgb_to_grayscale_16(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgb_16_should_be_grayscale_16.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_color_type();
    });
}

#[bench]
fn bench_rgb_to_grayscale_8(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgb_8_should_be_grayscale_8.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_color_type();
    });
}

#[bench]
fn bench_rgba_to_palette_8(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgba_8_should_be_palette_8.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_color_type();
    });
}

#[bench]
fn bench_rgb_to_palette_8(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/rgb_8_should_be_palette_8.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_color_type();
    });
}

#[bench]
fn bench_palette_duplicate_reduction(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/palette_should_be_reduced_with_dupes.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_palette();
    });
}

#[bench]
fn bench_palette_unused_reduction(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/palette_should_be_reduced_with_unused.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_palette();
    });
}

#[bench]
fn bench_palette_full_reduction(b: &mut Bencher) {
    let input = test::black_box(PathBuf::from("tests/files/palette_should_be_reduced_with_both.png"));
    let mut png = png::PngData::new(&input).unwrap();

    b.iter(|| {
        png.reduce_palette();
    });
}
