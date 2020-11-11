use crate::render::Bounds;
use crate::render::Tracer;

#[derive(Clone, Debug)]
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
}
