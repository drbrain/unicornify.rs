use crate::geometry::Vector;
use crate::render::Bounds;
use crate::render::RenderingParameters;
use crate::render::TraceResult;
use crate::render::Tracer;

#[derive(Clone, Debug, PartialEq)]
pub struct GroupTracer {
    tracers: Vec<Tracer>,
    pub bounds: Bounds,
    current: bool,
    sorted: bool,
}

impl GroupTracer {
    pub fn new() -> Self {
        let tracers: Vec<Tracer> = Vec::new();
        let bounds = Bounds::empty();
        let current = false;
        let sorted = false;

        GroupTracer {
            tracers,
            bounds,
            current,
            sorted,
        }
    }

    pub fn push(&mut self, tracer: Tracer) {
        self.tracers.push(tracer);
    }

    pub fn prune(&self, rendering_parameters: RenderingParameters) -> Option<Tracer> {
        todo!("Implement GroupTracer.prune()");
    }

    pub fn trace(&self, x: f64, y: f64, ray: Vector) -> TraceResult {
        todo!("Implement BoneTracer.trace()");
    }
}
