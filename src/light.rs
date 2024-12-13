use crate::{color::Color, material::Material, tuples::Tuple};
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn point(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

pub fn lighting(
    material: &Material,
    light: &Light,
    point: &Tuple,
    eye_vector: &Tuple,
    normal_vector: &Tuple,
) -> Color {
    let effective_color = material.color * light.intensity;
    let light_vector = (light.position - *point).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = light_vector.dot(*normal_vector);

    let diffuse: Color;
    let specular: Color;

    if light_dot_normal < 0.0 {
        diffuse = Color::black();
        specular = Color::black();
    } else {
        diffuse = effective_color * (material.diffuse * light_dot_normal);

        let reflect_vector = -light_vector.reflect(*normal_vector);
        let reflect_dot_eye = reflect_vector.dot(*eye_vector);

        if reflect_dot_eye <= 0.0 {
            specular = Color::black();
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}
