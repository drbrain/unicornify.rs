use crate::render::Tracer;
use crate::render::WorldView;
use crate::render::Bounds;
use crate::geometry::Point;

#[derive(Clone, Debug)]
pub struct TranslatingTracer {
    source: Box<Tracer>,
    pub shift: Point,
    pub bounds: Bounds,
    world_view: WorldView,
}

impl TranslatingTracer {
    pub fn new(world_view: WorldView, source: Tracer, shift: Point) -> Self {
        let mut bounds = source.bounds();

        if !bounds.empty {
            bounds.x_min += shift.x;
            bounds.x_max += shift.x;
            bounds.y_min += shift.y;
            bounds.y_max += shift.y;
        }

        let source = Box::new(source);

        TranslatingTracer { source, shift, bounds, world_view }
    }
}
