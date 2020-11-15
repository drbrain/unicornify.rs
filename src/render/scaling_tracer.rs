use crate::geometry::Vector;
use crate::render::Bounds;
use crate::render::RenderingParameters;
use crate::render::TraceResult;
use crate::render::Tracer;
use crate::render::WorldView;

#[derive(Clone, Debug, PartialEq)]
pub struct ScalingTracer {
    source: Box<Tracer>,
    scale: f64,
    pub bounds: Bounds,
    world_view: WorldView,
}

impl ScalingTracer {
    pub fn new(world_view: &WorldView, source: Tracer, scale: f64) -> Self {
        let mut bounds = source.bounds();

        if !bounds.empty {
            bounds.x_min *= scale;
            bounds.x_max *= scale;
            bounds.y_min *= scale;
            bounds.y_max *= scale;
        }

        let source = Box::new(source);

        let world_view = world_view.clone();

        ScalingTracer {
            source,
            scale,
            bounds,
            world_view,
        }
    }

    pub fn prune(&self, rendering_parameters: &RenderingParameters) -> Option<Tracer> {
        let scaled = rendering_parameters.scale(self.scale);

        match self.source.prune(&scaled) {
            None => None,
            Some(pruned) => {
                if *self.source == pruned {
                    Some(Tracer::ScalingT(self.clone()))
                } else {
                    let tracer = ScalingTracer::new(&self.world_view, pruned, self.scale);

                    Some(Tracer::ScalingT(tracer))
                }
            }
        }
    }

    pub fn trace(&self, x: f64, y: f64, _ray: Vector) -> TraceResult {
        let x = x / self.scale;
        let y = y / self.scale;
        let ray = self.world_view.ray(x, y);

        let (z, dir, color) = self.source.trace(x, y, ray)?;

        let z = z * self.scale;

        Some((z, dir, color))
    }
}
