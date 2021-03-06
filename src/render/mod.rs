mod ball_projection;
mod bone_tracer;
mod bounds;
mod facet_tracer;
mod group_tracer;
mod quadrant_tracer;
mod rendering_parameters;
mod scaling_tracer;
mod sphere_projection;
mod tracer;
mod translating_tracer;
mod world_view;

pub use ball_projection::BallProjection;
pub use bone_tracer::BoneTracer;
pub use bounds::Bounds;
pub use facet_tracer::FacetTracer;
pub use group_tracer::GroupTracer;
pub use quadrant_tracer::QuadrantTracer;
pub use rendering_parameters::RenderingParameters;
pub use scaling_tracer::ScalingTracer;
pub use sphere_projection::SphereProjection;
pub use tracer::Tracer;
pub use translating_tracer::TranslatingTracer;
pub use world_view::WorldView;

use crate::geometry::Vector;
use crate::Color;

pub type TraceResult = Option<(f64, Vector, Color)>;

pub fn prune_bounds(tracer: Tracer, rendering_parameters: &RenderingParameters) -> Option<Tracer> {
    if rendering_parameters.contains(&tracer.bounds()) {
        Some(tracer)
    } else {
        None
    }
}
