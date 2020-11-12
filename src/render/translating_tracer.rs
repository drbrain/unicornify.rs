use crate::geometry::Point;
use crate::geometry::Vector;
use crate::render::Bounds;
use crate::render::RenderingParameters;
use crate::render::TraceResult;
use crate::render::Tracer;
use crate::render::WorldView;

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

        TranslatingTracer {
            source,
            shift,
            bounds,
            world_view,
        }
    }

    pub fn prune(&self, rendering_parameters: RenderingParameters) -> Option<Tracer> {
        todo!("Implement TranslatingTracer.prune()");
    }

    pub fn trace(&self, x: f64, y: f64, ray: Vector) -> TraceResult {
        todo!("Implement TranslatingTracer.trace()");
    }
}
