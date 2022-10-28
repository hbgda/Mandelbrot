use std::time::Instant;

use image::{ImageBuffer, RgbImage, Rgb};
use num::complex::Complex;
use rayon::prelude::{ParallelIterator, ParallelBridge};

const MAX_ITERATIONS: usize = 100;

const WIDTH: u32 = 28_000;
const HEIGHT: u32 = 16_000;

fn main() {
    println!("Creating mandelbrot: {WIDTH}x{HEIGHT}");

    // Use parallel by default as its the fastest
    benchmark("Mandelbrot [Parallel]", create_mandelbrot_parallel, true, "./mandel_parallel.png");
}

fn get_colour(iter: usize) -> Rgb<u8> {
    // Return white if iter == MAX_ITERATIONS,
    // meaning the number is part of the mandelbrot set
    if iter == MAX_ITERATIONS {
        return Rgb([255, 255, 255]);
    }
    
    // Arbitrary colour calculation for any numbers outside of the mandelbrot set,
    // The more iterations they last, the closer they are to white
    let colour = (iter % 255) as u8;
    Rgb([colour, colour, colour])
}

// Generic benchmarking to test how long each function takes to create the set
fn benchmark(title: &str, f: fn(u32, u32) -> RgbImage, save_img: bool, path: &str) {
    let now = Instant::now();
    let img = f(WIDTH, HEIGHT);
    let end = Instant::now() - now;

    let milis = end.as_millis();
    let secs = end.as_secs();

    println!("{title}: {secs}s | {milis}ms");
    if save_img {
        img.save(path).expect("Failed to save generated image.");
    }
}

// Using num::complex::Complex for calculations instead of individual f32s,
// assumed this would be faster, but maybe my implementation is flawed? 
fn escape_sequence_complex(origin: Complex<f32>) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    let mut iter: usize = 0;

    while z.norm_sqr() <= 4.0 && iter < MAX_ITERATIONS {
        z = z * z + origin;
        iter += 1;
    }

    iter
}

fn create_mandelbrot_complex(w: u32, h: u32) -> RgbImage {

    let mut img: RgbImage = ImageBuffer::new(w, h);
    for x in 0..w {
        for y in 0..h {

            let complex = Complex::new(
                (x as f32 / w as f32) * 3.5 - 2.5,
                (y as f32 / h as f32) * 2.0 - 1.0
            );

            let iter: usize = escape_sequence_complex(complex);
            img.put_pixel(x, y, get_colour(iter));
        }
    }

    img
}


// Standard implementation of the escape sequence algorithm
fn escape_sequence(x0: f32, y0: f32) -> usize {
    let mut iter: usize = 0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut x_sqr: f32 = 0.0;
    let mut y_sqr: f32 = 0.0;

    while x_sqr + y_sqr <= ( 4.0 ) && iter < MAX_ITERATIONS {

        let xt = x_sqr - y_sqr + x0;
        y = 2.0 * x * y + y0;
        x = xt;

        x_sqr = x * x;
        y_sqr = y * y;

        iter += 1;
    }

    iter
}

fn create_mandelbrot(w: u32, h: u32) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(w, h);
    for x in 0..w {
        let x0: f32 = (x as f32 / w as f32) * 3.5 - 2.5;
        for y in 0..h {
            let y0: f32 = (y as f32 / h as f32) * 2.0 - 1.0;
            let iter: usize = escape_sequence(x0, y0);
            img.put_pixel(x, y, get_colour(iter));
        }
    }

    img
}

fn create_mandelbrot_parallel(w: u32, h: u32) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(w, h);

    img.enumerate_rows_mut().par_bridge().for_each(|(y, row)| {
        for x in row {
            let x0: f32 = (x.0 as f32 / w as f32) * 3.5 - 2.5;
            let y0: f32 = (y as f32 / h as f32) * 2.0 - 1.0;
            let iter: usize = escape_sequence(x0, y0);
            *x.2 = get_colour(iter);
        }
    });


    // img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
    //     let x0: f32 = (x as f32 / w as f32) * 3.5 - 2.5;
    //     let y0: f32 = (y as f32 / h as f32) * 2.0 - 1.0;
    //     let iter: usize = escape_sequence(x0, y0);
    //     *pixel = get_colour(iter);
    // });


    // img.par_iter_mut().enumerate().for_each(|(i, pixel)| {
    //     let x: u32 = ((i / 3) as u32 % w) as u32;
    //     let y: u32 = ((i / 3) as u32 / w) as u32;
    //     if i % 3 != 0 {
    //         *pixel = 255;
    //         return;
    //     }
    //     let x0: f32 = (x as f32 / w as f32) * 3.5 - 2.5;
    //     let y0: f32 = (y as f32 / h as f32) * 2.0 - 1.0;
    //     let iter: usize = escape_sequence(x0, y0);
    //     let colour = (iter / MAX_ITERATIONS) as u8 * 255;
    //     *pixel = colour;
    // });

    img
}