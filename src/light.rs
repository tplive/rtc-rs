use crate::{color::Color, material::Material, shape::Shape, tuples::Tuple};

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
    object: &dyn Shape,
    light: &Light,
    point: &Tuple,
    eye_vector: &Tuple,
    normal_vector: &Tuple,
    in_shadow: bool,
) -> Color {
    let effective_color = if let Some(pattern) = &material.pattern {
        pattern.pattern_at_object(object, *point) * light.intensity
    } else {
        material.color * light.intensity
    };

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

    if in_shadow {
        ambient
    } else {
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    use crate::material::Material;
    use crate::{
        color::Color,
        light::{lighting, Light},
        sphere::Sphere,
        tuples::{point, vector},
    };

    #[test]
    fn a_point_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = point(0.0, 0.0, 0.0);
        let light = Light::point(position, intensity);

        assert!(light.position.eq(&position));
        assert!(light.intensity.eq(&intensity));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::point(point(0.0, 0.0, -10.0), Color::white());
        let result = lighting(
            &m,
            &Sphere::default(),
            &light,
            &position,
            &eyev,
            &normalv,
            false,
        );

        //println!("{:?}", &result);
        assert!(Color::new(1.9, 1.9, 1.9).eq(&result));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = Material::default();
        let sqrt2over2 = 2.0_f32.sqrt() / 2.0;
        let position = point(0.0, 0.0, 0.0);

        let eyev = point(0.0, sqrt2over2, -sqrt2over2);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::point(point(0.0, 0.0, -10.0), Color::white());
        let result = lighting(
            &m,
            &Sphere::default(),
            &light,
            &position,
            &eyev,
            &normalv,
            false,
        );

        //println!("{:?}", &result);
        assert!(Color::new(1.0, 1.0, 1.0).eq(&result));
    }

    #[test]
    fn lighting_with_eye_oposite_surface_light_offset_45_degrees() {
        let m = Material::default();
        let sqrt2over2 = 2.0_f32.sqrt() / 2.0;
        let position = point(0.0, 0.0, 0.0);

        let eyev = point(0.0, sqrt2over2, sqrt2over2);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::point(point(0.0, 10.0, -10.0), Color::white());
        let result = lighting(
            &m,
            &Sphere::default(),
            &light,
            &position,
            &eyev,
            &normalv,
            false,
        );

        //println!("{:?}", &result);
        assert!(Color::new(0.7364, 0.7364, 0.7364).eq(&result));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::default();
        let sqrt2over2 = 2.0_f32.sqrt() / 2.0;
        let position = point(0.0, 0.0, 0.0);

        let eyev = point(0.0, -sqrt2over2, -sqrt2over2);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::point(point(0.0, 10.0, -10.0), Color::white());
        let result = lighting(
            &m,
            &Sphere::default(),
            &light,
            &position,
            &eyev,
            &normalv,
            false,
        );

        //println!("{:?}", &result);
        assert!(Color::new(1.6364, 1.6364, 1.6364).eq(&result));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);

        let eyev = point(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::point(point(0.0, 0.0, 10.0), Color::white());
        let result = lighting(
            &m,
            &Sphere::default(),
            &light,
            &position,
            &eyev,
            &normalv,
            false,
        );

        //println!("{:?}", &result);
        assert!(Color::new(0.1, 0.1, 0.1).eq(&result));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::point(point(0.0, 0.0, -10.0), Color::white());
        let in_shadow = true;
        let result = lighting(
            &m,
            &Sphere::default(),
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
        );

        assert!(Color::new(0.1, 0.1, 0.1).eq(&result));
    }
}
