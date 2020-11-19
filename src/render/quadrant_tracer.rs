use std::ops::Range;

use crate::geometry::Point;
use crate::geometry::Vector;
use crate::render::Bounds;
use crate::render::RenderingParameters;
use crate::render::TraceResult;
use crate::render::Tracer;
use crate::render::TranslatingTracer;
use crate::render::WorldView;

#[derive(Clone, Debug, PartialEq)]
pub struct QuadrantTracer {
    source: Box<Tracer>,
    pub bounds: Bounds,
    x_range: Range<f64>,
    y_range: Range<f64>,
    world_view: WorldView,
}

impl QuadrantTracer {
    pub fn new(world_view: &WorldView, source: Tracer, size: u32, quadrant: u8) -> Self {
        let bounds = source.bounds();

        let quadrant_size = (size / 2) as f64;
        let size = size as f64;

        let (x_range, y_range) = match quadrant {
            1 => (0.0..quadrant_size, 0.0..quadrant_size),
            2 => (quadrant_size..size, 0.0..quadrant_size),
            3 => (0.0..quadrant_size, quadrant_size..size),
            4 => (quadrant_size..size, quadrant_size..size),
            _ => panic!("Invalid quadrant {}", quadrant),
        };

        let shift = Point::new(-x_range.start, -y_range.start);
        let translater = TranslatingTracer::new(&world_view, source, shift);
        let source = Box::new(Tracer::TranslatingT(translater));

        let world_view = world_view.clone();

        QuadrantTracer {
            source,
            bounds,
            x_range: (0.0..quadrant_size),
            y_range: (0.0..quadrant_size),
            world_view,
        }
    }

    pub fn prune(&self, rendering_parameters: &RenderingParameters) -> Option<Tracer> {
        match self.source.prune(rendering_parameters) {
            None => None,
            Some(pruned) => {
                let bounds = pruned.bounds();

                let tracer = QuadrantTracer {
                    source: Box::new(pruned),
                    bounds: bounds,
                    x_range: self.x_range.clone(),
                    y_range: self.y_range.clone(),
                    world_view: self.world_view.clone(),
                };

                Some(Tracer::QuadrantT(tracer))
            }
        }
    }

    pub fn trace(&self, x: f64, y: f64, ray: Vector) -> TraceResult {
        if !self.x_range.contains(&x) {
            None
        } else if !self.y_range.contains(&y) {
            None
        } else {
            self.source.trace(x, y, ray)
        }
    }
}
