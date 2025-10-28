use std::any::Any;

use crate::{color::Color, shape::Shape, tuples::Tuple};

pub enum PatternEnum {
    StripePattern(Color, Color),
}

pub trait Pattern: Send + Sync + std::fmt::Debug + Any {
    fn pattern_at(&self, world_point: Tuple) -> Color;
    fn pattern_at_object(&self, shape: &dyn Shape, object_point: Tuple) -> Color;
    fn eq_pattern(&self, other: &dyn Pattern) -> bool;
    fn clone_boxed(&self) -> Box<dyn Pattern>;
    fn as_any(&self) -> &dyn Any;
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
}

impl Pattern for StripePattern {
    fn pattern_at(&self, point: Tuple) -> Color {

        if point.x.floor().abs() as usize % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    fn pattern_at_object(&self, shape: &dyn Shape, world_point: Tuple) -> Color {
        let object_point = shape
            .transform()
            .try_inverse()
            .expect("Shape transform must be invertible for pattern calculation")
            * world_point;

            self.pattern_at(object_point)
    }

    fn eq_pattern(&self, other: &dyn Pattern) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |other| self == other)
    }

    fn clone_boxed(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
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
    use crate::{
        color::Color,
        pattern::{Pattern, StripePattern},
        tuples::point,
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
}
