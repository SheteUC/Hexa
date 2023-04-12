

extern crate image;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use image::{RgbImage, Rgb, ImageError};
use image::png::PngEncoder;

fn main() -> std::io::Result<()> {
    // Open the PDF or DOC file
    let mut file = File::open("C:\\Users\\Athar\\Downloads\\UCL.pdf")?;

    // Read the file contents into a buffer
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    // Convert the buffer to binary data
    let mut data = Vec::new();
    for byte in buf.iter() {
        for i in 0..8 {
            let bit = (byte >> i) & 1;
            if bit == 1 {
                data.push(255);
            } else {
                data.push(0);
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
    for (i, byte) in data.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        let color = Rgb([*byte, *byte, *byte]);
        img.put_pixel(x, y, color);
    }

    // Save the image as a PNG file
    // img.save("output.png")?;
    match img.save("output.png") {
        Ok(_) => println!("Image saved successfully"),
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    }

    Ok(())
}


// fn main() -> std::io::Result<()> {
//     // Open the image file
//     let img = image::open("path/to/image.png")?;

//     // Get the image dimensions and allocate a buffer for the binary data
//     let (width, height) = img.dimensions();
//     let mut data = Vec::with_capacity((width * height) as usize / 8);

//     // Iterate over each pixel in the image and extract the corresponding binary data
//     for y in 0..height {
//         for x in 0..width {
//             let pixel = img.get_pixel(x, y).0;
//             if pixel[0] > 127 {
//                 data.push(1);
//             } else {
//                 data.push(0);
//             }
//         }
//     }

//     // Convert the binary data back to bytes
//     let mut buf = Vec::new();
//     for i in 0..(data.len() / 8) {
//         let mut byte = 0;
//         for j in 0..8 {
//             if data[i*8 + j] == 1 {
//                 byte |= 1 << j;
//             }
//         }
//         buf.push(byte);
//     }

//     // Write the bytes to a new PDF or DOC file
//     let mut output_file = File::create("output.pdf")?;
//     output_file.write_all(&buf)?;

//     Ok(())
// }
