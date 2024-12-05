use image::{ ImageBuffer, Luma };

pub fn create_bw_image(map: &Vec<Vec<u8>>) -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    let min_width = 256;
    let original_width = map.get(0).map_or(0, |row| row.len()) as u32;
    let scale = if original_width < min_width {
        ((min_width as f32) / (original_width as f32)).ceil() as u32
    } else {
        1
    };

    let height = (map.len() as u32) * scale;
    let width = original_width * scale;

    let mut imgbuf = ImageBuffer::new(width, height);

    for (y, row) in map.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            for dy in 0..scale {
                for dx in 0..scale {
                    let pixel = imgbuf.get_pixel_mut(
                        (x as u32) * scale + dx,
                        (y as u32) * scale + dy
                    );
                    *pixel = if val == 1 { Luma([255]) } else { Luma([0]) };
                }
            }
        }
    }

    imgbuf
}

pub fn save_image(file_path: String, imgbuf: image::ImageBuffer<Luma<u8>, Vec<u8>>) {
    imgbuf.save(file_path).unwrap();
}
