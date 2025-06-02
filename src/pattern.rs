use crate::{color::Color, tuples::Tuple};

pub struct StripePattern {
    pub a: Color,
    pub b: Color,
}

impl StripePattern {
    fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn stripe_at(&self, point: Tuple) -> Color {
        if point.x.floor().abs() as usize % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Default for StripePattern {
    fn default() -> Self {
        Self {
            b: Color::black(),
            a: Color::white(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, pattern::StripePattern, tuples::point};

    #[test]
    fn creating_stripe_pattern() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert!(pattern.a == Color::white());
        assert!(pattern.b == Color::black())
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.stripe_at(point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.stripe_at(point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.stripe_at(point(-1.1, 0.0, 0.0)), Color::white());
    }
}
