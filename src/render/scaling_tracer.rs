use crate::render::Tracer;
use crate::render::WorldView;
use crate::render::Bounds;

#[derive(Clone, Debug)]
pub struct ScalingTracer {
    source: Box<Tracer>,
    scale: f64,
    pub bounds: Bounds,
    world_view: WorldView,
}

impl ScalingTracer {
    pub fn new(world_view: WorldView, source: Tracer, scale: f64) -> Self {
        let mut bounds = source.bounds();

        if !bounds.empty {
            bounds.x_min *= scale;
            bounds.x_max *= scale;
            bounds.y_min *= scale;
            bounds.y_max *= scale;
        }

        let source = Box::new(source);

        ScalingTracer { source, scale, bounds, world_view }
    }
}
