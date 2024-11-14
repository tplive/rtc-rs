use crate::colors::Color;
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

    fn format_pixels_as_ppm(&self) -> String {
        let ppm_width = 70;
        let mut formatted = String::from("");
        for c in &self.data {
            let color_data_str = format!("{} {} {} ", c.red.to_string(), c.green.to_string(), c.blue.to_string());
            formatted.push_str(&color_data_str);

        }
        String::from(formatted)
    }

    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index: usize = y * self.width + x;
        self.data[index] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.data[y * self.width + x]
    }

    pub fn to_ppm(&self) -> String {
        let head = format!("P3\n{} {}\n255\n", &self.width.to_string(), &self.height.to_string());
        let pixel_data = self.format_pixels_as_ppm();

        String::from(head + &pixel_data)
    }
}

