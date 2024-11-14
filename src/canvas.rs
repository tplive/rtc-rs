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
        String::from(head)
    }
}

