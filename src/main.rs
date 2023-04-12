extern crate image;
use std::fs::File;
use std::io::{BufReader, Read};
use image::{Rgb, RgbImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Opens file
    let file = File::open("C:\\Users\\Athar\\Downloads\\UCL.pdf")?;
    let mut buf_reader = BufReader::new(file);

    // Reads the file
    let mut data = Vec::new();

    // Convert the buffer to binary data
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = buf_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        for byte in buffer[..bytes_read].iter() {
            for i in 0..8 {
                let bit = (byte >> i) & 1;
                if bit == 1 {
                    data.push(255);
                } else {
                    data.push(0);
                }
            }
        }
    }

    // Compute the image dimensions based on the binary data size
    let len = data.len();
    let width = (len as f32).sqrt().ceil() as u32;
    let height = ((len as f32) / (width as f32)).ceil() as u32;

    // Create a new RGB image with the computed dimensions
    let mut img = RgbImage::new(width, height);

    // Iterate over each byte in the binary data and set the corresponding pixel color
    for (i, chunk) in data.chunks_exact(3).enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        let r = chunk.get(0).cloned().unwrap_or(0);
        let g = chunk.get(1).cloned().unwrap_or(0);
        let b = chunk.get(2).cloned().unwrap_or(0);
        let color = Rgb([r, g, b]);
        img.put_pixel(x, y, color);
    }

    img.save("output.png")?;

    Ok(())
}


