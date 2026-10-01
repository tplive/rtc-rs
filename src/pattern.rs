use std::ops::Sub;

use crate::{color::Color, matrix::Matrix4, shape::Shape, tuples::Tuple, util::RtcFl};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    kind: PatternKind,
    transform: Matrix4,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
    Stripe { a: Color, b: Color },
    Test,
    Gradient { a: Color, b: Color },
    Ring { a: Color, b: Color },
    Checkers { a: Color, b: Color },
}

impl Pattern {
    pub fn stripe(a: Color, b: Color) -> Self {
        Self {
            kind: PatternKind::Stripe { a, b },
            transform: Matrix4::identity(),
        }
    }

    pub fn test() -> Self {
        Self {
            kind: PatternKind::Test,
            transform: Matrix4::identity(),
        }
    }

    pub fn gradient(a: Color, b: Color) -> Self {
        Self {
            kind: PatternKind::Gradient { a, b },
            transform: Matrix4::identity(),
        }
    }

    pub fn ring(a: Color, b: Color) -> Self {
        Self {
            kind: PatternKind::Ring { a, b },
            transform: Matrix4::identity(),
        }
    }

    pub fn checkered(a: Color, b: Color) -> Self {
        Self {
            kind: PatternKind::Checkers { a, b },
            transform: Matrix4::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        match &self.kind {
            PatternKind::Stripe { a, b } => {
                if (point.x.floor().abs() as usize).is_multiple_of(2) {
                    *a
                } else {
                    *b
                }
            }
            PatternKind::Test => Color::new(point.x, point.y, point.z),
            PatternKind::Gradient { a, b } => *a + (b.sub(*a)) * (point.x - RtcFl::floor(point.x)),
            PatternKind::Ring { a, b } => {
                if RtcFl::sqrt(point.x.powi(2) + point.z.powi(2)) % 2.0 == 0.0 {
                    *a
                } else {
                    *b
                }
            }
            PatternKind::Checkers { a, b } => {
                if (
                  RtcFl::floor(point.x) + 
                  RtcFl::floor(point.y) + 
                  RtcFl::floor(point.z)
                ) % 2.0 == 0.0
                {
                    *a
                } else {
                    *b
                }
            }
        }
    }

    pub fn pattern_at_object(&self, shape: &dyn Shape, world_point: Tuple) -> Color {
        let object_point = shape
            .transform()
            .try_inverse()
            .expect("Shape transform must be invertible for pattern calculation")
            * world_point;

        let pattern_point = self
            .transform
            .try_inverse()
            .expect("Pattern transform must be invertible for pattern calculation")
            * object_point;

        self.pattern_at(pattern_point)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix4,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix4::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }
}

pub struct TestPattern {
    pub transform: Matrix4,
}

impl TestPattern {
    pub fn new() -> Self {
        Self {
            transform: Matrix4::identity(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradientPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix4,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix4::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }
}

struct CheckersPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix4,
}

impl CheckersPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix4::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        color::Color,
        material::Material,
        matrix::Matrix4,
        pattern::Pattern,
        sphere::Sphere,
        transformation::{scaling, translation, Transformation},
        tuples::point,
    };

    #[test]
    fn creating_stripe_pattern() {
        let pattern = Pattern::stripe(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(point(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn stripes_with_object_transformation() {
        let t = Transformation::new().scaling(2.0, 2.0, 2.0);
        let m = Material {
            pattern: Some(Pattern::stripe(Color::white(), Color::black())),
            ..Default::default()
        };

        let object = Sphere::new(t.get(), m);
        let pattern = object.material.pattern.as_ref().unwrap();
        let color = pattern.pattern_at_object(&object, point(1.5, 0.0, 0.0));

        assert_eq!(color, Color::white());
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let transform = Transformation::new().scaling(2.0, 2.0, 2.0);

        let mut pattern = Pattern::stripe(Color::white(), Color::black());
        pattern.set_transform(transform.get());

        let material = Material {
            pattern: Some(pattern.clone()),
            ..Default::default()
        };

        let object = Sphere::new(transform.get(), material);
        let c = pattern.pattern_at_object(&object, point(1.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_pattern_and_object_transformation() {
        let ot = scaling(2.0, 2.0, 2.0);
        let pt = translation(0.5, 0.0, 0.0);

        let mut p = Pattern::stripe(Color::white(), Color::black());
        p.set_transform(pt);

        let m = Material {
            pattern: Some(p.clone()),
            ..Default::default()
        };

        let object = Sphere::new(ot, m);

        let c = p.pattern_at_object(&object, point(2.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn default_pattern_transformation() {
        let pattern = Pattern::test();

        assert_eq!(pattern.transform, Matrix4::identity());
    }

    #[test]
    fn assigning_transformation() {
        let mut pattern = Pattern::test();
        let trans = translation(1.0, 2.0, 3.0);

        pattern.transform = trans;

        assert_eq!(pattern.transform, translation(1.0, 2.0, 3.0));
    }

    #[test]
    fn pattern_with_object_transformation() {
        let shape = Sphere::new(scaling(2.0, 2.0, 2.0), Material::default());
        let pattern = Pattern::test();
        let color = pattern.pattern_at_object(&shape, point(2.0, 3.0, 4.0));

        assert_eq!(color, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let shape = Sphere::new(Transformation::new().get(), Material::default());
        let mut pattern = Pattern::test();
        pattern.set_transform(scaling(2.0, 2.0, 2.0));
        let color = pattern.pattern_at_object(&shape, point(2.0, 3.0, 4.0));

        assert_eq!(color, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_object_and_pattern_transformation() {
        let shape = Sphere::new(scaling(2.0, 2.0, 2.0), Material::default());
        let mut pattern = Pattern::test();
        pattern.set_transform(translation(0.5, 1.0, 1.5));
        let color = pattern.pattern_at_object(&shape, point(2.5, 3.0, 3.5));

        assert_eq!(color, Color::new(0.75, 0.5, 0.25));
    }

    #[test]
    fn a_gradient_pattern_linearly_interpolates_between_colors() {
        let pattern = Pattern::gradient(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(
            pattern.pattern_at(point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(point(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern::ring(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), Color::black());
        assert_eq!(pattern.pattern_at(point(0.708, 0.0, 0.708)), Color::black());
        // 0.708 is slightly more than sqrt2over2
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::checkered(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.99, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(1.01, 0.0, 0.0)), Color::black());
    }
    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::checkered(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.99, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 1.01, 0.0)), Color::black());
    }
    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::checkered(Color::white(), Color::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.99)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.01)), Color::black());
    }
}
