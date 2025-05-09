use crate::{shape::Shape, util::{equal, RtcFl}};


#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: RtcFl,
    pub shape: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: RtcFl, shape: &'a dyn Shape) -> Self {
        Self { t, shape }
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        equal(self.t, other.t) && self.shape.id() == other.shape.id()
    }
}

pub struct Intersections<'a> {
    pub(crate) data: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new(mut data: Vec<Intersection<'a>>) -> Self {
        data.sort_unstable_by(|a, b| {
            a.t.partial_cmp(&b.t)
                .expect("Unable to sort intersections!")
        });
        Self { data }
    }

    pub fn hit(&self) -> Option<Intersection<'a>> {
        for n in self.data.iter() {
            if n.t >= 0.0 {
                return Some(*n);
            }
        }

        None
    }
}

mod tests {
    
}