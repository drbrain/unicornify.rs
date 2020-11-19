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

        let offset = size as f64;

        let (x_offset, y_offset) = match quadrant {
            1 => (0.0, 0.0),
            2 => (offset, 0.0),
            3 => (0.0, offset),
            4 => (offset, offset),
            _ => panic!("Invalid quadrant {}", quadrant),
        };

        let shift = Point::new(-x_offset, -y_offset);
        let translater = TranslatingTracer::new(&world_view, source, shift);
        let source = Box::new(Tracer::TranslatingT(translater));

        let world_view = world_view.clone();

        QuadrantTracer {
            source,
            bounds,
            x_range: (0.0..offset),
            y_range: (0.0..offset),
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
        self.source.trace(x, y, ray)
    }
}
