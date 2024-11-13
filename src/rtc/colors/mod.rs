pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {red: red, green: green, blue: blue}
    }
}

pub fn color(r: f32, g: f32, b: f32) -> Color {
    Color::new(r, g, b)
}

#[cfg(test)]
mod tests;