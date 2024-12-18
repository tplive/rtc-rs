use crate::{color::Color, util::RtcFl};
#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<Color> = vec![Color::black(); width * height];

        Self {
            width,
            height,
            data,
        }
    }

    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index: usize = y * self.width + x;
        if index >= self.data.len() { return }
        self.data[index] = color;
    }

    pub fn write_rect(&mut self, x: usize, y:usize, w: usize, h: usize, color: Color) {
        for i in x..x+w {
            for j in y..y+h {
                Self::write_pixel(self, i, j, color);
            }
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.data[y * self.width + x]
    }

    pub fn to_ppm(&self) -> String {
        let color_vector: Vec<u8> = self
            .data
            .iter()
            .flat_map(|col| {
                vec![
                    Self::scale(col.red),
                    Self::scale(col.green),
                    Self::scale(col.blue),
                ]
            })
            .collect();

        let mut ppm = format!(
            "P3\n{} {}\n255\n",
            &self.width.to_string(),
            &self.height.to_string()
        );

        let max_line: usize = 70;

        let mut line_length = 0;

        for v in color_vector.iter() {
            let c_str = v.to_string(); // Scaled color 0.0..1.0 --> 0..255

            if line_length + c_str.len() + 1 > max_line {
                // If we will overshoot max_line limit, add newline
                ppm.push('\n');
                line_length = 0;
            }

            if line_length > 0 {
                // If we are in the middle of a line, add a space
                ppm.push(' ');
                line_length += 1; // Remember to increment line_length as well!
            }

            ppm.push_str(&c_str); // Push color value

            line_length += c_str.len() // Add length of value to line length
        }

        ppm.push('\n'); // Always end on a newline
        ppm
    }

    pub fn to_png(&self) -> Vec<u8> {
        self.data.iter().flat_map(|col| {
            vec![
                Self::scale(col.red),
                Self::scale(col.green),
                Self::scale(col.blue),
                255, // Alpha channel
            ]
        }).collect()
    }

    fn scale(v: RtcFl) -> u8 {
        let scaled = (v * 255.0).round();
        scaled.clamp(0.0, 255.0) as u8
    }
}
