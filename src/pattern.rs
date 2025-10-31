use crate::{color::Color, shape::Shape, tuples::Tuple};

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Stripe(StripePattern),
    // Gradient(GradientPattern),
    // Ring(RingPattern),
    // Checker(CheckerPattern),
}

impl Pattern {
    pub fn pattern_at(&self, point: Tuple) -> Color {
        match self {
            Pattern::Stripe(p) => p.pattern_at(point),
            // Match other patterns
        }
    }

    pub fn pattern_at_object(&self, shape: &dyn Shape, world_point: Tuple) -> Color {
        let object_point = shape
            .transform()
            .try_inverse()
            .expect("Shape transform must be invertible for pattern calculation")
            * world_point;

        self.pattern_at(object_point)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        if point.x.floor().abs() as usize % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        color::Color, material::Material, pattern::{Pattern, StripePattern}, sphere::Sphere,
        transformation::Transformation, tuples::point,
    };

    #[test]
    fn creating_stripe_pattern() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert!(pattern.a == Color::white());
        assert!(pattern.b == Color::black());
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(Color::white(), Color::black());
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
        let p = StripePattern::new(Color::white(), Color::black());
        let mut m = Material::default();
        m.pattern = Some(Pattern::Stripe(StripePattern::new(Color::white(), Color::black())));

        let object = Sphere::new(t.get(), m);
        

    }
}
