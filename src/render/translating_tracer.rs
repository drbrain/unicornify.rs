use crate::geometry::Point;
use crate::geometry::Vector;
use crate::render::Bounds;
use crate::render::RenderingParameters;
use crate::render::TraceResult;
use crate::render::Tracer;
use crate::render::WorldView;

#[derive(Clone, Debug, PartialEq)]
pub struct TranslatingTracer {
    source: Box<Tracer>,
    pub shift: Point,
    pub bounds: Bounds,
    world_view: WorldView,
}

impl TranslatingTracer {
    pub fn new(world_view: &WorldView, source: Tracer, shift: Point) -> Self {
        let mut bounds = source.bounds();

        if !bounds.empty {
            bounds.x_min += shift.x;
            bounds.x_max += shift.x;
            bounds.y_min += shift.y;
            bounds.y_max += shift.y;
        }

        let source = Box::new(source);
        let world_view = world_view.clone();

        TranslatingTracer {
            source,
            shift,
            bounds,
            world_view,
        }
    }

    pub fn prune(&self, rendering_parameters: &RenderingParameters) -> Option<Tracer> {
        let shifted = rendering_parameters.translated(self.shift.x, self.shift.y);

        match self.source.prune(&shifted) {
            None => None,
            Some(pruned) => {
                if *self.source == pruned {
                    Some(Tracer::TranslatingT(self.clone()))
                } else {
                    let source = pruned;
                    let shift = self.shift.clone();

                    let tracer = TranslatingTracer::new(&self.world_view, source, shift);

                    Some(Tracer::TranslatingT(tracer))
                }
            }
        }
    }

    pub fn trace(&self, x: f64, y: f64, _ray: Vector) -> TraceResult {
        let x = x - self.shift.x;
        let y = y - self.shift.y;

        let ray = self.world_view.ray(x, y);

        self.source.trace(x, y, ray)
    }
}
