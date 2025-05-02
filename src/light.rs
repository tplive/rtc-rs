use crate::{color::Color, material::Material, tuples::Tuple};

#[derive(Copy, Clone)]
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
        // Light is on the other side of the surface
        diffuse = Color::black();
        specular = Color::black();
    } else {
        // Light is on the same side of the surface
        // Diffuse lighting is based on the angle between the light vector and the normal vector
        diffuse = effective_color * material.diffuse * light_dot_normal;

        // Specular lighting is based on the angle between the reflection vector and the eye vector
        let reflect_vector = -light_vector.reflect(*normal_vector);
        let reflect_dot_eye = reflect_vector.dot(*eye_vector);

        // The reflect_dot_eye value is positive if the light reflects toward the eye
        if reflect_dot_eye <= 0.0 {
            specular = Color::black();
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}
